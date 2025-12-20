use crate::{Data, Error};
use poise::serenity_prelude as serenity;
use tracing::{Level, error, info, span, warn};

mod message;
mod ready;

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

            let data_future = ready::load_data(data);

            info!("Starting to load (if available) saved guild and user data...");

            match tokio::join!(data_future) {
                (Ok(()),) => {}
                (Err(e),) => {
                    warn!("Unable to read data from disk, please investigate.");
                    error!("{}", e);
                }
            };

            Ok(())
        }
        serenity::FullEvent::Message { new_message } => {
            let _enter = message_span.enter();

            let timestamp_future = crate::event_handler::message::translate_time_into_timestamp(
                ctx,
                new_message,
                data,
            );

            // Rust futures are lazy and need to be joined at the same time to actually run,
            // similar to Promise.all on JS.
            let _ = tokio::join!(timestamp_future);

            Ok(())
        }
        _ => Ok(()),
    }
}
