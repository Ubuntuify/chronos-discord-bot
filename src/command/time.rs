use chrono::Utc;
use chrono_tz::TZ_VARIANTS;
use poise::{
    CreateReply,
    serenity_prelude::{
        self as serenity, ButtonStyle, CacheHttp, CreateButton, CreateEmbed, CreateEmbedAuthor,
        CreateInteractionResponseFollowup, CreateMessage, EmbedAuthor, FormattedTimestamp,
        MessageBuilder, ReactionType,
    },
};

mod autocomplete;
pub mod components;
mod users;

#[poise::command(slash_command, rename = "timeset")]
pub async fn set_tz<'a>(
    ctx: crate::Context<'a>,
    #[description = "The user you want to set the timezone for (defaults to you)."] user: Option<
        serenity::User,
    >,
    #[description = "Timezone to set"]
    #[autocomplete = "autocomplete::autocomplete_tz"]
    timezone: String,
) -> Result<(), crate::Error> {
    let user_id = match user {
        Some(user) => user.id,
        None => ctx.author().id,
    };
    let embed = serenity::CreateEmbed::new();

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

            ctx.reply(response.build()).await?;

            return Ok(()); // error handled
        }
    };

    users::set_user_tz(&ctx.data(), user_id, &timezone).await?;

    let embed = embed.description(format!(
        "Successfully set <@{}>'s time zone to `{}`",
        user_id, timezone
    ));

    let buttons = [CreateButton::new("add_common_tz")
        .emoji("⏰".parse::<ReactionType>().unwrap())
        .label("Add to common timezones")];

    let components = [serenity::CreateComponent::ActionRow(
        serenity::CreateActionRow::buttons(&buttons),
    )];

    let reply: CreateReply = poise::CreateReply::default()
        .components(&components)
        .embed(embed.clone());

    let message = ctx.send(reply).await?;

    let serenity_ctx = ctx.serenity_context();

    // edit old interaction

    let buttons = (&buttons).clone().map(|f| f.disabled(true));

    let components = [serenity::CreateComponent::ActionRow(
        serenity::CreateActionRow::buttons(&buttons),
    )];

    while let Some(mci) = serenity::ComponentInteractionCollector::new(serenity_ctx)
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == "add_common_tz")
        .await
    {
        mci.defer_ephemeral(serenity_ctx.http()).await?;

        message
            .edit(
                ctx,
                CreateReply::default()
                    .embed(embed.clone())
                    .components(&components),
            )
            .await?;

        // actually add to guild common timezone list

        crate::command::guild::guilds::add_guild_common_tz(
            &ctx.data(),
            ctx.guild_id().unwrap(),
            timezone,
        )
        .await?;

        // response to new interaction

        let embed = CreateEmbed::new().description(format!(
            "Added `{}` to the **{}**'s common timezones.",
            timezone.to_string(),
            ctx.guild().unwrap().name
        ));

        mci.create_followup(
            serenity_ctx.http(),
            CreateInteractionResponseFollowup::new().add_embed(embed),
        )
        .await?;
    }

    message
        .edit(
            ctx,
            CreateReply::default()
                .embed(embed.clone())
                .components(&components),
        )
        .await?;

    Ok(())
}

#[poise::command(
    context_menu_command = "What time is it for them?",
    slash_command,
    rename = "time" // rename the slash command
)]
pub async fn get_time(ctx: crate::Context<'_>, user: serenity::User) -> Result<(), crate::Error> {
    match users::find_user_tz(&ctx.data(), user.id).await {
        Some(tz) => {
            let now = Utc::now()
                .with_timezone(&tz)
                .format("%A, %d %B %Y at %_I:%M%P")
                .to_string();

            let user_display_name = user.display_name();
            let author = CreateEmbedAuthor::new(user_display_name)
                .icon_url(user.avatar_url().unwrap_or_default());

            let embed = CreateEmbed::new()
                .description(format!("It is currently **{}** for <@{}>", now, user.id))
                .author(author)
                .field("Current time", FormattedTimestamp::now().to_string(), false)
                .field(
                    format!("Current time for {}", user_display_name),
                    now + " `" + &tz.to_string() + "`",
                    false,
                );
            let reply = CreateReply::default().embed(embed);

            ctx.send(reply).await?;
        }
        None => {
            let embed = CreateEmbed::new().description(format!(
                "<@{}> does not have a time zone set yet. I don't know what time zone they're in.",
                user.id
            ));

            let reply = CreateReply::default().embed(embed);
            ctx.send(reply).await?;
        }
    };

    Ok(())
}
