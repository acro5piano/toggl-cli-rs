use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data<T> {
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: u32,
    pub wid: u32,
    pub pid: Option<u32>,
    pub billable: bool,
    pub start: String,
    pub stop: Option<String>,
    pub duration: i64,
    pub description: String,
    pub duronly: bool,
    pub at: String,
    pub uid: u32,
}

impl TimeEntry {
    pub fn display_stop(&self) -> &str {
        match &self.stop {
            Some(stop) => stop,
            _ => "                         ",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntryCreateParamWrapped {
    pub time_entry: TimeEntryCreateParam,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntryCreateParam {
    pub pid: Option<u32>,
    pub description: String,
    pub created_with: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: u32,
    pub wid: u32,
    pub cid: Option<u32>,
    pub name: String,
    pub billable: bool,
    pub is_private: bool,
    pub active: bool,
    pub template: bool,
    pub at: String,
    pub created_at: String,
    pub color: String,
    pub auto_estimates: bool,
    pub actual_hours: Option<i64>,
    pub hex_color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub id: u32,
    pub name: String,
    pub profile: u32,
    pub premium: bool,
    pub admin: bool,
    pub default_hourly_rate: i64,
    pub default_currency: String,
    pub only_admins_may_create_projects: bool,
    pub only_admins_see_billable_rates: bool,
    pub only_admins_see_team_dashboard: bool,
    pub projects_billable_by_default: bool,
    pub rounding: i64,
    pub rounding_minutes: i64,
    pub api_token: String,
    pub at: String,
    pub ical_enabled: bool,
}
