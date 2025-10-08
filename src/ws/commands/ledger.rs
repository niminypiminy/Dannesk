//ws/commands/ledger.rs
//this module fetches fee from xrp ledger
use serde_json::{json, Value};
use tokio_tungstenite::tungstenite::Message;
use crate::ws::connection::ConnectionManager;
use crate::channel::{CHANNEL, ProgressState};
use std::time::Instant;

pub async fn fetch_ledger_data(
    connection: &mut ConnectionManager,
    wallet_address: &str,
) -> Result<(u32, String), String> {
    static FAILED: &str = "Error: Unable to fetch fee or sequence";
    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
        progress: 0.2,
        message: "Fetching ledger data".to_string(),
    }));

    let msg_json = json!({
        "command": "get_ledger_data",
        "account": wallet_address
    });
    if connection.send(Message::Text(msg_json.to_string())).await.is_err() {
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0, // Changed from 0.2
            message: FAILED.to_string(),
        }));
        return Err(FAILED.to_string());
    }

    let start = Instant::now();
    while start.elapsed().as_secs() <= 8 {
        if let Some(Ok(message)) = connection.next_message().await {
            match process_response(message, wallet_address).await {
                Ok(Some((sequence, fee))) => return Ok((sequence, fee)),
                Ok(None) => continue,
                Err(_) => {
                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                        progress: 1.0, // Changed from 0.2
                        message: FAILED.to_string(),
                    }));
                    return Err(FAILED.to_string());
                }
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
        progress: 1.0, // Changed from 0.2
        message: FAILED.to_string(),
    }));
    Err(FAILED.to_string())
}

pub async fn process_response(message: Message, current_wallet: &str) -> Result<Option<(u32, String)>, String> {
    static FAILED: &str = "Error: Failed to process ledger data";
    match message {
        Message::Text(text) => {
            let data: Value = serde_json::from_str(&text)
                .map_err(|_| {
                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                        progress: 1.0,
                        message: FAILED.to_string(),
                    }));
                    FAILED.to_string()
                })?;

            if data.get("command").and_then(|c| c.as_str()) != Some("get_ledger_data") {
                return Ok(None);
            }

            let account = data["account"].as_str().ok_or_else(|| {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: FAILED.to_string(),
                }));
                FAILED.to_string()
            })?;
            if account != current_wallet {
                return Ok(None);
            }

            if data.get("error").and_then(|e| e.as_str()).is_some() {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: FAILED.to_string(),
                }));
                return Err(FAILED.to_string());
            }

            let fee = data["fee"].as_str().ok_or_else(|| {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: FAILED.to_string(),
                }));
                FAILED.to_string()
            })?.to_string();
            let sequence_str = data["sequence"].as_str().ok_or_else(|| {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: FAILED.to_string(),
                }));
                FAILED.to_string()
            })?;
            let sequence = sequence_str.parse::<u64>().map_err(|_| {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: FAILED.to_string(),
                }));
                FAILED.to_string()
            })?;
            let sequence: u32 = sequence.try_into().map_err(|_| {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: FAILED.to_string(),
                }));
                FAILED.to_string()
            })?;

            Ok(Some((sequence, fee)))
        }
        _ => Ok(None),
    }
}