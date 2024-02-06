mod database;
mod models;

use crate::database::Database;
use dotenv::dotenv;
use models::AlgoliaResponse;
use std::{env, error::Error};
use tokio::main;

#[main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let mut database = Database::init().await?;
    database.setup_tables().await?;

    let mut member_count = 20_000_000;
    let url = env::var("ALGOLIA_URL").expect("failed to find ALGOLIA_URL in env");

    loop {
        let resp = reqwest::post(&url).await?;
        let data = resp.json::<AlgoliaResponse>().await?;

        println!("{:#?}", data);
        break;
    }

    Ok(())
}
