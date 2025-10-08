// ws/commands/wallet_auth.rs

use crate::channel::{CHANNEL, ProgressState};
use crate::decrypt::decrypt_data;
use keyring::Entry;
use xrpl::wallet::Wallet;
use zeroize::Zeroize;

pub fn authenticate_wallet(passphrase: Option<String>, seed: Option<String>, wallet_address: &str) -> Result<Wallet, String> {
    let final_seed = match (passphrase, seed) {
        (None, Some(s)) => s, // Use seed if provided
        (Some(p), None) => {
            let mut input = p;
            let entry = Entry::new("rust_wallet", wallet_address).map_err(|_| {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: "Error: Keyring entry not found".to_string(),
                }));
                "Error: Keyring entry not found".to_string()
            })?;
            let encrypted_data = entry.get_password().map_err(|_| {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: "Error: Keyring entry not found".to_string(),
                }));
                "Error: Keyring entry not found".to_string()
            })?;
            let (encrypted, salt, iv) = serde_json::from_str::<(String, String, String)>(&encrypted_data)
                .map_err(|_| {
                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                        progress: 1.0,
                        message: "Error: Invalid keyring data".to_string(),
                    }));
                    "Error: Invalid keyring data".to_string()
                })?;
            let decrypted_seed = decrypt_data(input.clone(), encrypted, salt, iv).map_err(|_| {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: "Error: Decryption failed".to_string(),
                }));
                "Error: Decryption failed".to_string()
            })?;
            input.zeroize();
            decrypted_seed
        }
        _ => {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: "Error: Must provide either passphrase or seed".to_string(),
            }));
            return Err("Error: Must provide either passphrase or seed".to_string());
        }
    };

    Wallet::new(&final_seed, 0).map_err(|_| {
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: "Error: Invalid seed".to_string(),
        }));
        "Error: Invalid seed".to_string()
    })
}