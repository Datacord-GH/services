use crate::models::HackerOneThanks;
use serenity::prelude::SerenityError;
use serenity::{http::Http, model::channel::Embed, model::webhook::Webhook, utils::Colour};
use std::cmp::Ordering;
use std::env;

pub async fn send_new_user(hacker: &HackerOneThanks) -> Result<(), SerenityError> {
    println!("New hacker: {}", hacker.username);

    let http = Http::new("token");
    let token = env::var("HACKERONE_WEBHOOK_URL").expect("missing 'HACKERONE_WEBHOOK_URL' in .env");
    let webhook = Webhook::from_url(&http, &token).await?;

    let hackerone_embed = Embed::fake(|e| {
        e.colour(Colour::from_rgb(0, 0, 1))
            .description(format!(
                "**{}** `({})` was added with **{}** reputation\n**Position:** {}\n\n**Profile:** {}",
                hacker.username, hacker.id, hacker.reputation, hacker.position, hacker.get_hackerone_url()
            ))
            .thumbnail(&hacker.get_avatar_url())
    });

    webhook
        .execute(&http, true, |w| {
            w.content(format!(
                "<@&{}>",
                env::var("ROLE_ID").expect("missing ROLE_ID in .env"),
            ))
            .username("HackerOne Manager")
            .embeds(vec![hackerone_embed])
        })
        .await?;

    Ok(())
}

pub async fn send_updated_rep(
    new_hacker: &HackerOneThanks,
    old_hacker: &HackerOneThanks,
) -> Result<(), SerenityError> {
    println!(
        "Hacker reputation change: {} went from {} to {}",
        new_hacker.username, old_hacker.reputation, new_hacker.reputation
    );

    let http = Http::new("token");
    let token = env::var("HACKERONE_WEBHOOK_URL").expect("missing 'HACKERONE_WEBHOOK_URL' in .env");
    let webhook = Webhook::from_url(&http, &token).await?;
    let what_way = match new_hacker.reputation.cmp(&old_hacker.reputation) {
        Ordering::Less => String::from("decreased"),
        Ordering::Greater => String::from("increased"),
        Ordering::Equal => String::from("didnt change"),
    };

    let reputation_type = old_hacker.reputation_type(new_hacker.reputation - old_hacker.reputation);

    let hackerone_embed = Embed::fake(|e| {
        e.colour(Colour::from_rgb(0, 0, 1))
            .description(format!(
                "**{}** `({})` reputation {} from **{}** to **{}** `({})`\n**Position:** {}\n**Report type:** {:#?}\n\n**Profile:** {}",
                new_hacker.username,
                new_hacker.id,
                what_way,
                old_hacker.reputation,
                new_hacker.reputation,
                new_hacker.reputation - old_hacker.reputation,
                new_hacker.position,
                reputation_type,
                new_hacker.get_hackerone_url()
            ))
            .thumbnail(&new_hacker.get_avatar_url())
    });

    webhook
        .execute(&http, true, |w| {
            w.content(format!(
                "<@&{}>",
                env::var("ROLE_ID").expect("missing ROLE_ID in .env"),
            ))
            .username("HackerOne Manager")
            .embeds(vec![hackerone_embed])
        })
        .await?;

    Ok(())
}
