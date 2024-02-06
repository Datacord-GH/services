use crate::models::{Job, JobDatabase, TypeOfUpdate};
use serenity::prelude::SerenityError;
use serenity::{http::Http, model::channel::Embed, model::webhook::Webhook, utils::Colour};
use std::env;

pub async fn send_new_job_message(job: &Job) -> Result<(), SerenityError> {
    println!(
        "[+] New job {} ({}) @ {}",
        job.title,
        job.id,
        job.get_discord_url()
    );

    let http = Http::new("token");
    let token = env::var("WEBHOOK_URL").expect("missing WEBHOOK_URL in .env");
    let webhook = Webhook::from_url(&http, &token).await?;

    let changelog_embed = Embed::fake(|e| {
        e.colour(Colour::from_rgb(0, 255, 0))
            .title(&job.title)
            .field("Department", &job.departments.first().unwrap().name, true)
            .field("Location", &job.location.name, true)
            .field("Office", &job.offices.first().unwrap().name, true)
            .field(
                "Discord Jobs",
                format!("[Here]({})", &job.get_discord_url()),
                true,
            )
            .field("Updated At", &job.get_updated_at(), true)
            .field(
                "Education",
                &job.education.clone().unwrap_or_else(|| String::from("N/A")),
                true,
            )
            .field("Id", &job.id, true)
            .field("Internal Job Id", &job.internal_job_id, true)
            .field("Requisition Id", &job.requisition_id, true)
    });

    webhook
        .execute(&http, true, |w| {
            w.content(format!(
                "<@&{}>",
                env::var("ROLE_ID").expect("missing ROLE_ID in .env"),
            ))
            .username("Job Manager")
            .embeds(vec![changelog_embed])
        })
        .await?;

    Ok(())
}

pub async fn send_updated_job_message(
    job: &JobDatabase,
    update: TypeOfUpdate,
) -> Result<(), SerenityError> {
    println!(
        "[~] Updated job {} ({}) -> {:#?}",
        job.title, job.id, update
    );

    let http = Http::new("token");
    let token = env::var("WEBHOOK_URL").expect("missing WEBHOOK_URL in .env");
    let webhook = Webhook::from_url(&http, &token).await?;

    let coluor = match update {
        TypeOfUpdate::AddedBackFromRemove => Colour::from_rgb(0, 0, 255),
        TypeOfUpdate::Removed => Colour::from_rgb(255, 0, 0),
    };

    let changelog_embed = Embed::fake(|e| {
        e.colour(coluor)
            .title(&job.title)
            .field("Id", &job.id, true)
            .field(
                "Discord Jobs",
                format!("[Here]({})", &job.get_discord_url()),
                true,
            )
    });

    webhook
        .execute(&http, true, |w| {
            w.content(format!(
                "<@&{}>",
                env::var("ROLE_ID").expect("missing ROLE_ID in .env"),
            ))
            .username("Job Manager")
            .embeds(vec![changelog_embed])
        })
        .await?;

    Ok(())
}
