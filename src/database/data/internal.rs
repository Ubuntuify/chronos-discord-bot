use std::path::Path;
use tracing::info;

pub fn get_path() -> Box<Path> {
    if cfg!(feature = "docker") {
        Path::new("/data").into()
    } else {
        let path = std::env::var("BOT_DATA_PATH").unwrap();
        info!("Using data path: {:?}", &path);
        Path::new(&path).into()
    }
}
