mod discord;
mod models;
mod utils;

use std::env;

use crate::discord::send_message;
use crate::utils::{change_text, check_for_change};
use dotenv::dotenv;
use models::{ProductDatabase, ProductRespose};
use reqwest::header::USER_AGENT;
use rusqlite::{Connection, Result};
use tokio::main;

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let conn = Connection::open(env::var("DB_URL").expect("missing DB_URL in .env"))?;
    conn.execute(
            "CREATE TABLE IF NOT EXISTS products (id INTEGER PRIMARY KEY, title TEXT, variant_title TEXT, available BOOL, price TEXT)",
        (),
    )?;

    let url = "https://discord.store/products.json?limit=250";
    let client = reqwest::Client::new();

    let body = client
        .get(url)
        .header(USER_AGENT, "https://github.com/Datacord-GH/store-tracker")
        .send()
        .await
        .unwrap();

    let products = body.json::<ProductRespose>().await.unwrap();

    println!("Found {} products", products.products.len());

    for product in &products.products {
        for product_variant in &product.variants {
            let sql_select = format!("SELECT * FROM products WHERE id = '{}'", product_variant.id);
            let mut stmt = conn.prepare(&sql_select)?;
            let mut product_db = stmt.query_map([], |row| {
                Ok(ProductDatabase {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    variant_title: row.get(2)?,
                    available: row.get(3)?,
                    price: row.get(4)?,
                    from_db: true,
                })
            })?;

            let product_current = ProductDatabase {
                id: product_variant.id,
                title: product.title.clone(),
                variant_title: product_variant.title.clone(),
                available: product_variant.available,
                price: product_variant.price.clone(),
                from_db: false,
            };

            let product_old = match product_db.nth(0) {
                Some(value) => value.unwrap(),
                None => product_current.clone(),
            };

            let changes = check_for_change(&product_old, &product_current);

            if changes.len() > 0 {
                let mut body: Vec<String> = vec![];

                for change in changes {
                    body.push(change_text(change, &product_old, &product_current));
                }

                println!(
                    "Product {} ({}) was changed",
                    product_current.title, product_current.id
                );

                conn.execute(
                    "UPDATE products SET title = ?1, variant_title = ?2, available = ?3, price = ?4 WHERE id = ?5",
            (
                    &product_current.title,
                    &product_current.variant_title,
                    &product_current.available,
                    &product_current.price,
                    &product_current.id,
            ),
        )?;

                match send_message(body, &product, product_variant).await {
                    Ok(_) => continue,
                    Err(err) => eprintln!("Updating products {}", err),
                }
            } else if product_current == product_old && !product_old.from_db {
                println!(
                    "New product {} ({})",
                    product_current.title, product_current.id
                );

                conn.execute(
                            "INSERT INTO products (id, title, variant_title, available, price) VALUES (?1, ?2, ?3, ?4, ?5)",
                    (
                            &product_current.id,
                            &product_current.title,
                            &product_current.variant_title,
                            &product_current.available,
                            &product_current.price,
                    ),
                )?;

                match send_message(vec![], &product, product_variant).await {
                    Ok(_) => continue,
                    Err(err) => eprintln!("Error on insert new product {}", err),
                }
            }
        }
    }

    Ok(())
}
