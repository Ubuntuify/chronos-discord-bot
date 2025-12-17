use poise::{
    Framework, PrefixFrameworkOptions,
    serenity_prelude::{self as serenity},
};

struct Data {
    time_zone: Option<tzfile::Tz>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

mod commands;
mod event_handler;
mod time;
mod utility;

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN unset, cannot start bot (no credentials, maybe you haven't loaded your environment variables?)");
    let intents = serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::GUILDS
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    println!("Starting bot...");

    let framework: Framework<Data, Error> = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: crate::commands::all(),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                //poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { time_zone: None })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .event_handler(crate::event_handler::Handler)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
