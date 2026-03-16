use std::{collections::HashMap, path::Path};

use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use poise::serenity_prelude as serenity;
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, RwLock};

mod r#impl;
mod internal;

pub struct Data {
    path: Box<Path>,
    pub user: RwLock<UserSerdeHashMap>,
    pub guild: RwLock<GuildSerdeHashMap>,
    pub last_saved: Mutex<DateTime<Utc>>,
}

impl Data {
    pub fn new() -> Data {
        Data {
            path: internal::get_path(),
            user: RwLock::new(UserSerdeHashMap(HashMap::new())),
            guild: RwLock::new(GuildSerdeHashMap(HashMap::new())),
            last_saved: Mutex::new(Utc::now()),
        }
    }

    pub async fn import(&self) {
        let path = &self.path;

        let mut guild_data = self.guild.write().await;
        (*guild_data).read_from(path.clone()).await;

        let mut user_data = self.guild.write().await;
        (*user_data).read_from(path.clone()).await;
    }
}

#[derive(Debug)]
pub enum DatabaseError {
    WriteError,
    DeserializeError,
    SerializeError,
    ReadError,
}

pub trait ReadWriteData<'a, T: Serialize + Deserialize<'a> + Clone> {
    /// Writes to disk the struct, must implement trait serde::Serialize;
    #[allow(async_fn_in_trait)]
    async fn write_to(&self, path: Box<Path>);

    /// This is an inherently dangerous operation, this will overwrite data. You should only be
    /// running this trait when initializing the struct.
    ///
    /// Reads and overwrites data within itself, leaving previous data that was not saved to be
    /// erased. There is no merge functionality and should be considered unsafe during operation
    /// of the program.
    #[allow(async_fn_in_trait)]
    async fn read_from(&mut self, path: Box<Path>);
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserSerdeHashMap(pub HashMap<serenity::UserId, UserData>);

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct UserData {
    pub tz: chrono_tz::Tz,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GuildSerdeHashMap(pub HashMap<serenity::GuildId, UserData>);

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct GuildData {
    common_time_zones: Vec<Tz>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChannelSerdeHashMap(pub HashMap<serenity::ChannelId, UserData>);

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ChannelData {
    is_time_channel: bool,
}
