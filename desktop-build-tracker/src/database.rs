use chrono::Utc;
use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};
use std::{env, error::Error};

use crate::models::{self, BuildDatabase, ReleaseChannel};

#[derive(Debug)]
pub struct Database {
    pub connection: Connection,
}

impl Database {
    pub fn init() -> Result<Self, Box<dyn Error>> {
        let migrations = Migrations::new(vec![
            M::up("CREATE TABLE IF NOT EXISTS builds (id INTEGER PRIMARY KEY, channel TEXT, build_number TEXT, build_hash TEXT, build_id TEXT)"),
            M::up("ALTER TABLE builds ADD date TEXT"),
        ]);

        let mut conn = Connection::open(env::var("DB_URL").expect("missing DB_URL in .env"))?;

        migrations.to_latest(&mut conn).unwrap();

        Ok(Self { connection: conn })
    }

    pub fn insert_build(&self, build: &BuildDatabase) {
        self.connection.execute(
            "INSERT INTO builds (channel, build_number, build_hash, build_id, date) VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                &build.channel,
                &build.build_number,
                &build.build_hash,
                &build.build_id,
                &build.date
            ),
        ).expect("failed to insert into builds");
    }

    pub fn get_build_count(
        &self,
        build_id: &str,
        release_channel: &str,
    ) -> Result<usize, Box<dyn Error>> {
        let sql_select = format!(
            "SELECT * FROM builds WHERE build_id = '{}' AND channel = '{}'",
            build_id, release_channel
        );

        let mut stmt = self.connection.prepare(&sql_select)?;
        let db_build = stmt.query_map([], |row| {
            Ok(models::BuildDatabase {
                id: row.get(0)?,
                channel: row.get(1)?,
                build_number: row.get(2)?,
                build_hash: row.get(3)?,
                build_id: row.get(4)?,
                date: row.get(5)?,
            })
        })?;

        Ok(db_build.count() + 1)
    }

    pub fn get_amount_of_builds_today(
        &self,
        release_channel: &str,
    ) -> Result<usize, Box<dyn Error>> {
        let current_utc = Utc::now();
        let date = format!("{}", current_utc.format("%Y-%m-%d"));

        let sql_select = format!(
            "SELECT * FROM builds WHERE channel = '{}' AND date LIKE '{}%'",
            release_channel, date
        );

        let mut stmt = self.connection.prepare(&sql_select)?;
        let builds = stmt.query_map([], |row| {
            Ok(models::BuildDatabase {
                id: row.get(0)?,
                channel: row.get(1)?,
                build_number: row.get(2)?,
                build_hash: row.get(3)?,
                build_id: row.get(4)?,
                date: row.get(5)?,
            })
        })?;

        Ok(builds.count())
    }

    pub fn is_build_revert(
        &self,
        build_number: &str,
        release_channel: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let sql_select = format!(
            "SELECT * FROM builds WHERE channel = '{}' ORDER BY date desc LIMIT 1",
            release_channel
        );
        let mut stmt = self.connection.prepare(&sql_select)?;
        match stmt.query_row([], |row| {
            Ok(models::BuildDatabase {
                id: row.get(0)?,
                channel: row.get(1)?,
                build_number: row.get(2)?,
                build_hash: row.get(3)?,
                build_id: row.get(4)?,
                date: row.get(5)?,
            })
        }) {
            Ok(build) => {
                if build_number.parse::<usize>()? < build.build_number.parse::<usize>()? {
                    return Ok(true);
                } else {
                    Ok(false)
                }
            }
            Err(_) => Ok(false),
        }
    }
}
