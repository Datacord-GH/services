mod models;
mod utils;

use dotenv::dotenv;
use regex::Regex;
use rusqlite::{Connection, Result};
use std::{collections::HashMap, env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let conn = Connection::open(env::var("DB_URL").expect("missing DB_URL in .env"))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS builds (id INTEGER PRIMARY KEY, channel TEXT, build_number TEXT, build_hash TEXT, build_id TEXT)",
        (),
    )?;

    let js_files = Regex::new(r"/assets/([a-zA-z0-9]+).js").unwrap();
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

        let sql_select = format!(
            "SELECT * FROM builds WHERE build_id = '{}' AND channel = '{}'",
            build_id, release_channel
        );
        let mut stmt = conn.prepare(&sql_select)?;
        let build = stmt.query_map([], |row| {
            Ok(models::Build {
                id: row.get(0)?,
                channel: row.get(1)?,
                build_number: row.get(2)?,
                build_hash: row.get(3)?,
                build_id: row.get(4)?,
            })
        })?;

        if build.count() > 0 {
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

        let current = models::Build {
            id: 0,
            channel: release_channel.to_string(),
            build_number: build_number.to_string(),
            build_hash: build_hash.to_string(),
            build_id: build_id.to_string(),
        };
        conn.execute(
            "INSERT INTO builds (channel, build_number, build_hash, build_id) VALUES (?1, ?2, ?3, ?4)",
            (
                &current.channel,
                &current.build_number,
                &current.build_hash,
                &current.build_id,
            ),
        )?;

        utils::send_message(&current).await?;

        println!("---------{}---------", release_channel);
        println!("Build Number: {}", build_number);
        println!("Build Id: {}", &build_id[..7]);
        println!("Build Hash: {}", build_hash);
    }

    Ok(())
}
