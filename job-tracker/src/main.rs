mod discord;
mod models;
mod utils;

use crate::{
    discord::{send_new_job_message, send_updated_job_message},
    models::{Job, JobDatabase, TypeOfUpdate},
};
use dotenv::dotenv;
use models::GreenHouseResponse;
use rusqlite::{Connection, Result};
use std::{collections::HashMap, env};
use tokio::main;

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let connection = Connection::open(env::var("DB_URL").expect("missing DB_URL in .env"))?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS jobs (id INTEGER PRIMARY KEY, title TEXT, removed BOOLEAN)",
        (),
    )?;

    let url = "https://api.greenhouse.io/v1/boards/discord/jobs?content=true";
    let client = reqwest::Client::new();

    let body = client.get(url).send().await.unwrap();
    let greenhouse = body.json::<GreenHouseResponse>().await.unwrap();

    let sql_select = format!("SELECT * FROM jobs");
    let mut stmt = connection.prepare(&sql_select)?;
    let jobs_db_raw = stmt.query_map([], |row| {
        Ok(JobDatabase {
            id: row.get(0)?,
            title: row.get(1)?,
            removed: row.get(2)?,
        })
    })?;

    let mut jobs_db: HashMap<usize, JobDatabase> = HashMap::new();

    for job_db_raw_value in jobs_db_raw {
        match job_db_raw_value {
            Ok(job) => {
                jobs_db.insert(job.id, job);
            }
            Err(err) => panic!("{:#?}", err),
        }
    }

    let mut jobs_live: HashMap<usize, Job> = HashMap::new();

    for job in &greenhouse.jobs {
        jobs_live.insert(job.id.clone(), job.clone());

        match jobs_db.get(&job.id) {
            Some(job_db) => {
                // In database and on the live site

                // Removed and added back to the website
                if job_db.removed {
                    send_updated_job_message(job_db, TypeOfUpdate::AddedBackFromRemove).await?;

                    connection.execute(
                        "UPDATE jobs SET removed = ?1 WHERE id = ?2",
                        (false, job_db.id),
                    )?;
                }
            }
            None => {
                // Not in the database but on the live site
                // Send new job alert

                send_new_job_message(job).await?;

                connection.execute(
                    "INSERT INTO jobs (id, title, removed) VALUES (?1, ?2, ?3)",
                    (&job.id, &job.title, false),
                )?;
            }
        }
    }

    for (job_id, job_db) in &jobs_db {
        match jobs_live.get(job_id) {
            Some(_) => {
                // In the database and on the live site
            }
            None => {
                // In the database but not on the live site
                // Send removed alert
                if job_db.removed {
                    continue;
                }

                send_updated_job_message(job_db, TypeOfUpdate::Removed).await?;

                connection.execute("UPDATE jobs SET removed = ?1 WHERE id = ?2", (true, job_id))?;
            }
        }
    }

    println!("[!] Processed {} jobs from Greenhouse", &jobs_live.len());
    println!("[!] Processed {} jobs in the database", &jobs_db.len());

    Ok(())
}
