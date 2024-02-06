use crate::models::DiscoveryDatabase;
use sqlx::pool::PoolConnection;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::env;
use std::error::Error;

pub struct Database {
    pub conn: PoolConnection<Sqlite>,
}

impl Database {
    pub async fn init() -> Result<Database, Box<dyn Error>> {
        let db_url = &env::var("DB_URL").expect("failed to find DB_URL in env");

        if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
            println!("[+] Creating database {}", db_url);
            match Sqlite::create_database(db_url).await {
                Ok(_) => println!("[+] Create db success"),
                Err(error) => panic!("error: {}", error),
            }
        } else {
            println!("[!] Database already exists");
        }

        let pool = SqlitePool::connect(&db_url).await?;

        Ok(Database {
            conn: pool.acquire().await?,
        })
    }

    pub async fn setup_tables(&mut self) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS guilds
                (guild_id TEXT PRIMARY KEY, guild_name TEXT)"#,
        )
        .execute(&mut self.conn)
        .await?;

        Ok(())
    }

    pub async fn is_guild_in_db(&mut self, id: &String) -> bool {
        match sqlx::query_as::<_, DiscoveryDatabase>("SELECT * FROM guilds WHERE guild_id = ?")
            .bind(&id)
            .fetch_one(&mut self.conn)
            .await
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub async fn add_guild_to_db(
        &mut self,
        guild_id: &String,
        guild_name: &String,
    ) -> Result<(), Box<dyn Error>> {
        sqlx::query("INSERT INTO guilds (guild_id, guild_name) VALUES (?, ?)")
            .bind(guild_id)
            .bind(guild_name)
            .execute(&mut self.conn)
            .await?;

        Ok(())
    }
}
