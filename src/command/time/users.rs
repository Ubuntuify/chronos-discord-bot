use poise::serenity_prelude as serenity;

use crate::database::data::{Data, UserData, UserSerdeHashMap};

pub async fn find_user_tz(data: &Data, user: serenity::UserId) -> Option<chrono_tz::Tz> {
    let lock = data.user.read().await;
    let UserSerdeHashMap(data) = &*lock;
    data.get(&user).map(|d| d.tz)
}

pub async fn set_user_tz(
    data: &Data,
    user: serenity::UserId,
    tz: &chrono_tz::Tz,
) -> Result<(), crate::Error> {
    let mut lock = data.user.write().await;
    let UserSerdeHashMap(data) = &mut *lock;

    data.entry(user)
        .and_modify(|data| data.tz = tz.to_owned())
        .or_insert(UserData { tz: tz.to_owned() });

    Ok(())
}
