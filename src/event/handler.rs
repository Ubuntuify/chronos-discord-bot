use poise::serenity_prelude as serenity;

pub async fn handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, crate::database::data::Data, crate::Error>,
    _data: &crate::database::data::Data,
) -> Result<(), crate::Error> {
    match event {
        _ => Ok(()),
    }
}
