use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::time::sleep;
use crate::utils::json_storage::{self, remove_json, get_config_path};
use crate::channel::{CHANNEL, WSCommand, ProgressState, TransactionState};

pub struct WalletOperations;

impl WalletOperations {
    /// Deletes only the encrypted private key file (xrp_encrypt.json)
    pub async fn delete_key(wallet_address: String) {
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 0.0,
            message: "Starting XRP key deletion...".to_string(),
        }));

        let mut delete_success = true;

        // 1. Target the XRP-specific encryption file
        if let Ok(path) = get_config_path("xrp_encrypt.json") {
            if path.exists() {
                if let Err(e) = remove_json("xrp_encrypt.json") {
                    delete_success = false;
                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                        progress: 0.3,
                        message: format!("Warning: Could not delete encrypted file: {}", e),
                    }));
                }
            } else {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 0.2,
                    message: "XRP key already removed from storage.".to_string(),
                }));
            }
        }

        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 0.5,
            message: "Updating XRP wallet metadata...".to_string(),
        }));

        sleep(Duration::from_millis(500)).await;

        // 2. Update xrp.json to reflect the key is gone
        if json_storage::update_json("xrp.json", |data: &mut serde_json::Value| {
            if let Some(obj) = data.as_object_mut() {
                obj.insert("private_key_deleted".to_string(), serde_json::Value::Bool(true));
            }
        }).is_err() {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: "Error: Failed to update XRP metadata".to_string(),
            }));
            return;
        }

        // 3. Update UI State
        let (current_balance, _, _) = *CHANNEL.wallet_balance_rx.borrow();
        let _ = CHANNEL.wallet_balance_tx.send((
            current_balance,
            Some(wallet_address),
            true, // key deleted
        ));

        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: if delete_success { "XRP Key deletion complete".to_string() } else { "Deletion finished with errors".to_string() },
        }));
    }

    /// Fully removes the wallet — deletes encrypted key, removes metadata JSON, notifies backend
    pub async fn remove_wallet(wallet_address: String, ws_tx: Sender<WSCommand>) {
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 0.0,
            message: "Starting XRP wallet removal...".to_string(),
        }));

        // 1. Delete the encrypted sensitive data
        if let Ok(path) = get_config_path("xrp_encrypt.json") {
            if path.exists() {
                let _ = remove_json("xrp_encrypt.json");
            }
        }

        // 2. Delete the wallet metadata (xrp.json)
        if let Ok(path) = get_config_path("xrp.json") {
            if path.exists() {
                if let Err(e) = remove_json("xrp.json") {
                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                        progress: 1.0,
                        message: format!("Error removing XRP wallet file: {}", e),
                    }));
                    return;
                }
            }
        }

        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 0.5,
            message: "Local files removed, notifying backend...".to_string(),
        }));

        sleep(Duration::from_millis(500)).await;

        // 3. Notify backend - WSCommand struct manual initialization
        let command = WSCommand {
            command: "delete_wallet".to_string(),
            wallet: Some(wallet_address.clone()),
            recipient: None,
            amount: None,
            passphrase: None,
            trustline_limit: None,
            fee: None,
            tx_type: None,
            taker_pays: None,
            taker_gets: None,
            seed: None,
            flags: None,
            wallet_type: None,
            bip39: None,
        };

        let _ = ws_tx.try_send(command);

        // 4. Reset UI/Channels
        let cleared = TransactionState { transactions: HashMap::new() };
        let _ = CHANNEL.transactions_tx.send(cleared);
        let _ = CHANNEL.wallet_balance_tx.send((0.0, None, false));
        let _ = CHANNEL.rlusd_tx.send((0.0, false, None));
        let _ = CHANNEL.euro_tx.send((0.0, false, None));

        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: "XRP wallet removal complete".to_string(),
        }));
    }
}