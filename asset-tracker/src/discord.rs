use serenity::model::prelude::AttachmentType;
use serenity::prelude::SerenityError;
use serenity::{http::Http, model::webhook::Webhook};
use std::borrow::Cow;
use std::{env, vec};

use crate::models::DiscordMessage;
use crate::utils;

pub async fn send_message(msg: Vec<DiscordMessage>) -> Result<(), SerenityError> {
    let http = Http::new("token");
    let token = env::var("WEBHOOK_URL").expect("missing WEBHOOK_URL in .env");
    let webhook = Webhook::from_url(&http, &token).await?;

    let mut files: Vec<AttachmentType> = vec![];
    for file in msg {
        let byte = utils::clone_buffer(&file.data);

        files.push(AttachmentType::Bytes {
            data: Cow::from(byte),
            filename: format!("{}.{}", file.file_name, file.file_type),
        })
    }

    webhook
        .execute(&http, true, |w| {
            w.content(format!(
                "<@&{}>",
                env::var("ROLE_ID").expect("missing ROLE_ID in .env"),
            ))
            .username("Asset Manager")
            .add_files(files)
        })
        .await?;

    Ok(())
}
