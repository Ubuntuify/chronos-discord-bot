/***
* Modules
*/

use crate::{Data, Error};

mod bot;
mod setup;
mod time;

/***
* Returns all public (as in, registered globally) commands
*/
pub fn public() -> Vec<poise::Command<Data, Error>> {
    vec![
        crate::commands::time::time(),
        crate::commands::time::context_get_user_time(),
    ]
}

/***
* Returns all private (as in, registered to some guilds, most likely a development one)
* commands.
*/
pub fn private() -> Vec<poise::Command<Data, Error>> {
    vec![
        crate::commands::bot::register(),
        crate::commands::bot::force_write(),
    ]
}

/***
* Returns all commands (whether registered globally or not)
*/
pub fn all() -> Vec<poise::Command<Data, Error>> {
    let mut commands = public();
    commands.append(&mut private());

    commands
}
