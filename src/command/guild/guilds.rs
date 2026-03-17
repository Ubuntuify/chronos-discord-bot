use crate::database::data::{Data, DatabaseError, GuildData, GuildSerdeHashMap};
use poise::serenity_prelude as serenity;

pub async fn find_guild_common_tzs(
    data: &Data,
    guild: serenity::GuildId,
) -> Option<Vec<chrono_tz::Tz>> {
    let _ = data.check_for_save().await;

    let lock = data.guild.read().await;
    let GuildSerdeHashMap(data) = &*lock;

    data.get(&guild).map(|d| d.common_time_zones.clone())
}

pub async fn add_guild_common_tz(
    data: &Data,
    guild: serenity::GuildId,
    tz: chrono_tz::Tz,
) -> Result<(), crate::Error> {
    let _ = data.check_for_save().await;

    let mut lock = data.guild.write().await;
    let GuildSerdeHashMap(data) = &mut *lock;

    data.entry(guild)
        .and_modify(|f| f.common_time_zones.push(tz))
        .or_insert(GuildData {
            common_time_zones: vec![tz.to_owned()],
        });

    Ok(())
}
