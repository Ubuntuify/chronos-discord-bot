use chrono::Utc;
use poise::serenity_prelude::{
    self as serenity, FormattedTimestamp, Mentionable, MessageBuilder, UserId,
};

use crate::structs::data::UserData;

#[poise::command(slash_command, subcommands("now", "set_timezone"))]
pub async fn time(_ctx: crate::Context<'_>) -> Result<(), crate::Error> {
    Ok(()) // shouldn't error and run anyways, it's a parent command that's not a slash one
}

/// Get the current time for the members within the server, and a Discord timestamp to see your
/// own.
#[poise::command(slash_command, rename = "now")]
async fn now(ctx: crate::Context<'_>, ephemeral: Option<bool>) -> Result<(), crate::Error> {
    let response = format!("Current time is: {}", FormattedTimestamp::now());

    let user_data = &ctx.data().user_data;
    let user_data_lock = user_data.read().await;

    let default = match user_data_lock.get(&ctx.author().id) {
        Some(data) => data.prefers_ephemeral,
        None => false,
    };

    if ephemeral.unwrap_or(default) {
        let _ = ctx.defer_ephemeral().await;
    }

    ctx.say(response).await?;
    Ok(())
}

/// Set the timezone of a guild member, allows for automatic time translation when sending a
/// message.
#[poise::command(slash_command, rename = "set")]
async fn set_timezone(
    ctx: crate::Context<'_>,
    #[description = "The user you want to set the timezone for."] user: Option<serenity::Member>,
    #[description = "A POSIX compliant timezone string."] timezone: String,
) -> Result<(), crate::Error> {
    let tz: chrono_tz::Tz = timezone.parse().unwrap();

    let target_user: UserId;
    let mut target_is_author: bool = true;

    match user {
        Some(member) => {
            let target_is_author = &mut target_is_author;
            *target_is_author = false;

            target_user = member.user.id;
        }
        None => target_user = ctx.author().id,
    }

    let mut user_data_lock = ctx // go for instant drop
        .data()
        .user_data
        .write()
        .await;

    user_data_lock
        .entry(target_user)
        .and_modify(|data| data.time_zone = Some(tz))
        .or_insert(UserData::new(None, Some(tz)));

    drop(user_data_lock);

    let mention = target_user.mention();

    let mut response = MessageBuilder::new();

    response
        .push("Successfully set time zone to ")
        .push_mono(tz.to_string())
        .push(format!(" for {}.", mention));

    ctx.say(response.to_string()).await?;

    Ok(())
}

#[poise::command(context_menu_command = "What time is it?")]
pub async fn context_get_user_time(
    ctx: crate::Context<'_>,
    user: serenity::User,
) -> Result<(), crate::Error> {
    let _ = ctx.defer_ephemeral().await;

    let tz = match ctx.data().get_owned_user_data(user.id).await {
        Some(data) => data.time_zone,
        None => None,
    };

    if let Some(tz) = tz {
        let now = Utc::now()
            .with_timezone(&tz)
            .format("%A, %d %B %Y at %r")
            .to_string();

        let text = format!("It is currently __{}__ for {}.", now, user.mention());
        let _ = ctx.reply(text).await;
    } else {
        let error_msg =
            crate::strings::errors::NO_TIME_ZONE.replace("{user}", &user.mention().to_string());

        let _ = ctx.reply(error_msg).await;
    }

    Ok(())
}
