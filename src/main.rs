use std::sync::Arc;

use poise::serenity_prelude::{self as serenity};

use crate::database::data::Data;

pub mod command {
    pub mod bot;
    pub mod guild;
    pub mod time;
}

pub mod database {
    pub mod data;
}

pub mod event {
    pub mod handler;
}

extern crate tracing;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, database::data::Data, Error>;

#[tokio::main]
#[tracing::instrument]
async fn main() {
    tracing_subscriber::fmt().init();

    let token =
        serenity::Token::from_env("DISCORD_TOKEN").expect("DISCORD_TOKEN not set, aborting!");

    let intents = serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework_options = poise::FrameworkOptions {
        commands: vec![
            command::bot::register(),
            command::time::time(),
            command::time::set_tz(),
            command::time::ct_stub_get_time(),
            command::time::sc_stub_get_user_time(),
        ],
        ..Default::default()
    };

    tracing::info!("Seems good to go, launching bot!");

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(Box::new(poise::Framework::new(framework_options)))
        .data(Arc::new(Data::new()))
        .await;

    client.unwrap().start().await.unwrap();
}
