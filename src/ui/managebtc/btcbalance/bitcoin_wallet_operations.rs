use keyring::Entry;
use serde_json;
use std::thread::sleep;
use std::time::Duration;
use crate::channel::{CHANNEL, WSCommand, ProgressState, BTCTransactionState};
use std::collections::HashMap;
use tokio::sync::mpsc;
use crate::utils::json_storage;

pub struct BitcoinWalletOperations;

impl BitcoinWalletOperations {
    pub fn delete_key(wallet_address: Option<String>) {
        static FAILED: &str = "Error: Failed to delete key";
        if let Some(address) = wallet_address {
            std::thread::spawn(move || {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 0.0,
                    message: "Starting key deletion".to_string(),
                }));

                // Delete key from keyring
                let entry = match Entry::new("bitcoin_wallet", &address) {
                    Ok(entry) => entry,
                    Err(_) => {
                        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                            progress: 1.0,
                            message: FAILED.to_string(),
                        }));
                        return;
                    }
                };
                if let Err(_) = entry.delete_credential() {
                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                        progress: 1.0,
                        message: FAILED.to_string(),
                    }));
                    return;
                }

                // Update progress
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 0.5,
                    message: "Key deleted, updating wallet data".to_string(),
                }));
                sleep(Duration::from_millis(500));

                // Update btc.json using json_storage::update_json
                if let Err(_) = json_storage::update_json("btc.json", |data: &mut serde_json::Value| {
                    if let Some(obj) = data.as_object_mut() {
                        obj.insert("private_key_deleted".to_string(), serde_json::Value::Bool(true));
                    }
                }) {
                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                        progress: 1.0,
                        message: FAILED.to_string(),
                    }));
                    return;
                }

                // Update wallet balance channel
                let (current_balance, _, _private_key_deleted) = *CHANNEL.bitcoin_wallet_rx.borrow();
                if let Err(_) = CHANNEL.bitcoin_wallet_tx.send((
                    current_balance,
                    Some(address),
                    true, // private_key_deleted set to true
                )) {
                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                        progress: 1.0,
                        message: FAILED.to_string(),
                    }));
                    return;
                }

                // Send completion update
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: "Key deletion complete".to_string(),
                }));
            });
        }
    }

    pub fn remove_wallet(wallet_address: Option<String>, commands_tx: mpsc::Sender<WSCommand>) {
        static FAILED: &str = "Error: Failed to remove Bitcoin wallet";
        if let Some(address) = wallet_address {
            std::thread::spawn(move || {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 0.0,
                    message: "Starting Bitcoin wallet removal".to_string(),
                }));

                // Delete key from keyring
                if let Ok(entry) = Entry::new("bitcoin_wallet", &address) {
                    if entry.get_password().is_ok() {
                        if let Err(_) = entry.delete_credential() {
                            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                                progress: 1.0,
                                message: FAILED.to_string(),
                            }));
                            return;
                        }
                    }
                }

                // Remove btc.json using json_storage::remove_json
                if let Err(_) = json_storage::remove_json("btc.json") {
                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                        progress: 0.0,
                        message: FAILED.to_string(),
                    }));
                    return;
                }

                // Update progress
                if let Err(_) = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 0.5,
                    message: "Bitcoin wallet file removed, sending delete command".to_string(),
                })) {
                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                        progress: 1.0,
                        message: FAILED.to_string(),
                    }));
                    return;
                }
                sleep(Duration::from_millis(500));

                // Send delete command
                let command = WSCommand {
                    command: "delete_bitcoin_wallet".to_string(),
                    wallet: Some(address.clone()),
                    recipient: None,
                    amount: None,
                    passphrase: None,
                    trustline_limit: None,
                    tx_type: None,
                    taker_pays: None,
                    taker_gets: None,
                    seed: None,
                    flags: None,
                    wallet_type: None,
                };
                if let Err(_) = commands_tx.try_send(command) {
                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                        progress: 1.0,
                        message: FAILED.to_string(),
                    }));
                    return;
                }

                // Clear the Bitcoin transaction hashmap
                if let Err(_) = CHANNEL.btc_transactions_tx.send(BTCTransactionState {
                    transactions: HashMap::new(),
                }) {
                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                        progress: 1.0,
                        message: FAILED.to_string(),
                    }));
                    return;
                }

                // Update Bitcoin wallet channel
                if let Err(_) = CHANNEL.bitcoin_wallet_tx.send((0.0, None, false)) {
                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                        progress: 1.0,
                        message: FAILED.to_string(),
                    }));
                    return;
                }

                // Send completion update
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: "Bitcoin wallet removal complete".to_string(),
                }));
            });
        }
    }
}