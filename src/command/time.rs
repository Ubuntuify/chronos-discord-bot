use chrono::Utc;
use poise::serenity_prelude as serenity;

mod users;

#[poise::command(slash_command, subcommands("set_tz", "get_time"))]
pub async fn time(_ctx: crate::Context<'_>) -> Result<(), crate::Error> {
    Ok(()) // shouldn't ever run anyways, since this is a parent command and can't be invoked
    // through prefix commands
}

#[poise::command(slash_command, rename = "set")]
pub async fn set_tz(
    ctx: crate::Context<'_>,
    #[description = "The user you want to set the timezone for (defaults to you)."] user: Option<
        serenity::Member,
    >,
    #[description = "Timezone to set"] timezone: String,
) -> Result<(), crate::Error> {
    let user_id = match user {
        Some(user) => user.user.id,
        None => ctx.author().id,
    };

    let timezone: chrono_tz::Tz = timezone.parse()?;

    users::set_user_tz(&ctx.data(), user_id, &timezone).await?;

    ctx.reply(format!(
        "Successfully set <@{}>'s time zone to `{}`",
        user_id, timezone
    ))
    .await?;

    Ok(())
}

#[poise::command(
    context_menu_command = "What time is it for them?",
    slash_command,
    rename = "user" // rename the slash command
)]
pub async fn get_time(ctx: crate::Context<'_>, user: serenity::User) -> Result<(), crate::Error> {
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
