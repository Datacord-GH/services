mod discord;
mod models;

use discord::{send_new_user, send_updated_rep};
use dotenv::dotenv;
use models::HackerOneThanks;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::env;
use tokio::main;

use crate::models::HackerOneThanksDB;

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db_url = &env::var("DATABASE_URL")?;

    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("Creating database {}", db_url);
        match Sqlite::create_database(db_url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let pool = SqlitePool::connect(&db_url).await?;
    let mut conn = pool.acquire().await?;
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS hackers 
            (user_id TEXT PRIMARY KEY, username TEXT, reputation INTEGER, profile_url TEXT)"#,
    )
    .execute(&mut conn)
    .await?;

    let hackers = reqwest::get("https://h1rep.jup.workers.dev/")
        .await?
        .json::<Vec<HackerOneThanks>>()
        .await?;

    println!("Found '{}' hacker(s)", hackers.len());

    for hacker in hackers {
        let hacker_db =
            sqlx::query_as::<_, HackerOneThanksDB>("SELECT * FROM hackers WHERE user_id = ?")
                .bind(&hacker.id)
                .fetch_one(&pool)
                .await;

        match hacker_db {
            Ok(old) => {
                if old.reputation != hacker.reputation {
                    send_updated_rep(
                        &hacker,
                        &HackerOneThanks {
                            avatar_url: "".to_string(),
                            id: old.user_id,
                            username: old.username,
                            reputation: old.reputation,
                            position: 0,
                        },
                    )
                    .await?;

                    sqlx::query("UPDATE hackers SET reputation = ? WHERE user_id = ?")
                        .bind(&hacker.reputation)
                        .bind(&hacker.id)
                        .execute(&mut conn)
                        .await?;
                }
            }
            Err(_) => {
                send_new_user(&hacker).await?;

                sqlx::query("INSERT INTO hackers (user_id, username, reputation, profile_url) VALUES (?, ?, ?, ?)")
                    .bind(&hacker.id)
                    .bind(&hacker.username)
                    .bind(&hacker.reputation)
                    .bind(&hacker.get_hackerone_url())
                    .execute(&mut conn)
                    .await?;
            }
        }
    }

    Ok(())
}
