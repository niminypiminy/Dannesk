// btc import_wallet.rs (updated to handle JSON creation and channel update on response)

use serde_json::{json, Value};
use tokio_tungstenite::tungstenite::Message;
use crate::ws::connection::ConnectionManager;
use crate::channel::{CHANNEL, WSCommand, ProgressState};
use crate::utils::json_storage; // ADDED: For json write

pub async fn execute(
    connection: &mut ConnectionManager,
    bitcoin_current_wallet: &mut String,
    cmd: WSCommand,
) -> Result<(), String> {
    static FAILED: &str = "Error: Bitcoin wallet import failed";
    if let Some(wallet) = cmd.wallet.clone() {
        *bitcoin_current_wallet = wallet.clone();
        let msg_json = json!({"command": "import_bitcoin_wallet", "wallet": wallet});
        connection
            .send(Message::Text(msg_json.to_string()))
            .await
            .map_err(|e| {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: format!("{}: {}", FAILED, e),
                }));
                format!("{}: {}", FAILED, e)
            })?;
        Ok(())
    } else {
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: FAILED.to_string(),
        }));
        Err(FAILED.to_string())
    }
}

pub async fn process_response(message: Message, bitcoin_current_wallet: &str) -> Result<(), String> {
    static FAILED: &str = "Error: Bitcoin wallet import failed";
    match message {
        Message::Text(text) => {
            let data: Value = serde_json::from_str(&text).map_err(|e| {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: format!("{}: Failed to parse JSON: {}", FAILED, e),
                }));
                format!("Failed to parse JSON: {}", e)
            })?;

            if let Some(wallet) = data.get("wallet").and_then(|w| w.as_str()) {
                if wallet != bitcoin_current_wallet {
                    return Ok(());
                }

                // ADDED: Create JSON on confirmed response (using wallet from backend)
                let wallet_data = json!({
                    "address": wallet,
                    "private_key_deleted": false
                });
                json_storage::write_json("btc.json", &wallet_data)
                    .map_err(|e| {
                        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                            progress: 1.0,
                            message: format!("{}: Failed to write btc.json: {}", FAILED, e),
                        }));
                        format!("{}: Failed to write btc.json: {}", FAILED, e)
                    })?;

                // Bitcoin-specific fields from NOWNodes response
                let balance_btc = data
                    .get("balance")
                    .and_then(|b| b.as_str())
                    .and_then(|b| b.parse::<f64>().ok())
                    .map(|b| b / 100_000_000.0) // Convert satoshis to BTC
                    .unwrap_or(0.0);

                // Send balance updates to bitcoin_wallet_tx
                CHANNEL
                    .bitcoin_wallet_tx
                    .send((balance_btc, Some(wallet.to_string()), false))
                    .map_err(|e| {
                        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                            progress: 1.0,
                            message: format!("{}: Failed to send Bitcoin balance: {}", FAILED, e),
                        }));
                        format!("Failed to send Bitcoin balance: {}", e)
                    })?;

                // Send completion signal
                CHANNEL
                    .progress_tx
                    .send(Some(ProgressState {
                        progress: 1.0,
                        message: "Bitcoin wallet imported successfully".to_string(),
                    }))
                    .map_err(|e| format!("Failed to send progress: {}", e))?;
            } else {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: format!("{}: No wallet field in message", FAILED),
                }));
                return Err("No wallet field in message".to_string());
            }
        }
        _ => {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: format!("{}: Invalid message type", FAILED),
            }));
            return Err("Invalid message type".to_string());
        }
    }
    Ok(())
}