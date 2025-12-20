use std::path::Path;

#[poise::command(prefix_command, owners_only)]
pub async fn force_write(
    ctx: crate::Context<'_>,
    path: Option<String>,
) -> Result<(), crate::Error> {
    let data: &crate::data::Data = ctx.data();
    let path = match &path {
        Some(path) => Path::new(path),
        None => &data.data_path.join("user_data.json"),
    };

    let _ = data.save_user_data(path.into()).await;

    let _ = ctx
        .reply(format!(
            "Successfully saved the `user_data` HashMap at `{}`",
            path.to_str().unwrap().to_owned()
        ))
        .await;

    Ok(())
}

#[poise::command(prefix_command, owners_only)]
pub async fn register(ctx: crate::Context<'_>) -> Result<(), crate::Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
