// src/ui/managebtc/btcbalance/wallet_operations.rs

use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::time::sleep;
use crate::utils::json_storage::{self, remove_json, get_config_path};
use crate::channel::{CHANNEL, WSCommand, ProgressState, BTCTransactionState};

pub struct BitcoinWalletOperations;

impl BitcoinWalletOperations {
    /// Deletes only the encrypted private key file (encrypt.json)
    pub async fn delete_key(wallet_address: String) {
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 0.0,
            message: "Starting key deletion...".to_string(),
        }));

        // Check if file exists before trying to delete
        let mut delete_success = true;
        if let Ok(path) = get_config_path("btc_encrypt.json") {
            if path.exists() {
                if let Err(e) = remove_json("btc_encrypt.json") {
                    delete_success = false;
                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                        progress: 0.3,
                        message: format!("Warning: Could not delete encrypted file: {}", e),
                    }));
                }
            } else {
                // Skip if already deleted
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 0.2,
                    message: "Key already removed from storage.".to_string(),
                }));
            }
        }

        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 0.5,
            message: "Updating wallet metadata...".to_string(),
        }));

        sleep(Duration::from_millis(500)).await;

        // Update btc.json to reflect the key is gone (metadata only)
        if json_storage::update_json("btc.json", |data: &mut serde_json::Value| {
            if let Some(obj) = data.as_object_mut() {
                obj.insert("private_key_deleted".to_string(), serde_json::Value::Bool(true));
            }
        }).is_err() {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: "Error: Failed to update Bitcoin metadata".to_string(),
            }));
            return;
        }

        // Update UI State
        let (current_balance, _, _) = *CHANNEL.bitcoin_wallet_rx.borrow();
        let _ = CHANNEL.bitcoin_wallet_tx.send((
            current_balance,
            Some(wallet_address),
            true, // key deleted
        ));

        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: if delete_success { "Key deletion complete".to_string() } else { "Deletion finished with errors".to_string() },
        }));
    }

    /// Fully removes the wallet — deletes encrypted key, removes metadata JSON, notifies backend
   /// Fully removes the wallet — deletes encrypted key, removes metadata JSON, notifies backend
    pub async fn remove_wallet(wallet_address: String, ws_tx: Sender<WSCommand>) {
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 0.0,
            message: "Removing Bitcoin wallet...".to_string(),
        }));

        // 1. Delete the encrypted sensitive data (if it exists)
        if let Ok(path) = get_config_path("btc_encrypt.json") {
            if path.exists() {
                let _ = remove_json("btc_encrypt.json");
            }
        }

        // 2. Delete the wallet metadata (btc.json)
        if let Ok(path) = get_config_path("btc.json") {
            if path.exists() {
                if let Err(e) = remove_json("btc.json") {
                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                        progress: 1.0,
                        message: format!("Error removing wallet file: {}", e),
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

        // 3. Notify backend - Explicitly defining all fields since Default is not implemented
        let command = WSCommand {
            command: "delete_bitcoin_wallet".to_string(),
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
        let cleared_txs = BTCTransactionState { transactions: HashMap::new() };
        let _ = CHANNEL.btc_transactions_tx.send(cleared_txs);
        let _ = CHANNEL.bitcoin_wallet_tx.send((0.0, None, false));

        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: "Bitcoin wallet removal complete".to_string(),
        }));
    }
}