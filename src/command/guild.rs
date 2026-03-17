use chrono::Utc;
use poise::serenity_prelude::{CreateEmbed, CreateEmbedAuthor, FormattedTimestamp};
use tracing::info;

pub mod components;
pub mod guilds;

#[poise::command(slash_command, subcommand_required, subcommands("guild_timezones"))]
pub async fn guild(_ctx: crate::Context<'_>) -> Result<(), crate::Error> {
    Ok(())
}

#[poise::command(slash_command, rename = "time")]
pub async fn guild_timezones(ctx: crate::Context<'_>) -> Result<(), crate::Error> {
    let guild_id = ctx.guild_id().unwrap();

    if let Some(common_tzs) = guilds::find_guild_common_tzs(&ctx.data(), guild_id).await {
        let guild = ctx.guild().unwrap();
        let guild_icon = guild.icon_url().unwrap();
        let author = CreateEmbedAuthor::new(&guild.name).icon_url(guild_icon);

        let mut embed = CreateEmbed::new()
            .field("Current time", FormattedTimestamp::now().to_string(), false)
            .author(author);

        for tz in common_tzs {
            embed = embed.field(
                tz.to_string(),
                Utc::now()
                    .with_timezone(&tz)
                    .format("%A, %d %B %Y at %_I:%M%P")
                    .to_string(),
                false,
            );
            info!(
                "Added {} time zone to list to send to guild {}",
                tz, guild.id
            );
        }
    } else {
        ctx.reply("❌ There are no common time zones stored in this guild.")
            .await?;
    }

    Ok(())
}
