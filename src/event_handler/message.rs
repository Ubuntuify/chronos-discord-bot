use chrono_tz::Tz;
use std::{
    collections::HashMap,
    sync::Arc,
    thread::{self, ScopedJoinHandle},
};
use tokio::sync::RwLock;
use tracing::{debug, info};

use poise::serenity_prelude::{
    self as serenity, FormattedTimestamp, FormattedTimestampStyle, MessageBuilder, Timestamp,
    UserId,
};

use crate::{
    Error,
    structs::user_data::{self, UserData},
    time::{get_closest_future_time, get_closest_future_time_12hr},
};

mod helpful_messages;
mod regex_matching;

#[tracing::instrument(skip(data, message, ctx))]
pub async fn translate_time_into_timestamp(
    ctx: &serenity::Context,
    message: &serenity::Message,
    data: &crate::Data,
) -> Result<(), crate::Error> {
    let user_id = message.author.id;

    if user_id == data.bot_id {
        return Ok(()); // don't do stuff when the bot itself is triggering it.
    }

    let mut timestamp: Timestamp = Timestamp::from_unix_timestamp(0)?; // placeholder, should be changed after, if
    // you get this, it's a bug.

    let simple_time = regex_matching::match_simple_time(&message.content);
    let preposition_time = regex_matching::match_preposition_time(&message.content);

    let user_data = data.user_data.read();

    if let Some(time) = simple_time {
        let user_data = user_data.await;
        let tz = match get_time_zone(&user_data, &message.author.id).await {
            Some(tz) => tz,
            None => return Ok(()),
        };

        timestamp = get_closest_future_time(time, tz)?.into();
    } else if let Some((time, is_24hr_clock)) = preposition_time {
        let user_data = user_data.await;
        let tz = match get_time_zone(&user_data, &message.author.id).await {
            Some(tz) => tz,
            None => return Ok(()),
        };

        match is_24hr_clock {
            true => timestamp = get_closest_future_time(time, tz)?.into(),
            false => timestamp = get_closest_future_time_12hr(time, tz).unwrap().into(),
        }
    }

    if timestamp == Timestamp::from_unix_timestamp(0)? {
        return Ok(());
    }

    let _ = send_timestamp_message(
        ctx,
        message,
        timestamp,
        Some("Accuracy is still being worked on, if there's an issue, please @ me."),
    )
    .await;

    Ok(())
}

async fn get_time_zone(user_data: &HashMap<UserId, UserData>, user: &UserId) -> Option<Tz> {
    match user_data.get(user) {
        Some(data) => data.time_zone,
        None => None,
    }
}

async fn send_timestamp_message(
    ctx: &serenity::Context,
    message: &serenity::Message,
    timestamp: serenity::Timestamp,
    warning: Option<&str>,
) -> Result<(), Error> {
    let response = MessageBuilder::new()
        .push_line(
            (FormattedTimestamp::new(timestamp, Some(FormattedTimestampStyle::LongDateTime)))
                .to_string(),
        )
        .push("-# ")
        .push(warning.unwrap_or_default())
        .to_string();

    let _ = message.reply(ctx, response).await;

    Ok(())
}
