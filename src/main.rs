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
    Start {
        #[structopt(long)]
        pid: u32,
        #[structopt(long)]
        description: String,
    },
    Stop {},
    View {},
    Init {
        #[structopt(long)]
        token: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        Program::Start { pid, description } => {
            let time_entry = TimeEntryCreateParam {
                pid: pid,
                description: description,
                created_with: "toggl-cli-rs".to_string(),
            };
            let resp = client.create_time_entry(time_entry).await?;
            println!("{:#?}", resp);
        }
        Program::Stop {} => {
            let current = client.clone().get_current_time_entry().await?;
            match current {
                Some(entry) => {
                    client.stop_time_entry(entry.id).await?;
                }
                _ => println!("Currently no entry exists"),
            }
        }
        Program::View {} => {
            let current = client.get_current_time_entry().await?;
            match current {
                Some(entry) => println!("{:#?}", entry),
                _ => println!("Currently no entry exists"),
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
