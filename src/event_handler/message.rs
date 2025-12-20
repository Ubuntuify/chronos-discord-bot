use chrono_tz::Tz;
use std::collections::HashMap;

use poise::serenity_prelude::{
    self as serenity, FormattedTimestamp, FormattedTimestampStyle, Mentionable, MessageBuilder,
    Timestamp, UserId,
};

use crate::{
    Error,
    structs::data::UserData,
    time::{get_closest_future_time, get_closest_future_time_12hr},
};

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

    let notz_error = crate::strings::errors::NO_TIME_ZONE
        .replace("{user}", &message.author.mention().to_string());

    if let Some(time) = simple_time {
        let user_data = user_data.await;
        let tz = match get_time_zone(&user_data, &message.author.id).await {
            Some(tz) => tz,
            None => {
                let _ = message.reply(ctx, notz_error).await;
                return Ok(());
            }
        };

        timestamp = get_closest_future_time(time, tz)?.into();
    } else if let Some((time, is_24hr_clock)) = preposition_time {
        let user_data = user_data.await;
        let tz = match get_time_zone(&user_data, &message.author.id).await {
            Some(tz) => tz,
            None => {
                let _ = message.reply(ctx, notz_error).await;
                return Ok(());
            }
        };

        match is_24hr_clock {
            true => timestamp = get_closest_future_time(time, tz)?.into(),
            false => timestamp = get_closest_future_time_12hr(time, tz).unwrap().into(),
        }
    }

    if timestamp == Timestamp::from_unix_timestamp(0)? {
        return Ok(()); // it's not being set, so abort.
    }

    let _ = send_timestamp_message(ctx, message, timestamp, None).await;

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
    let mut response = MessageBuilder::new();

    response.push_line(
        (FormattedTimestamp::new(timestamp, Some(FormattedTimestampStyle::LongDateTime)))
            .to_string(),
    );

    if let Some(warning) = warning {
        response.push("-# ").push(warning);
    }

    let response: String = response.to_string();

    let _ = message.reply(ctx, response).await;

    Ok(())
}
