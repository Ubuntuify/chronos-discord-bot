use poise::serenity_prelude::{
    self as serenity, CreateEmbed, FormattedTimestamp, Mentionable, Message, MessageBuilder,
    OnlineStatus, UserId,
};

use crate::structs::user_data::UserData;

#[poise::command(slash_command)]
pub async fn now(ctx: crate::Context<'_>, ephemeral: bool) -> Result<(), crate::Error> {
    let response = format!("Current time is: {}", FormattedTimestamp::now());

    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn set_timezone(
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
