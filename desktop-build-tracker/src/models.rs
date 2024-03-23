use std::str::FromStr;

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct BuildDatabase {
    pub id: i32,
    pub channel: String,
    pub build_number: String,
    pub build_hash: String,
    pub build_id: String,
    pub date: Option<String>,
}

#[derive(Debug)]
pub enum ReleaseChannel {
    Stable,
    PTB,
    Canary,
}

#[derive(Debug)]
pub struct Build {
    pub id: i32,
    pub channel: ReleaseChannel,
    pub build_number: usize,
    pub build_hash: String,
    pub build_id: String,
    pub date: Option<DateTime<Utc>>,
    pub amount_of_builds_today: usize,
    pub is_revert: bool,
}

impl Build {
    pub fn convert_from_database(
        build: BuildDatabase,
        amount_of_builds_today: usize,
        is_revert: bool,
    ) -> Self {
        let datetime = DateTime::<Utc>::from_str(build.date.unwrap().as_str()).unwrap();

        let channel = match build.channel.as_str() {
            "Stable" => ReleaseChannel::Stable,
            "PTB" => ReleaseChannel::PTB,
            "Canary" => ReleaseChannel::Canary,
            _ => panic!("Invalid release channel"),
        };

        Self {
            id: build.id,
            build_hash: build.build_hash,
            build_id: build.build_id[..7].to_string(),
            build_number: build.build_number.parse().unwrap(),
            channel,
            date: Some(datetime),
            amount_of_builds_today,
            is_revert,
        }
    }
}
