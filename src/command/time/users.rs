use poise::serenity_prelude as serenity;
use tracing::info;

use crate::database::data::{Data, UserData, UserSerdeHashMap};

#[tracing::instrument(skip(data, user))]
pub async fn find_user_tz(data: &Data, user: serenity::UserId) -> Option<chrono_tz::Tz> {
    let _ = data.check_for_save().await;

    let lock = data.user.read().await;
    let UserSerdeHashMap(data) = &*lock;

    info!("Read timezone data for user {}", user);

    data.get(&user).map(|d| d.tz)
}

#[tracing::instrument(skip(data, user))]
pub async fn set_user_tz(
    data: &Data,
    user: serenity::UserId,
    tz: &chrono_tz::Tz,
) -> Result<(), crate::Error> {
    let _ = data.check_for_save().await;

    let mut lock = data.user.write().await;
    let UserSerdeHashMap(data) = &mut *lock;

    data.entry(user)
        .and_modify(|data| data.tz = tz.to_owned())
        .or_insert(UserData { tz: tz.to_owned() });

    info!("Modified existing timezone data for user {}", user);

    Ok(())
}
