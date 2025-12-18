use std::collections::HashMap;
use std::sync::Arc;

use poise::serenity_prelude::{self as serenity, UserId};
use tokio::sync::RwLock;

pub struct Data {
    pub bot_id: serenity::UserId,
    pub user_data: Arc<RwLock<HashMap<serenity::UserId, crate::structs::user_data::UserData>>>,
}

impl Data {
    fn get_user_data(&self, user_id: UserId) {}
}
