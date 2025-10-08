use ring::rand::SystemRandom;
use ring::signature::{Ed25519KeyPair, KeyPair};
use std::io;
use crate::utils::json_storage;
use crate::channel::{StartupData, CHANNEL};
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

    let startup_data = match load_startup_data() {
        Ok(data) => {
            // Verify public key length and keypair validity
            if data.public_key.len() != 32 {
                let new_data = generate_startup_data();
                let _ = save_startup_data(&new_data);
                new_data
            } else {
                match Ed25519KeyPair::from_pkcs8(&data.private_key) {
                    Ok(kp) => {
                        if kp.public_key().as_ref() == data.public_key {
                            data
                        } else {
                            let new_data = generate_startup_data();
                            let _ = save_startup_data(&new_data);
                            new_data
                        }
                    }
                    Err(_) => {
                        let new_data = generate_startup_data();
                        let _ = save_startup_data(&new_data);
                        new_data
                    }
                }
            }
        }
        Err(_) => {
            let data = generate_startup_data();
            let _ = save_startup_data(&data);
            data
        }
    };

    // Send startup data to global channel
    let _ = CHANNEL.startup_tx.send(Some(startup_data));
}

// Fetch the version and URLs from the remote URL
async fn fetch_version() -> Result<VersionResponse, reqwest::Error> {
    let client = Client::new();
    let response = client
        .get("https://dannesk.pages.dev/version.json")
        .send()
        .await?;
    let version_data: VersionResponse = response.json().await?;
    Ok(version_data)
}

// Generate new keypair
fn generate_startup_data() -> StartupData {
    let rng = SystemRandom::new();
    let pkcs8_bytes = match Ed25519KeyPair::generate_pkcs8(&rng) {
        Ok(bytes) => bytes,
        Err(_) => panic!("Keypair generation failed"),
    };
    let keypair = match Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()) {
        Ok(kp) => kp,
        Err(_) => panic!("Keypair parsing failed"),
    };

    let private_key = pkcs8_bytes.as_ref().to_vec();
    let public_key = keypair.public_key().as_ref().to_vec();

    // Verify keypair
    if public_key.len() != 32 {
        panic!("Invalid public key length");
    }
    match Ed25519KeyPair::from_pkcs8(&private_key) {
        Ok(kp) => {
            if kp.public_key().as_ref() != public_key {
                panic!("Generated keypair public key mismatch");
            }
        }
        Err(_) => panic!("Generated invalid keypair"),
    }

    StartupData {
        private_key,
        public_key,
    }
}

// Load startup data from startup.json
fn load_startup_data() -> io::Result<StartupData> {
    let data = json_storage::read_json("startup.json")?;
    Ok(data)
}

// Save startup data to startup.json
fn save_startup_data(data: &StartupData) -> io::Result<()> {
    json_storage::write_json("startup.json", data)?;
    Ok(())
}