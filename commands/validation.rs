// ws/commands/validation.rs
//This module handles input validation for tx_type, wallet, passphrase, and current_wallet.

use crate::channel::{CHANNEL, ProgressState, WSCommand};

pub fn validate_inputs(
    cmd: &WSCommand,
    current_wallet: &mut String,
) -> Result<(String, String, String), String> {
    static FAILED: &str = "Error: Transaction failed";

    // ... (tx_type and wallet logic remains same) ...
    let tx_type = match &cmd.tx_type {
        Some(tx_type) => tx_type.clone(),
        None => return Err(FAILED.to_string()), 
    };

    let wallet = match &cmd.wallet {
        Some(wallet) => wallet.clone(),
        None => return Err(FAILED.to_string()),
    };

    // Ensure either passphrase or seed is provided
    // .is_none() works the same on Option<Zeroizing<String>>
    if cmd.passphrase.is_none() && cmd.seed.is_none() {
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: "Error: Must provide either passphrase or seed".to_string(),
        }));
        return Err("Error: Must provide either passphrase or seed".to_string());
    }
    if !current_wallet.is_empty() && *current_wallet != wallet {
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: FAILED.to_string(),
        }));
        return Err(FAILED.to_string());
    }
    *current_wallet = wallet.clone();

    // Return a dummy passphrase since it's not used
    Ok((tx_type, wallet, String::new()))
}