use poise::{
    CreateReply,
    serenity_prelude::{self as serenity, CreateEmbed, FormattedTimestamp},
};

#[poise::command(slash_command)]
pub async fn now(ctx: crate::Context<'_>) -> Result<(), crate::Error> {
    let response = format!("Current time is: {}", FormattedTimestamp::now()); // TODO:
    // check online members

    ctx.reply(response).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn set_timezone(
    ctx: crate::Context<'_>,
    #[description = "The user you want to set the timezone for."] user: serenity::Member,
    #[description = "A POSIX compliant timezone string."] timezone: String,
) -> Result<(), crate::Error> {
    let tz = match tzfile::Tz::named(&timezone) {
        Ok(tz) => tz,
        Err(e) => {
            let embed = CreateEmbed::new().title("Error occured.");
            let response = CreateReply::default().embed(embed);

            let _ = ctx.send(response).await;
            return Err(Box::new(e));
        }
    };

    Ok(())
}
