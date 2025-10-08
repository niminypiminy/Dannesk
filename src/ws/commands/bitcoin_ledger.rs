use serde_json::{json, Value};
use tokio_tungstenite::tungstenite::Message;
use crate::ws::connection::ConnectionManager;
use crate::channel::{CHANNEL, ProgressState};
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct UTXO {
    pub txid: String,
    pub vout: u32,
    pub amount: u64, // Amount in satoshis
}

pub async fn fetch_utxo_data(
    connection: &mut ConnectionManager,
    wallet_address: &str,
) -> Result<Vec<UTXO>, String> {
    static FAILED: &str = "Error: Unable to fetch UTXO";

    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
        progress: 0.2,
        message: "Fetching UTXO data".to_string(),
    }));

    let msg_json = json!({
        "command": "get_bitcoin_utxo_data",
        "address": wallet_address
    });
    if connection.send(Message::Text(msg_json.to_string())).await.is_err() {
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: FAILED.to_string(),
        }));
        return Err(FAILED.to_string());
    }

    let start = Instant::now();
    while start.elapsed().as_secs() <= 8 {
        if let Some(message_result) = connection.next_message().await {
            match message_result {
                Ok(message) => {
                    match process_response(message, wallet_address).await {
                        Ok(Some(utxos)) => {
                            if utxos.is_empty() {
                                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                                    progress: 1.0,
                                    message: "Error: No UTXOs found".to_string(),
                                }));
                                return Err("No UTXOs found".to_string());
                            }
                            for utxo in &utxos {
                                if utxo.amount == 0 {
                                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                                        progress: 1.0,
                                        message: "Error: UTXO with zero amount".to_string(),
                                    }));
                                    return Err("UTXO with zero amount".to_string());
                                }
                                if utxo.txid.len() != 64 || !utxo.txid.chars().all(|c| c.is_ascii_hexdigit()) {
                                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                                        progress: 1.0,
                                        message: "Error: Invalid UTXO txid".to_string(),
                                    }));
                                    return Err(format!("Invalid UTXO txid: {}", utxo.txid));
                                }
                            }
                            return Ok(utxos);
                        }
                        Ok(None) => {
                            continue;
                        }
                        Err(e) => {
                            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                                progress: 1.0,
                                message: e.clone(),
                            }));
                            return Err(e);
                        }
                    }
                }
                Err(e) => {
                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                        progress: 1.0,
                        message: FAILED.to_string(),
                    }));
                    return Err(format!("WebSocket error: {:?}", e));
                }
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
        progress: 1.0,
        message: FAILED.to_string(),
    }));
    Err(FAILED.to_string())
}

pub async fn process_response(message: Message, _wallet_address: &str) -> Result<Option<Vec<UTXO>>, String> {
    static FAILED: &str = "Error: Failed to process UTXO data";
    match message {
        Message::Text(text) => {
            let data: Value = serde_json::from_str(&text).map_err(|e| {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: FAILED.to_string(),
                }));
                format!("JSON parsing error: {:?}", e)
            })?;

            if data.get("command").and_then(|c| c.as_str()) != Some("get_bitcoin_utxo_data") {
                return Ok(None);
            }

            if let Some(error) = data["utxo_info"]["error"].as_str() {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: error.to_string(),
                }));
                return Err(format!("Server error: {}", error));
            }

            let utxos: Vec<UTXO> = data["utxo_info"]["utxos"]
                .as_array()
                .ok_or_else(|| {
                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                        progress: 1.0,
                        message: FAILED.to_string(),
                    }));
                    FAILED.to_string()
                })?
                .iter()
                .filter_map(|utxo| {
                    let txid = match utxo["txid"].as_str() {
                        Some(s) => s.to_string(),
                        None => {
                            return None;
                        }
                    };
                    let vout = match utxo["vout"].as_u64() {
                        Some(n) => n as u32,
                        None => {
                            return None;
                        }
                    };
                    let amount = match utxo["value"].as_str().and_then(|s| s.parse::<u64>().ok()) {
                        Some(n) => n,
                        None => {
                            return None;
                        }
                    };
                    Some(UTXO { txid, vout, amount })
                })
                .collect();

            Ok(Some(utxos))
        }
        _ => {
            Ok(None)
        }
    }
}
