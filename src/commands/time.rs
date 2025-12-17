use std::fmt::format;

use poise::serenity_prelude::{self as serenity, FormattedTimestamp, Timestamp};

#[poise::command(slash_command)]
pub async fn now(ctx: crate::Context<'_>) -> Result<(), crate::Error> {
    let response = format!("Current time is: {}", FormattedTimestamp::now());

    ctx.reply(response).await?;
    Ok(())
}
