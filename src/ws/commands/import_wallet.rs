use serde_json::{json, Value};
use tokio_tungstenite::tungstenite::Message;
use crate::ws::connection::ConnectionManager;
use crate::channel::{CHANNEL, WSCommand, ProgressState};

pub async fn execute(
    connection: &mut ConnectionManager,
    current_wallet: &mut String,
    cmd: WSCommand,
) -> Result<(), String> {
    static FAILED: &str = "Error: Wallet import failed";
    if let Some(wallet) = cmd.wallet.clone() {
        *current_wallet = wallet.clone();
        let msg_json = json!({"command": "import_wallet", "wallet": wallet});
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

pub async fn process_response(message: Message, current_wallet: &str) -> Result<(), String> {
    static FAILED: &str = "Error: Wallet import failed";
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
                if wallet != current_wallet {
                    return Ok(());
                }

                let balance_xrp = data
                    .get("balance")
                    .and_then(|b| b.as_str())
                    .and_then(|b| b.parse::<f64>().ok())
                    .map(|b| b / 1_000_000.0)
                    .unwrap_or(0.0);
                let is_active = data.get("xrp_active").and_then(|a| a.as_bool()).unwrap_or(false);
                let has_rlusd = data.get("has_rlusd").and_then(|h| h.as_bool()).unwrap_or(false);
                let rlusd_balance = data
                    .get("rlusd_balance")
                    .and_then(|b| b.as_str())
                    .and_then(|b| b.parse::<f64>().ok())
                    .unwrap_or(0.0);
                let trustline_limit = data
                    .get("trustline_limit")
                    .and_then(|l| l.as_str())
                    .and_then(|l| l.parse::<f64>().ok());
                let has_euro = data.get("has_euro").and_then(|h| h.as_bool()).unwrap_or(false);
                let euro_balance = data
                    .get("euro_balance")
                    .and_then(|b| b.as_str())
                    .and_then(|b| b.parse::<f64>().ok())
                    .unwrap_or(0.0);
                let trustline_euro_limit = data
                    .get("trustline_euro_limit")
                    .and_then(|l| l.as_str())
                    .and_then(|l| l.parse::<f64>().ok());

                // Send balance updates
                let (_, _, _, private_key_deleted) = *CHANNEL.wallet_balance_rx.borrow();
                CHANNEL
                    .wallet_balance_tx
                    .send((balance_xrp, Some(wallet.to_string()), is_active, private_key_deleted))
                    .map_err(|e| {
                        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                            progress: 1.0,
                            message: format!("{}: Failed to send balance: {}", FAILED, e),
                        }));
                        format!("Failed to send balance: {}", e)
                    })?;

                CHANNEL
                    .rlusd_tx
                    .send((rlusd_balance, has_rlusd, trustline_limit))
                    .map_err(|e| {
                        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                            progress: 1.0,
                            message: format!("{}: Failed to send RLUSD balance: {}", FAILED, e),
                        }));
                        format!("Failed to send RLUSD balance: {}", e)
                    })?;

                CHANNEL
                    .euro_tx
                    .send((euro_balance, has_euro, trustline_euro_limit))
                    .map_err(|e| {
                        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                            progress: 1.0,
                            message: format!("{}: Failed to send Euro balance: {}", FAILED, e),
                        }));
                        format!("Failed to send Euro balance: {}", e)
                    })?;

                // Send completion signal
                CHANNEL
                    .progress_tx
                    .send(Some(ProgressState {
                        progress: 1.0,
                        message: "Wallet imported successfully".to_string(),
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