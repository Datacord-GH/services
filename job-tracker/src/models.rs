use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum TypeOfUpdate {
    AddedBackFromRemove,
    Removed,
}

pub struct JobDatabase {
    pub id: usize,
    pub title: String,
    pub removed: bool,
}

impl JobDatabase {
    pub fn get_discord_url(&self) -> String {
        format!("https://discord.com/jobs/{}", self.id)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GreenHouseResponse {
    pub jobs: Vec<Job>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub education: Option<String>,
    #[serde(rename = "internal_job_id")]
    pub internal_job_id: usize,
    pub location: Location,
    pub id: usize,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "requisition_id")]
    pub requisition_id: String,
    pub title: String,
    pub departments: Vec<Department>,
    pub offices: Vec<Office>,
}

impl Job {
    pub fn get_discord_url(&self) -> String {
        format!("https://discord.com/jobs/{}", self.id)
    }

    pub fn get_updated_at(&self) -> String {
        let datetime =
            DateTime::parse_from_rfc3339(&self.updated_at).expect("Failed to parse updated_at");

        let time = datetime.timestamp();

        format!("<t:{}:F> (<t:{}:R>)", &time, &time)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Department {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Office {
    pub name: String,
    pub location: Option<String>,
}
