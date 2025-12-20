use tracing::info;

pub use crate::structs::data::Data;

use std::path::Path;

pub fn get_data_path() -> Box<std::path::Path> {
    // TODO: add more code for finding other directories on other systems..
    // This will only work on Linux systems, and not all of them.

    let paths = std::env::var("XDG_DATA_HOME").unwrap();
    let path: &std::path::Path = Path::new(&paths);

    info!("Using data path of {}", &path.to_str().unwrap().to_string());

    path.into()
}

impl Data {
    pub async fn initialize_data(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}
