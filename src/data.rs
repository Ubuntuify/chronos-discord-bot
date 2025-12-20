use tracing::{info, warn};

use tokio::io::AsyncWriteExt;

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
    #[tracing::instrument(skip(self))]
    pub async fn initialize_data(&self) -> Result<(), std::io::Error> {
        let path = &self.data_path.to_owned();

        const EMPTY_JSON: &[u8] = "{}".as_bytes();

        let user_data_path = path.join("user_data.json");

        if !user_data_path.exists() {
            warn!(
                "User data file does not exist, creating one now; if one was provided, please make sure the following path exists. {}",
                user_data_path.to_str().unwrap().to_string()
            );

            let mut file = tokio::fs::File::create_new(&user_data_path).await?;
            let _ = file.write(EMPTY_JSON).await;
        };

        self.import_user_data(user_data_path.into()).await

        // TODO: add guild data later
    }
}
