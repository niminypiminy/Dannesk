use serde_json::{json, Value};
use tokio_tungstenite::tungstenite::Message;
use crate::ws::connection::ConnectionManager;
use crate::channel::{CHANNEL, WSCommand, ProgressState};
use crate::utils::json_storage;

// ====================== HELPER (add new assets here only) ======================
// Mirrors the same "one source of truth" philosophy from the backend

fn parse_asset(
    data: &Value,
    has_key: &str,
    balance_key: &str,
    limit_key: &str,
) -> (f64, bool, Option<f64>) {
    let has = data.get(has_key).and_then(|h| h.as_bool()).unwrap_or(false);
    let balance = data
        .get(balance_key)
        .and_then(|b| b.as_str())
        .and_then(|b| b.parse::<f64>().ok())
        .unwrap_or(0.0);
    let limit = data
        .get(limit_key)
        .and_then(|l| l.as_str())
        .and_then(|l| l.parse::<f64>().ok());

    (balance, has, limit)
}

// ====================== END OF HELPER ======================

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
            .send(Message::text(msg_json.to_string()))
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

                // Wallet JSON (unchanged)
                let wallet_data = json!({
                    "address": wallet,
                    "private_key_deleted": false
                });
                json_storage::write_json("xrp.json", &wallet_data)
                    .map_err(|e| format!("{}: Failed to write xrp.json: {}", FAILED, e))?;

                // XRP (unchanged)
                let balance_xrp = data
                    .get("balance")
                    .and_then(|b| b.as_str())
                    .and_then(|b| b.parse::<f64>().ok())
                    .map(|b| b / 1_000_000.0)
                    .unwrap_or(0.0);

                // === Centralized stablecoin parsing (no more duplication) ===
                let (rlusd_balance, has_rlusd, trustline_limit) =
                    parse_asset(&data, "has_rlusd", "rlusd_balance", "trustline_limit");

                let (euro_balance, has_euro, trustline_euro_limit) =
                    parse_asset(&data, "has_euro", "euro_balance", "trustline_euro_limit");

                let (xsgd_balance, has_xsgd, trustline_xsgd_limit) =
                    parse_asset(&data, "has_xsgd", "xsgd_balance", "trustline_xsgd_limit");

                // === Send updates (unchanged logic, just much cleaner) ===
                let (_, _, private_key_deleted) = *CHANNEL.wallet_balance_rx.borrow();
                CHANNEL
                    .wallet_balance_tx
                    .send((balance_xrp, Some(wallet.to_string()), private_key_deleted))
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

                CHANNEL
                    .sgd_tx
                    .send((xsgd_balance, has_xsgd, trustline_xsgd_limit))
                    .map_err(|e| {
                        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                            progress: 1.0,
                            message: format!("{}: Failed to send XSGD balance: {}", FAILED, e),
                        }));
                        format!("Failed to send XSGD balance: {}", e)
                    })?;

                // Completion signal (unchanged)
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