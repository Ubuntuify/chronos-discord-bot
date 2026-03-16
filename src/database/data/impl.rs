use crate::database::data::{ReadWriteData, UserSerdeHashMap};

impl ReadWriteData<'_, UserSerdeHashMap> for UserSerdeHashMap {
    async fn write_to(&self, path: Box<std::path::Path>) {}

    async fn read_from(&mut self, path: Box<std::path::Path>) {}
}

async fn read_serialized_generic<T: serde::Serialize>(data: &mut T, path: Box<std::path::Path>) {}
