use chrono::{DateTime, NaiveDateTime, NaiveTime, Utc};
use poise::serenity_prelude::{self as serenity, FormattedTimestampStyle, Message};
use poise::serenity_prelude::{EventHandler, FormattedTimestamp};
use regex::Regex;
use tzfile::Tz;

use crate::event_handler::Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: &serenity::Context, message: Message) {
        println!("Message detected.");

        // See the crate `Regex`'s documentation for how this works.
        let haystack = message.content.clone();
        let re1 = Regex::new("^([0-1]?[0-9]|2[0-3]):([0-5][0-9])$").unwrap(); // should match anything
        // resembling 24/12hr time, like 12:00 or 22:00.

        let capture_str = re1.captures(&haystack);
        if let Some(capture) = capture_str {
            let hour: u32 = capture[1].to_string().parse().unwrap();
            let minute: u32 = capture[2].to_string().parse().unwrap();

            let user_time_zone = Tz::local().unwrap(); // TODO: hardcode for
            // now to build functionality.

            let date_time = Utc::now()
                .with_timezone(&&user_time_zone)
                .with_time(NaiveTime::from_hms_opt(hour, minute, 0).unwrap())
                .unwrap(); // unwrap is allowed
            // here as the RegEx should make sure that the input is valid already.

            let timestamp: FormattedTimestamp = FormattedTimestamp::new(
                date_time.into(),
                Some(FormattedTimestampStyle::ShortDateTime),
            );

            let response = format!("Timestamp attempt: {}", timestamp);

            let _ = message.reply(context, response).await;
        };

        let re2 = Regex::new("^([0-1]?[0-9])(am|pm)$").unwrap();
    }
}
