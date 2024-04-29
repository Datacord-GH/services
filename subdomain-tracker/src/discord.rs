use serenity::prelude::SerenityError;
use serenity::{http::Http, model::channel::Embed, model::webhook::Webhook, utils::Colour};
use std::env;

use crate::models::endpoints::Endpoint;

pub async fn send_message(endpoint: &Endpoint, subdomain: &String) -> Result<(), SerenityError> {
    let http = Http::new("token");
    let token = env::var("SUBDOMAIN_WEBHOOK_URL").expect("missing SUBDOMAIN_WEBHOOK_URL in .env");
    let webhook = Webhook::from_url(&http, &token).await?;

    let changelog_embed = Embed::fake(|e| {
        e.colour(Colour::from_rgb(129, 207, 118))
            .field("Url", &endpoint.url, true)
            .field("Subdomain", format!("https://{}", &subdomain), true)
            .footer(|f| f.text(&endpoint.description))
    });

    webhook
        .execute(&http, true, |w| {
            w.content(format!(
                "<@&{}>",
                env::var("ROLE_ID").expect("missing ROLE_ID in .env"),
            ))
            .username("Subdomain Manager")
            .embeds(vec![changelog_embed])
        })
        .await?;

    Ok(())
}
