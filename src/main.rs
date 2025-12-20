use std::{collections::HashMap, sync::Arc};

use poise::{
    Framework,
    serenity_prelude::{self as serenity},
};

use tokio::sync::RwLock;
use tracing::info;

use crate::data::Data;

extern crate tracing;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

mod commands;
mod data;
mod event_handler;
mod strings;
mod structs;
mod time;

#[tracing::instrument]
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // start using tracing_subscriber logs

    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN unset, cannot start bot (no credentials, maybe you haven't loaded your environment variables?)");
    let intents = serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::GUILDS
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    info!("Seems to be good, starting to initialize bot.");

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
        .setup(|_ctx, ready, _framework| {
            Box::pin(async move {
                //poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    bot_id: ready.user.id,
                    data_path: crate::data::get_data_path(),
                    user_data: Arc::new(RwLock::new(HashMap::new())),
                    guild_data: Arc::new(RwLock::new(HashMap::new())),
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
