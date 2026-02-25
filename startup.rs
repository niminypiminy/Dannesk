use crate::channel::CHANNEL;
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Clone)]
struct VersionResponse {
    version: String,
}

/// One-time global initializations (Crypto, Environment Variables)
pub fn init_globals() {
    // 1. Initialize Crypto Provider
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    // 2. Set WGPU Backends based on OS
    #[cfg(target_os = "macos")]
    unsafe { std::env::set_var("WGPU_BACKEND", "metal"); }

    #[cfg(any(target_os = "windows", target_os = "linux"))]
    unsafe { std::env::set_var("WGPU_BACKEND", "vulkan"); }
}

/// Background tasks that require a running Tokio handle
pub fn init_startup(handle: &tokio::runtime::Handle) {
    let handle_clone = handle.clone();
    
    handle_clone.spawn(async move {
        match tokio::time::timeout(Duration::from_secs(5), fetch_version()).await {
            Ok(Ok(data)) => {
                let _ = CHANNEL.version_tx.send(Some(data.version));
            }
            _ => {
                let _ = CHANNEL.version_tx.send(None);
            }
        }
    });
}

async fn fetch_version() -> Result<VersionResponse, reqwest::Error> {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;
        
    let response = client
        .get("https://dannesk.com/version.json")
        .send()
        .await?;
    let version_data: VersionResponse = response.json().await?;
    Ok(version_data)
}