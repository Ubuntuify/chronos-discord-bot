use poise::serenity_prelude::{self as serenity};

use crate::event::handler;

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

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, database::data::Data, Error>;

#[tokio::main]
#[tracing::instrument]
async fn main() {
    let _ = tracing_subscriber::fmt().with_target(false);

    let token =
        serenity::Token::from_env("DISCORD_TOKEN").expect("DISCORD_TOKEN not set, aborting!");

    let intents = serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework_options = poise::FrameworkOptions {
        commands: vec![
            command::time::time(),
            command::time::set_tz(),
            command::time::get_time(),
        ],
        ..Default::default()
    };

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(Box::new(poise::Framework::new(framework_options)))
        .await;

    client.unwrap().start().await.unwrap();
}
