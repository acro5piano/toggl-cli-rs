use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data<T> {
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: u32,
    pub wid: u32,
    pub pid: u32,
    pub billable: bool,
    pub start: String,
    pub duration: i64,
    pub description: String,
    pub duronly: bool,
    pub at: String,
    pub uid: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntryCreateParamWrapped {
    pub time_entry: TimeEntryCreateParam,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntryCreateParam {
    pub pid: u32,
    pub description: String,
    pub created_with: String,
}
