use crate::channel::CHANNEL;
use reqwest::Client;
use serde::Deserialize;

// Define a struct to deserialize the version JSON
#[derive(Deserialize, Clone)]
struct VersionResponse {
    version: String,
    windows_url: String,
    macos_url: String,
    linux_url: String,
}

// Initialize startup data and populate global channel
pub fn init_startup() {
    // Fetch the remote version and URLs
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    let remote_version_data = runtime.block_on(fetch_version());

    // Extract version and select URL based on OS
    let (version, url) = match remote_version_data {
        Ok(data) => {
            let selected_url = match std::env::consts::OS {
                "windows" => Some(data.windows_url),
                "macos" => Some(data.macos_url),
                "linux" => Some(data.linux_url),
                _ => None, // No URL for unknown OS
            };
            (Some(data.version), selected_url)
        }
        Err(_) => (None, None), // On error, send None to both channels
    };

    // Send version and URL to their respective channels
    let _ = CHANNEL.version_tx.send(version);
    let _ = CHANNEL.update_url_tx.send(url);

}

// Fetch the version and URLs from the remote URL
async fn fetch_version() -> Result<VersionResponse, reqwest::Error> {
    let client = Client::new();
    let response = client
        .get("")
        .send()
        .await?;
    let version_data: VersionResponse = response.json().await?;
    Ok(version_data)
}