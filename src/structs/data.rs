use chrono_tz::Tz;
use poise::serenity_prelude::{self as serenity, UserId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::RwLock;
use tracing::warn;

pub struct Data {
    pub bot_id: serenity::UserId,
    pub data_path: Box<std::path::Path>,
    pub user_data: Arc<RwLock<HashMap<serenity::UserId, UserData>>>,
    pub guild_data: Arc<RwLock<HashMap<serenity::GuildId, GuildData>>>,
}

impl Data {
    /***
     * Returns an owned reference to UserData using clone. If you're simply modifying a field,
     * it is most likely better to use something else, like modifying the hashmap directly.
     */
    pub async fn get_owned_user_data(&self, user: serenity::UserId) -> Option<UserData> {
        let user_data = self.user_data.clone();
        let user_data_lock = user_data.read().await;

        user_data_lock
            .get(&user)
            .map(|user_data| user_data.to_owned())
    }

    pub async fn save_user_data(&self, path: Box<std::path::Path>) -> Result<(), std::io::Error> {
        let user_data = Arc::clone(&self.user_data);
        let user_data_lock = user_data.read().await;

        let content = serde_json::to_string(&*user_data_lock).unwrap();
        fs::write(path, content.as_bytes()).await
    }

    #[tracing::instrument(skip(self))]
    pub async fn import_user_data(&self, path: Box<std::path::Path>) -> Result<(), std::io::Error> {
        let data = fs::read(path);
        let lock = self.user_data.write();

        let (data, mut lock) = tokio::join!(data, lock);

        let data = data?;

        if !lock.is_empty() {
            warn!(
                "User data isn't empty, data may be destroyed in this process, continuing anyways..."
            );
        };

        let serialized = str::from_utf8(&data).expect("User data invalid, not in UTF-8 format! Stopping to prevent further corruption, please inspect the file itself.");

        let deserialized: HashMap<UserId, UserData> = serde_json::from_str(serialized).unwrap();

        let user_data = &mut lock;

        **user_data = deserialized;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildData {
    pub guild_channel_role: Vec<(serenity::ChannelId, GuildChannelRole)>,
    pub timezones: Vec<Tz>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GuildChannelRole {
    TimestampChannel,
    None,
}
