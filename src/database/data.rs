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

    pub last_saved: Mutex<DateTime<Utc>>,
}

impl Data {
    pub fn new() -> Data {
        Data {
            path: internal::get_path(),
            user: RwLock::new(UserSerdeHashMap(HashMap::new())),
            last_saved: Mutex::new(Utc::now()),
        }
    }

    pub async fn import(&self) {}
}

pub trait ReadWriteData<'a, T: Serialize + Deserialize<'a>> {
    /// Writes to disk the struct, must implement trait serde::Serialize;
    async fn write_to(&self, path: Box<Path>);

    /// This is an inherently dangerous operation, this will overwrite data. You should only be
    /// running this trait when initializing the struct.
    ///
    /// Reads and overwrites data within itself, leaving previous data that was not saved to be
    /// erased. There is no merge functionality and should be considered unsafe during operation
    /// of the program.
    async fn read_from(&mut self, path: Box<Path>);
}

#[derive(Serialize, Deserialize)]
pub struct UserSerdeHashMap(HashMap<serenity::UserId, UserData>);

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct UserData {
    tz: Option<chrono_tz::Tz>,
}

#[derive(Serialize, Deserialize)]
pub struct GuildSerdeHashMap(HashMap<serenity::GuildId, UserData>);

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct GuildData {
    common_time_zones: Vec<Tz>,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelSerdeHashMap(HashMap<serenity::ChannelId, UserData>);

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ChannelData {
    is_time_channel: bool,
}
