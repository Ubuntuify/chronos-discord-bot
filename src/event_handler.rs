use crate::{Data, Error};
use poise::serenity_prelude as serenity;
use tracing::{Level, info, span};

mod message;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &crate::Data,
) -> Result<(), Error> {
    let ready_span = span!(Level::INFO, "ready");
    let message_span = span!(Level::TRACE, "message");

    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            let _enter = ready_span.enter();

            const PKG_NAME: &str = env!("CARGO_PKG_NAME");
            const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

            info!(
                "Successfully logged in as {}, running on version {} of crate {}.",
                data_about_bot.user.name, PKG_VERSION, PKG_NAME
            );
            Ok(())
        }
        serenity::FullEvent::Message { new_message } => {
            let _enter = message_span.enter();

            crate::event_handler::message::translate_time_into_timestamp(ctx, new_message, data)
                .await;

            Ok(())
        }
        _ => Ok(()),
    }
}
