use std::path::Path;

use poise::{
    Framework,
    serenity_prelude::{self as serenity},
};

extern crate tracing;

struct Data {
    bot_id: serenity::UserId,
    time_zone: Option<tzfile::Tz>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

mod commands;
mod event_handler;
mod structs;
mod time;
mod utility;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN unset, cannot start bot (no credentials, maybe you haven't loaded your environment variables?)");
    let intents = serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::GUILDS
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    if !Path::new("/usr/share/zoneinfo").exists() {
        panic!(
            "No timezone information found on system, cannot continue... If you're on a non-standard system, such as NixOS, please symlink your timezone information at /usr/share/zoneinfo"
        )
    };

    println!("Starting bot...");

    let framework: Framework<Data, Error> = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: crate::commands::all(),
            event_handler: |ctx, event, framework, data| {
                Box::pin(crate::event_handler::event_handler(
                    ctx, event, framework, data,
                ))
            },
            ..Default::default()
        })
        .setup(|_ctx, ready, framework| {
            Box::pin(async move {
                //poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    bot_id: ready.user.id,
                    time_zone: None,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
