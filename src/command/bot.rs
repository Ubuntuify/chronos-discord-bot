#[poise::command(prefix_command, owners_only)]
pub async fn register(ctx: crate::Context<'_>) -> Result<(), crate::Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

pub async fn save(ctx: crate::Context<'_>) -> Result<(), crate::Error> {
    Ok(())
}
