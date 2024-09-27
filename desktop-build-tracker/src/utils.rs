use crate::models::{Build, ReleaseChannel};
use serenity::all::{Colour, CreateEmbed, EmbedMessageBuilding, ExecuteWebhook};
use serenity::prelude::SerenityError;
use serenity::{http::Http, model::channel::Embed, model::webhook::Webhook};
use std::env;

fn embed_info(channel: &ReleaseChannel) -> (String, Colour) {
    match channel {
        ReleaseChannel::Stable => (
            env::var("STABLE_ROLE_ID").expect("missing STABLE_ROLE_ID in .env"),
            Colour::from_rgb(7, 180, 255),
        ),
        ReleaseChannel::PTB => (
            env::var("PTB_ROLE_ID").expect("missing PTB_ROLE_ID in .env"),
            Colour::from_rgb(155, 89, 182),
        ),
        ReleaseChannel::Canary => (
            env::var("CANARY_ROLE_ID").expect("missing CANARY_ROLE_ID in .env"),
            Colour::from_rgb(230, 126, 34),
        ),
    }
}

fn emoji_format(val: &bool) -> &str {
    match val {
        true => "<:greenTick:851441548922847262>",
        false => "<:redTick:851441548994412614>",
    }
}

pub async fn send_message(build: &Build) -> Result<(), SerenityError> {
    let http = Http::new("token");
    let token =
        env::var("DESKTOP_BUILD_WEBHOOK_URL").expect("missing DESKTOP_BUILD_WEBHOOK_URL in .env");
    let webhook = Webhook::from_url(&http, &token).await?;

    let (role_id, colour) = embed_info(&build.channel);

    let build_embed = CreateEmbed::new()
        .title(format!("{:#?} update", build.channel))
        .colour(colour)
        .field("Build Number", format!("`{}`", build.build_number), true)
        .field("Build Id", format!("`{}`", build.build_id), true)
        .field("Build Hash", format!("`{}`", build.build_hash), false)
        .field("Reverted build", emoji_format(&build.is_revert), true)
        .field(
            "Builds today",
            format!("`{}`", build.amount_of_builds_today + 1),
            true,
        );

    let webhook_builder = ExecuteWebhook::new()
        .content(format!("<@&{}>", role_id))
        .username("Desktop Build Manager")
        .embeds(vec![build_embed]);
    webhook.execute(&http, true, webhook_builder).await?;

    Ok(())
}
