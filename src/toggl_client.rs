use crate::toggl_types::{Data, TimeEntry, TimeEntryCreateParam, TimeEntryCreateParamWrapped};
use crate::util::AnyError;
use serde::de;
use serde::Serialize;

#[cfg(test)]
use std::env;

pub const TOGGL_ENDPOINT: &str = "https://api.track.toggl.com/api/v8";

#[derive(Clone)]
pub struct TogglClient<'a> {
    pub endpoint: &'a str,
    pub api_token: &'a str,
}

impl TogglClient<'_> {
    async fn get<T: de::DeserializeOwned>(self, path: &str) -> Result<T, AnyError> {
        let resp: Data<T> = reqwest::Client::new()
            .get(format!("{}{}", self.endpoint, path))
            .header("Content-Type", "application/json")
            .basic_auth(self.api_token, Some("api_token"))
            .send()
            .await?
            .json()
            .await?;
        Ok(resp.data)
    }

    async fn post<T: de::DeserializeOwned, D: Serialize>(
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

    pub async fn get_current_time_entry(self) -> Result<Option<TimeEntry>, AnyError> {
        Ok(self.get("/time_entries/current").await?)
    }

    pub async fn create_time_entry(
        self,
        param: TimeEntryCreateParam,
    ) -> Result<Option<TimeEntry>, Box<dyn std::error::Error>> {
        let wrapped = TimeEntryCreateParamWrapped { time_entry: param };
        Ok(self.post("/time_entries/start", wrapped).await?)
    }

    pub async fn stop_time_entry(
        self,
        time_entry_id: u32,
    ) -> Result<Option<TimeEntry>, Box<dyn std::error::Error>> {
        let path = format!("/time_entries/{:?}/stop", time_entry_id);
        Ok(self.put(&path).await?)
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
