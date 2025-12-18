use poise::serenity_prelude::{
    self as serenity, FormattedTimestamp, FormattedTimestampStyle, MessageBuilder,
};
use tzfile::Tz;

use crate::{Error, time::get_closest_future_time};

mod regex_matching;

pub async fn translate_time_into_timestamp(
    ctx: &serenity::Context,
    message: &serenity::Message,
    data: &crate::Data,
) {
    if message.author.id == data.bot_id {
        return;
    };

    if let Some(time) = regex_matching::match_simple_time(message.content.clone()) {
        let time = get_closest_future_time(time, Tz::local().unwrap()).unwrap();

        let _ = send_timestamp_message(
            ctx,
            message,
            time.into(),
            Some("Time may be inaccurate, please report if this is not the time you meant."),
        )
        .await;
    };
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
