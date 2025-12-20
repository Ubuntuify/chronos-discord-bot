use chrono_tz::Tz;

use poise::serenity_prelude::{
    self as serenity, FormattedTimestamp, FormattedTimestampStyle, Mentionable, MessageBuilder,
    Timestamp,
};

use crate::{
    Error,
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

    let user_data = data.get_owned_user_data(user_id);
    let notz_error = crate::strings::errors::NO_TIME_ZONE
        .replace("{user}", &message.author.mention().to_string());

    let regex_vec = vec![simple_time, preposition_time];

    let mut tz: Tz = Tz::UTC;

    // means there's a match within the vector
    if !&regex_vec.contains(&None) {
        tz = match &user_data.await {
            Some(d) => match d.time_zone {
                Some(tz) => tz,
                None => {
                    let _ = message.reply(ctx, notz_error).await;
                    return Ok(());
                }
            },
            None => {
                let _ = message.reply(ctx, notz_error).await;
                return Ok(());
            }
        };
    }

    for time in regex_vec {
        if let Some((time, is_24hr_time)) = time {
            match is_24hr_time {
                true => timestamp = get_closest_future_time(time, tz)?.into(),
                false => timestamp = get_closest_future_time_12hr(time, tz).unwrap().into(),
            }
        }
    }

    if timestamp == Timestamp::from_unix_timestamp(0)? {
        return Ok(()); // it's not being set, so abort.
    }

    let _ = send_timestamp_message(ctx, message, timestamp, None).await;

    Ok(())
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
