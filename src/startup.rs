use crate::channel::CHANNEL;
use reqwest::Client;
use serde::Deserialize;

// Define a struct to deserialize the version JSON
#[derive(Deserialize, Clone)]
struct VersionResponse {
    version: String,
}

pub fn init_startup(handle: &tokio::runtime::Handle) {
    let handle_clone = handle.clone();
    
    // Spawn the fetch so it doesn't block main()
    handle_clone.spawn(async move {
        // You might want to add a timeout here so it doesn't hang forever
        match tokio::time::timeout(std::time::Duration::from_secs(5), fetch_version()).await {
            Ok(Ok(data)) => {
                let _ = CHANNEL.version_tx.send(Some(data.version));
            }
            _ => {
                // If it times out or fails, we send None (Offline mode)
                let _ = CHANNEL.version_tx.send(None);
            }
        }
    });
}

// Fetch the version and URLs from the remote URL
async fn fetch_version() -> Result<VersionResponse, reqwest::Error> {
    let client = Client::new();
    let response = client
        .get("https://dannesk.com/version.json")
        .send()
        .await?;
    let version_data: VersionResponse = response.json().await?;
    Ok(version_data)
}