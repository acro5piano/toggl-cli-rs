// See https://github.com/toggl/toggl_api_docs/blob/master/toggl_api.md

use crate::toggl_types::{
    Data, Project, TimeEntry, TimeEntryCreateParam, TimeEntryCreateParamWrapped, Workspace,
};
use crate::util::AnyError;
use serde::de;
use serde::Serialize;

pub const TOGGL_ENDPOINT: &str = "https://api.track.toggl.com/api/v8";

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

    // TODO: enable to add data
    // async fn put<T: de::DeserializeOwned, D: Serialize>(
    async fn put<T: de::DeserializeOwned>(&self, path: &str) -> Result<T, AnyError> {
        let resp: Data<T> = reqwest::Client::new()
            .put(format!("{}{}", self.endpoint, path))
            .header("Content-Type", "application/json")
            .header("Content-Length", "0")
            .basic_auth(self.api_token, Some("api_token"))
            // .json(&data)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp.data)
    }

    pub async fn get_current_time_entry(&self) -> Result<Option<TimeEntry>, AnyError> {
        Ok(self
            .get::<Data<Option<TimeEntry>>>("/time_entries/current")
            .await?
            .data)
    }

    pub async fn get_time_entries(&self) -> Result<Vec<TimeEntry>, AnyError> {
        let time_entires: Vec<TimeEntry> = self.get("/time_entries").await?;
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
        Ok(self.get(&path).await?)
    }

    pub async fn get_workspaces(&self) -> Result<Vec<Workspace>, AnyError> {
        Ok(self.get("/workspaces").await?)
    }

    pub async fn create_time_entry(
        &self,
        param: TimeEntryCreateParam,
    ) -> Result<Option<TimeEntry>, Box<dyn std::error::Error>> {
        let wrapped = TimeEntryCreateParamWrapped { time_entry: param };
        Ok(self.post("/time_entries/start", wrapped).await?)
    }

    pub async fn stop_time_entry(
        &self,
        time_entry_id: u32,
    ) -> Result<Option<TimeEntry>, Box<dyn std::error::Error>> {
        let path = format!("/time_entries/{}/stop", time_entry_id);
        Ok(self.put(&path).await?)
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
