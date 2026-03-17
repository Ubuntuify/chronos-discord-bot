use chrono_tz::TZ_VARIANTS;
use poise::serenity_prelude as serenity;
use tracing::debug;

use crate::Context;

pub async fn autocomplete_tz<'a>(
    _ctx: Context<'a>,
    partial: &'a str,
) -> serenity::CreateAutocompleteResponse<'a> {
    let choices = TZ_VARIANTS.map(|tz| tz.to_string());

    let choices: Vec<_> = choices
        .clone()
        .into_iter()
        .filter(move |name| name.contains(partial))
        .take(25)
        .map(serenity::AutocompleteChoice::from)
        .collect();

    debug!("Sending autocomplete response for time zone...");

    serenity::CreateAutocompleteResponse::new().set_choices(choices)
}
