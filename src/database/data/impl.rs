use std::sync::Arc;

use poise::serenity_prelude::futures::TryFutureExt;

use crate::database::data::{DatabaseError, GuildSerdeHashMap, ReadWriteData, UserSerdeHashMap};

impl ReadWriteData<'_, UserSerdeHashMap> for UserSerdeHashMap {
    async fn write_to(&self, path: Box<std::path::Path>) -> Result<(), DatabaseError> {
        let path = path.join("users.json");
        write(&self, &path).await?;
        Ok(())
    }

    async fn read_from(&mut self, path: Box<std::path::Path>) -> Result<(), DatabaseError> {
        let path = path.join("users.json");
        read_and_overwrite::<UserSerdeHashMap>(self, &path)?;
        Ok(())
    }
}

impl ReadWriteData<'_, GuildSerdeHashMap> for GuildSerdeHashMap {
    async fn write_to(&self, path: Box<std::path::Path>) -> Result<(), DatabaseError> {
        let path = path.join("guilds.json");
        write(&self, &path).await?;
        Ok(())
    }

    async fn read_from(&mut self, path: Box<std::path::Path>) -> Result<(), DatabaseError> {
        let path = path.join("guilds.json");
        read_and_overwrite::<GuildSerdeHashMap>(self, &path)?;
        Ok(())
    }
}

fn read_and_overwrite<T: serde::de::DeserializeOwned + Clone>(
    data: &mut T,
    path: &std::path::Path,
) -> Result<(), DatabaseError> {
    let file = Arc::new(std::fs::read_to_string(path).map_err(|_| DatabaseError::ReadError)?); // should be fine to
    // use standard library since we're going to be only doing this once.

    let deserialized: T = serde_json::from_str(&file).map_err(|_| DatabaseError::SerializeError)?;
    *data = deserialized.clone();

    Ok(())
}

async fn write<T: serde::Serialize>(data: &T, path: &std::path::Path) -> Result<(), DatabaseError> {
    let serialized = serde_json::to_string(data).map_err(|_| DatabaseError::DeserializeError)?;
    tokio::fs::write(path, serialized)
        .map_err(|_| DatabaseError::WriteError)
        .await?;

    Ok(())
}
