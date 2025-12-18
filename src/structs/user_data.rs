use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub prefers_ephemeral: bool, // prefers slash commands be secret, when possible (default to false)
    pub time_zone: Option<chrono_tz::Tz>,
}

impl UserData {
    pub fn new(prefer_ephemeral: Option<bool>, time_zone: Option<chrono_tz::Tz>) -> UserData {
        UserData {
            prefers_ephemeral: prefer_ephemeral.unwrap_or(false),
            time_zone,
        }
    }
}
