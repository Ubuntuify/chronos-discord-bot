#[poise::command(prefix_command)]
async fn force_write(ctx: crate::Context<'_>) -> Result<(), crate::Error> {
    let data = ctx.data().import_user_data("");

    Ok(())
}
