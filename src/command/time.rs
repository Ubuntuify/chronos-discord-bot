use chrono::Utc;
use chrono_tz::TZ_VARIANTS;
use poise::serenity_prelude::{self as serenity, MessageBuilder};

pub mod components;
mod users;

#[poise::command(
    slash_command,
    subcommands("set_tz", "sc_stub_get_user_time"),
    subcommand_required
)]
pub async fn time(_ctx: crate::Context<'_>) -> Result<(), crate::Error> {
    Ok(()) // shouldn't ever run anyways, since this is a parent command and can't be invoked
    // through prefix commands
}

#[poise::command(slash_command, rename = "set")]
pub async fn set_tz(
    ctx: crate::Context<'_>,
    #[description = "The user you want to set the timezone for (defaults to you)."] user: Option<
        serenity::User,
    >,
    #[description = "Timezone to set"] timezone: String,
) -> Result<(), crate::Error> {
    let user_id = match user {
        Some(user) => user.id,
        None => ctx.author().id,
    };

    let timezone: chrono_tz::Tz = match timezone.parse() {
        Ok(tz) => tz,
        Err(_err) => {
            let mut response = MessageBuilder::new().push("I can't understand that time zone.");
            let timezones = TZ_VARIANTS.map(|t| t.to_string().to_owned());

            let similar_timezone: Vec<String> = timezones
                .iter()
                .filter(|f| f.contains(&timezone))
                .map(|f| f.to_owned())
                .collect();

            if !similar_timezone.is_empty() {
                response = response
                    .push(" Did you mean ")
                    .push_mono_safe(&*similar_timezone[0])
                    .push("?");
            };

            ctx.reply(response.to_string()).await?;

            return Ok(()); // error handled
        }
    };

    users::set_user_tz(&ctx.data(), user_id, &timezone).await?;

    ctx.reply(format!(
        "Successfully set <@{}>'s time zone to `{}`",
        user_id, timezone
    ))
    .await?;

    Ok(())
}

#[poise::command(context_menu_command = "What time is it for them?")]
pub async fn ct_stub_get_time(
    // context menu stub
    ctx: crate::Context<'_>,
    user: serenity::User,
) -> Result<(), crate::Error> {
    get_time(ctx, user).await
}

#[poise::command(
    slash_command,
    rename = "user" // rename the slash command
)]
pub async fn sc_stub_get_user_time(
    // slash command stuff, both get handed over to the same logic
    ctx: crate::Context<'_>,
    #[description = "The user you want to get the time of."] user: Option<serenity::User>,
) -> Result<(), crate::Error> {
    let user = match user {
        Some(user) => user,
        None => ctx.author().to_owned(),
    };
    get_time(ctx, user).await
}

async fn get_time(ctx: crate::Context<'_>, user: serenity::User) -> Result<(), crate::Error> {
    match users::find_user_tz(&ctx.data(), user.id).await {
        Some(tz) => {
            let now = Utc::now().with_timezone(&tz);

            ctx.reply(format!(
                "It is currently **{}** for <@{}>.",
                now.format("%A, %d %B %Y at %r"),
                user.id
            ))
            .await?;
        }
        None => {
            ctx.reply(format!(
                "<@{}> does not have a time zone set yet. I don't know what time zone they're in.",
                user.id
            ))
            .await?;
        }
    };

    Ok(())
}
