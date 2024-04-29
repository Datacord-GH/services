mod discord;
mod models;

use dotenv::dotenv;
use models::endpoints::Endpoint;
use rusqlite::{Connection, Result};
use std::time::SystemTime;
use std::{env, fs, time::Duration};
use tokio::time::sleep;

use crate::discord::send_message;
use crate::models::subdomains::SubdomainDB;

const RATELIMIT_SLEEP: Duration = Duration::from_secs(60);
const CHUNK_STILL_RATELIMITED_SLEEP: Duration = Duration::from_secs(70);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let conn = Connection::open(env::var("DB_URL").expect("missing DB_URL in .env"))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS subdomains (id INTEGER PRIMARY KEY, domain TEXT, subdomain TEXT, date TEXT)",
        (),
    )?;

    let endpoints_data =
        fs::read_to_string("./endpoints.json").expect("unable to read 'endpoints.json'");
    let endpoints: Vec<Endpoint> =
        serde_json::from_str(&endpoints_data).expect("unable to parse 'endpoints.json'");

    for chunk_endpoint in endpoints.chunks(5) {
        for endpoint in chunk_endpoint {
            let url = format!(
                "https://api.subdomain.center/?domain={}",
                endpoint.get_clean_url()
            );
            let mut subdomains = reqwest::get(&url).await?.json::<Vec<String>>().await;
            if subdomains.is_err() {
                println!("Ratelimited during chunks, trying again after 70 seconds");
                sleep(CHUNK_STILL_RATELIMITED_SLEEP).await;
                subdomains = reqwest::get(&url).await?.json::<Vec<String>>().await;

                if subdomains.is_err() {
                    println!("Still error breaking this chunk");
                    break;
                }
            }

            for subdomain in subdomains.unwrap() {
                let sql_select = format!(
                    "SELECT * FROM subdomains WHERE domain = '{}' AND subdomain = '{}'",
                    endpoint.get_clean_url(),
                    subdomain
                );
                let mut stmt = conn.prepare(&sql_select)?;
                let subdomain_db = stmt.query_map([], |row| {
                    Ok(SubdomainDB {
                        domain: row.get(0)?,
                        subdomain: row.get(1)?,
                        date: row.get(2)?,
                    })
                })?;

                if subdomain_db.count() > 0 {
                    continue;
                }

                send_message(&endpoint, &subdomain).await?;

                println!(
                    "New subdomain found on '{}' -> '{}'",
                    &endpoint.get_clean_url(),
                    &subdomain
                );

                let duration_since_epoch = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap();

                let _ = conn.execute(
                    "INSERT INTO subdomains (domain, subdomain, date) VALUES (?1, ?2, ?3)",
                    (
                        &endpoint.get_clean_url(),
                        &subdomain,
                        duration_since_epoch.as_secs().to_string(),
                    ),
                );
            }
        }

        println!("Done with chunk sleeping for 60 seconds");
        sleep(RATELIMIT_SLEEP).await;
    }

    Ok(())
}
