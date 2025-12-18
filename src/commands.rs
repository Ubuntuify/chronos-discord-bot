/***
* Modules
*/

use crate::{Data, Error};

mod time;

/***
* Basic commmands that don't require a module;
*/

#[poise::command(prefix_command)]
async fn register(ctx: crate::Context<'_>) -> Result<(), crate::Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

/***
* Returns all public (as in, registered globally) commands
*/
pub fn public() -> Vec<poise::Command<Data, Error>> {
    vec![
        crate::commands::time::now(),
        crate::commands::time::set_timezone(),
    ]
}

/***
* Returns all private (as in, registered to some guilds, most likely a development one)
* commands.
*/
pub fn private() -> Vec<poise::Command<Data, Error>> {
    vec![crate::commands::register()]
}

/***
* Returns all commands (whether registered globally or not)
*/
pub fn all() -> Vec<poise::Command<Data, Error>> {
    let mut commands = public();
    commands.append(&mut private());

    commands
}
