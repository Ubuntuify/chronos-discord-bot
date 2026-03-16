use poise::{
    Framework,
    serenity_prelude::{self as serenity, Ready},
};

use crate::{database::data::Data, event::handler};

pub mod command {
    mod bot;
    mod guild;
    mod time;
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

    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set, aborting!");

    let intents = serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework: Framework<database::data::Data, Error> = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![],
            event_handler: |ctx, event, framework, data| {
                Box::pin(handler::handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                let data = Data::new();

                Ok(data)
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
