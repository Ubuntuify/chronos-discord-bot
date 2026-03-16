use std::sync::Arc;

use crate::database::data::{DatabaseError, ReadWriteData, UserSerdeHashMap};

impl ReadWriteData<'_, UserSerdeHashMap> for UserSerdeHashMap {
    async fn write_to(&self, path: Box<std::path::Path>) {
        write(&self, path).await.unwrap();
    }

    async fn read_from(&mut self, path: Box<std::path::Path>) {
        read_and_overwrite::<UserSerdeHashMap>(self, path).unwrap();
    }
}

fn read_and_overwrite<T: serde::de::DeserializeOwned + Clone>(
    data: &mut T,
    path: Box<std::path::Path>,
) -> Result<(), std::io::Error> {
    let file = Arc::new(std::fs::read_to_string(path)?); // should be fine to
    // use standard library since we're going to be only doing this once.

    let deserialized: T = serde_json::from_str(&file)?;
    *data = deserialized.clone();

    Ok(())
}

async fn write<T: serde::Serialize>(
    data: &T,
    path: Box<std::path::Path>,
) -> Result<(), DatabaseError> {
    let serialized = serde_json::to_string(data).map_err(|_| DatabaseError::WriteError)?;
    tokio::fs::write(path, serialized).await;

    Ok(())
}
