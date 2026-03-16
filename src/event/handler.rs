use poise::serenity_prelude as serenity;

pub async fn handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, crate::database::data::Data, crate::Error>,
    data: &crate::database::data::Data,
) -> Result<(), crate::Error> {
    match event {
        _ => Ok(()),
    }
}
