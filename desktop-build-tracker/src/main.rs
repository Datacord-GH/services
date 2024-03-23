mod database;
mod models;
mod utils;

use database::Database;
use dotenv::dotenv;
use regex::Regex;
use rusqlite::Result;
use std::collections::HashMap;

use crate::models::Build;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db = Database::init().expect("failed to create database");

    let js_files = Regex::new(r"/assets/([\.a-zA-z0-9]+).js").unwrap();
    let build_number_rg =
        Regex::new(r#"Build Number: "\).concat\("(?P<version>[0-9]+)"+"#).unwrap();
    let build_hash_rg =
        Regex::new(r#"Version Hash: "\).concat\("(?P<hash>[A-Za-z0-9]+)"+"#).unwrap();

    let channels: HashMap<&str, &str> = HashMap::from([
        ("Stable", "https://discord.com"),
        ("PTB", "https://ptb.discord.com"),
        ("Canary", "https://canary.discord.com"),
    ]);

    for (release_channel, url) in channels {
        let app_url = format!("{}/app", url);
        let resp = reqwest::get(app_url).await?;
        let headers = resp.headers().clone();
        let text = resp.text().await?;

        let build_id = match headers.get("x-build-id") {
            Some(header) => header.to_str().unwrap(),
            None => panic!("x-build-id is missing from headers"),
        };

        let build_count = db
            .get_build_count(build_id, release_channel)
            .expect("failed to get builds");

        if build_count > 0 {
            println!("[!] {} is still on build {}", release_channel, build_id);
            continue;
        }

        let mut build_number = String::new();
        let mut build_hash = String::new();
        let files = js_files
            .find_iter(&text)
            .map(|mat: regex::Match<'_>| mat.as_str());

        for js in files {
            let js_file_url: String = format!("{}{}", url, js);
            let js_file_data = reqwest::get(&js_file_url).await?.text().await?;

            build_number = match &build_number_rg.captures(&js_file_data) {
                Some(version) => version["version"].to_string(),
                None => continue,
            };
            build_hash = match &build_hash_rg.captures(&js_file_data) {
                Some(hash) => hash["hash"].to_string(),
                None => continue,
            };

            if !String::is_empty(&build_number) && !String::is_empty(&build_hash) {
                break;
            }
        }

        if String::is_empty(&build_number) || String::is_empty(&build_hash) {
            println!(
                "[!] No build number or build hash was found during search, something is wrong!"
            );
            continue;
        }

        let current = models::BuildDatabase {
            id: 0,
            channel: release_channel.to_string(),
            build_number: build_number.to_string(),
            build_hash: build_hash.to_string(),
            build_id: build_id.to_string(),
            date: Some(chrono::offset::Utc::now().to_string()),
        };

        let builds_today = db.get_amount_of_builds_today(&release_channel)?;
        let is_revert_build = db.is_build_revert(&current.build_number, &release_channel)?;

        db.insert_build(&current);

        let build = Build::convert_from_database(current, builds_today, is_revert_build);

        utils::send_message(&build).await?;

        println!("---------{}---------", release_channel);
        println!("Build Number: {}", build.build_number);
        println!("Build Id: {}", build.build_id);
        println!("Build Hash: {}", build.build_hash);
    }

    Ok(())
}
