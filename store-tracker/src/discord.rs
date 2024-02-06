use serenity::json::Value;
use serenity::prelude::SerenityError;
use serenity::{http::Http, model::channel::Embed, model::webhook::Webhook, utils::Colour};
use std::env;

use crate::models::{Product, ProductVariant};

pub async fn send_message(
    changes: Vec<String>,
    product: &Product,
    variant: &ProductVariant,
) -> Result<(), SerenityError> {
    let http = Http::new("token");
    let token = env::var("WEBHOOK_URL").expect("missing WEBHOOK_URL in .env");
    let webhook = Webhook::from_url(&http, &token).await?;

    let content = format!(
        "<@&{}>",
        env::var("ROLE_ID").expect("missing ROLE_ID in .env")
    );

    let store_embed: Value;

    if changes.len() > 0 {
        store_embed = Embed::fake(|e| {
            e.colour(Colour::from_rgb(255, 169, 249))
                .title(format!(
                    "Change found on {} - {}",
                    &product.title, &variant.title
                ))
                .description(changes.join("\n\n"))
                .field(
                    "Store Link",
                    format!(
                        "[Link](https://discord.store/collections/all/products/{})",
                        &product.handle
                    ),
                    true,
                )
        });
    } else {
        store_embed = Embed::fake(|e| {
            e.colour(Colour::from_rgb(255, 169, 249))
                .title(format!(
                    "New variant of {} - {}",
                    &product.title, &variant.title
                ))
                .field("Handle", &product.handle, true)
                .field("Product Type", &product.product_type, true)
                .field("Weight", format!("{} grams", &variant.grams), true)
                .field("Is available", &variant.available, true)
                .field("Requires Shipping", &variant.requires_shipping, true)
                .field("Taxable", &variant.taxable, true)
                .field("Price", format!("{} USD", &variant.price), true)
                .field("Position", variant.position, true)
                .field(
                    "Store Link",
                    format!(
                        "[Link](https://discord.store/collections/all/products/{})",
                        &product.handle
                    ),
                    true,
                )
                .image(&product.images.first().unwrap().src)
        });
    }

    match webhook
        .execute(&http, true, |w| {
            w.content(content)
                .username("Store Manager")
                .embeds(vec![store_embed])
        })
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
