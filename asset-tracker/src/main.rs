mod database;
mod discord;
mod models;
mod utils;

use crate::database::Database;
use crate::models::{Asset, DiscordMessage};
use bytes::Bytes;
use dotenv::dotenv;
use regex::Regex;
use resvg::usvg::TreeParsing;
use std::env;
use std::error::Error;
use tokio::main;

#[main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let urls: Vec<&str> = vec![
        "https://canary.discord.com",
        "https://canary.discord.com/app",
    ];

    let asset_files_rg = Regex::new(r"\/assets\/([\.a-zA-Z0-9]+)\.([a-zA-Z0-9]+)")?;

    let mut found_assets: Vec<Asset> = vec![];

    for url in urls {
        let resp = reqwest::get(url).await?;
        let txt = resp.text().await?;

        for file in asset_files_rg.captures_iter(&txt) {
            let hash = file.get(1).unwrap().as_str().to_string();
            let file_type = file.get(2).unwrap().as_str().to_string();

            let file_asset = Asset {
                path: format!("/assets/{}.{}", hash, file_type),
                hash,
                file_type,
            };

            found_assets.push(file_asset);
        }
    }

    println!("[!] Found {} asset path files", found_assets.len());

    let mut database = Database::init().await?;
    database.setup_tables().await?;

    for asset_files in found_assets {
        let assets = asset_files.get_all_assets().await?;

        for asset in assets {
            if !database.is_hash_in_db(&asset.hash).await {
                database.add_hash_to_db(&asset.hash).await?;

                println!("[+] {}.{} ({})", &asset.hash, &asset.file_type, &asset.path);

                if &env::var("DRY_RUN").expect("failed to find 'DRY_RUN' in env") == "true" {
                    continue;
                }

                let bytes = asset.download().await.unwrap();

                let mut images: Vec<DiscordMessage> = vec![DiscordMessage {
                    data: bytes.clone(),
                    file_name: asset.hash.clone(),
                    file_type: asset.file_type.clone(),
                }];

                if asset.file_type == String::from("svg") {
                    let opt = resvg::usvg::Options::default();
                    let tree = resvg::usvg::Tree::from_data(&bytes, &opt)?;
                    let rtree = resvg::Tree::from_usvg(&tree);
                    let mut zoom = 2.0;

                    let (width, height) = rtree.size.to_int_size().dimensions();

                    if width < 20 || height < 20 {
                        zoom = 10.0;
                    } else if width <= 500 || height <= 500 {
                        zoom = 5.0;
                    }

                    let pixmap_size = rtree.size.to_int_size().scale_by(zoom).unwrap();
                    let mut pixmap =
                        resvg::tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
                            .unwrap();
                    let render_ts = resvg::tiny_skia::Transform::from_scale(zoom, zoom);
                    rtree.render(render_ts, &mut pixmap.as_mut());

                    let data = pixmap.encode_png()?;

                    images.push(DiscordMessage {
                        data: Bytes::from(data),
                        file_name: asset.hash,
                        file_type: String::from("png"),
                    });
                }

                discord::send_message(images).await?;
            }
        }
    }

    println!("[+] Done parsing asset files");

    Ok(())
}
