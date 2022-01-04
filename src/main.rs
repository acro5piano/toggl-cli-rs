use crate::toggl_types::TimeEntryCreateParam;
use std::env;
use std::fs;
use std::path::Path;
use structopt::StructOpt;

mod toggl_client;
mod toggl_types;
mod util;

#[derive(StructOpt)]
#[structopt(about = "Toggl cli for geek")]
enum Program {
    StartTimer {
        #[structopt(long)]
        pid: Option<u32>,
        #[structopt(long)]
        project_name: Option<String>,
        #[structopt(long)]
        description: String,
    },
    StopTimer {},
    ViewTimer {},
    ListProjects {},
    Init {
        #[structopt(long)]
        token: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), util::AnyError> {
    env_logger::init();

    let path = Path::new(&env::var("HOME")?).join(".toggl-token");
    let api_token = match fs::read_to_string(&path) {
        Ok(content) => content,
        _ => "".to_string(),
    };
    let client = toggl_client::TogglClient {
        endpoint: toggl_client::TOGGL_ENDPOINT,
        api_token: &api_token,
    };

    match Program::from_args() {
        Program::StartTimer {
            pid,
            project_name,
            description,
        } => {
            let mut real_pid: Option<u32> = pid;
            if pid.is_none() {
                if let Some(n) = project_name {
                    let projects = client.get_all_projects_of_user().await.unwrap();
                    for project in projects {
                        if project.name == n {
                            real_pid = Some(project.id);
                        }
                    }
                    if real_pid.is_none() {
                        panic!(
                            "Project {} not found! You can list projects by `toggl list-projects`",
                            n
                        );
                    }
                }
            }
            let time_entry = TimeEntryCreateParam {
                pid: real_pid,
                description: description,
                created_with: "toggl-cli-rs".to_string(),
            };
            let resp = client.create_time_entry(time_entry).await?;
            println!("{:#?}", resp);
        }
        Program::StopTimer {} => {
            let current = client.clone().get_current_time_entry().await?;
            match current {
                Some(entry) => {
                    client.stop_time_entry(entry.id).await?;
                    println!("Stopped task: {:?}", entry);
                    println!("Success!");
                }
                _ => println!("Currently no entry exists"),
            }
        }
        Program::ViewTimer {} => {
            let current = client.get_current_time_entry().await?;
            match current {
                Some(entry) => println!("{:?}", entry),
                _ => println!("Currently no entry exists"),
            }
        }
        Program::ListProjects {} => {
            let projects = client.get_all_projects_of_user().await?;
            for project in projects {
                println!("{} | {}", project.id, project.name);
            }
        }
        Program::Init { token } => {
            fs::write(&path, token)?;
            println!("Wrote token to {:?}", path);
            println!("Success!");
        }
    }
    Ok(())
}
