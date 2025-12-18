use poise::serenity_prelude as serenity;

pub async fn no_time_zone(ctx: &serenity::Context, message: &serenity::Message) {
    message.reply(ctx, "`no_time_zone_found`").await;
}

