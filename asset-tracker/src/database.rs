use crate::models::AssetDatabase;
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
            CREATE TABLE IF NOT EXISTS assets
                (hash TEXT PRIMARY KEY)"#,
        )
        .execute(&mut self.conn)
        .await?;

        Ok(())
    }

    pub async fn is_hash_in_db(&mut self, hash: &String) -> bool {
        match sqlx::query_as::<_, AssetDatabase>("SELECT * FROM assets WHERE hash = ?")
            .bind(&hash)
            .fetch_one(&mut self.conn)
            .await
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub async fn add_hash_to_db(&mut self, hash: &String) -> Result<(), Box<dyn Error>> {
        sqlx::query("INSERT INTO assets (hash) VALUES (?)")
            .bind(hash)
            .execute(&mut self.conn)
            .await?;

        Ok(())
    }
}
