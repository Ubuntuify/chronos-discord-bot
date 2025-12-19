pub trait Load<T> {
    async fn load_from(path: std::path::Path) -> Result<T, std::io::Error>;
}

pub trait Save {
    async fn save_to(path: std::path::Path) -> Result<(), std::io::Error>;
}
