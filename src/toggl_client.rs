// See https://github.com/toggl/toggl_api_docs/blob/master/toggl_api.md

use crate::toggl_types::{Data, Project, TimeEntry, TimeEntryCreateParam, Workspace};
use crate::util::AnyError;
use serde::de;
use serde::Serialize;
use serde_json::json;

pub const TOGGL_ENDPOINT: &str = "https://api.track.toggl.com/api/v9";

#[derive(Clone)]
pub struct TogglClient<'a> {
    pub endpoint: &'a str,
    pub api_token: &'a str,
}

impl TogglClient<'_> {
    async fn get<T: de::DeserializeOwned>(&self, path: &str) -> Result<T, AnyError> {
        let resp: T = reqwest::Client::new()
            .get(format!("{}{}", self.endpoint, path))
            .header("Content-Type", "application/json")
            .basic_auth(self.api_token, Some("api_token"))
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    async fn post<T: de::DeserializeOwned + std::fmt::Debug, D: Serialize>(
        &self,
        path: &str,
        data: D,
    ) -> Result<T, AnyError> {
        let resp: Data<T> = reqwest::Client::new()
            .post(format!("{}{}", self.endpoint, path))
            .header("Content-Type", "application/json")
            .basic_auth(self.api_token, Some("api_token"))
            .json(&data)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp.data)
    }

    async fn put<T: de::DeserializeOwned, D: Serialize>(
        &self,
        path: &str,
        data: D,
    ) -> Result<T, AnyError> {
        let resp: Data<T> = reqwest::Client::new()
            .put(format!("{}{}", self.endpoint, path))
            .header("Content-Type", "application/json")
            // .header("Content-Length", "0")
            .basic_auth(self.api_token, Some("api_token"))
            .json(&data)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp.data)
    }

    pub async fn get_current_time_entry(&self) -> Result<Option<TimeEntry>, AnyError> {
        Ok(self
            .get::<Option<TimeEntry>>("/me/time_entries/current")
            .await?)
    }

    pub async fn get_time_entries(&self) -> Result<Vec<TimeEntry>, AnyError> {
        let time_entires: Vec<TimeEntry> = self.get("/me/time_entries").await?;
        Ok(time_entires)
    }

    pub async fn get_all_projects_of_user(&self) -> Result<Vec<Project>, AnyError> {
        let workspaces = self.get_workspaces().await?;
        let mut projects: Vec<Project> = vec![];
        for w in workspaces {
            for p in self.get_projects(w.id).await? {
                projects.push(p);
            }
        }
        Ok(projects)
    }

    pub async fn get_projects(&self, workspace_id: u32) -> Result<Vec<Project>, AnyError> {
        let path = format!("/workspaces/{}/projects", workspace_id);
        Ok(self.get::<Vec<Project>>(&path).await?)
    }

    async fn get_workspaces(&self) -> Result<Vec<Workspace>, AnyError> {
        Ok(self.get("/workspaces").await?)
    }

    pub async fn create_time_entry(
        &self,
        param: TimeEntryCreateParam,
    ) -> Result<Option<TimeEntry>, Box<dyn std::error::Error>> {
        let workspace_id = self.get_workspaces().await?.first().unwrap().id;
        let now = chrono::Utc::now().to_rfc3339();
        let wrapped = json!({
            "project_id": param.pid,
            "description": param.description,
            "start": now,
            "duration": -1,
            "created_with": param.created_with,
            "workspace_id":workspace_id
        });
        let path = format!("/workspaces/{}/time_entries", workspace_id);
        Ok(self.post(&path, wrapped).await?)
    }

    pub async fn stop_time_entry(
        &self,
        time_entry_id: u32,
    ) -> Result<Option<TimeEntry>, Box<dyn std::error::Error>> {
        let workspace_id = self.get_workspaces().await?.first().unwrap().id;
        let path = format!(
            "/workspaces/{}/time_entries/{}",
            workspace_id, time_entry_id
        );
        let now = chrono::Utc::now().to_rfc3339();
        Ok(self.put(&path, json!({ "stop": now })).await?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_api_entry() -> Result<(), Box<dyn std::error::Error>> {
        let client = TogglClient {
            endpoint: "https://api.track.toggl.com/api/v8/time_entries/current",
            api_token: &env::var("API_TOKEN")?,
        };

        let res = client.get_current_time_entry().await?;

        println!("{:?}", res);

        Ok(())
    }
}
