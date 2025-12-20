use tokio::io::AsyncWriteExt;
use tracing::warn;

#[tracing::instrument(skip(data))]
pub async fn load_data(data: &crate::Data) -> Result<(), std::io::Error> {
    let path = &data.data_path.to_owned();

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

    data.import_user_data(user_data_path.into()).await

    // TODO: add guild data later
}
