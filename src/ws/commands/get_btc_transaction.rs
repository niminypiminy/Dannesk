use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;
use crate::channel::{CHANNEL, BitcoinTransactionStatus, BTCTransactionState, BTCTransactionData};

pub async fn execute(
    _connection: &mut crate::ws::connection::ConnectionManager,
    _bitcoin_current_wallet: &mut String,
    _cmd: crate::channel::WSCommand,
) -> Result<(), String> {
    Ok(())
}

pub async fn process_response(message: Message, bitcoin_current_wallet: &str) -> Result<(), String> {
    match message {
        Message::Text(text) => {
            let data: Value = serde_json::from_str(&text)
                .map_err(|e| format!("Failed to parse JSON: {}", e))?;

            if let Some(wallet) = data.get("wallet").and_then(|w| w.as_str()) {
                if wallet != bitcoin_current_wallet {
                    return Ok(());
                }

                let command = data.get("command").and_then(|c| c.as_str());
                if command != Some("get_bitcoin_transaction") {
                    return Ok(());
                }

                let transactions_data = if let Some(tx) = data.get("transaction") {
                    if tx.is_null() {
                        Vec::new()
                    } else {
                        let txid = tx.get("txid").and_then(|h| h.as_str()).unwrap_or_default().to_string();
                        let status = match tx.get("status").and_then(|s| s.as_str()) {
                            Some("pending") => BitcoinTransactionStatus::Pending,
                            Some("confirmed") => BitcoinTransactionStatus::Success,
                            Some("failed") => BitcoinTransactionStatus::Failed,
                            Some("cancelled") => BitcoinTransactionStatus::Cancelled,
                            _ => {
                                return Err("Invalid transaction status".to_string());
                            }
                        };
                        let amount = tx.get("amount").and_then(|a| a.as_str()).unwrap_or("0").to_string();
                        let fees = tx.get("fees").and_then(|f| f.as_str()).unwrap_or("0").to_string();
                        let sender_addresses = tx.get("sender_addresses")
                            .and_then(|s| s.as_array())
                            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect::<Vec<String>>())
                            .unwrap_or_default();
                        let receiver_addresses = tx.get("receiver_addresses")
                            .and_then(|r| r.as_array())
                            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect::<Vec<String>>())
                            .unwrap_or_default();
                        let timestamp = tx.get("timestamp").and_then(|t| t.as_str()).unwrap_or_default().to_string();

                        vec![BTCTransactionData {
                            txid,
                            status,
                            amount,
                            fees,
                            receiver_addresses,
                            sender_addresses,
                            timestamp,
                        }]
                    }
                } else {
                    return Err("Missing transaction field".to_string());
                };

                if !transactions_data.is_empty() {
                    let btc_transactions_rx = CHANNEL.btc_transactions_rx.clone();
                    let mut current_transactions = btc_transactions_rx.borrow().transactions.clone();
                    for tx_data in transactions_data {
                        current_transactions.insert(tx_data.txid.clone(), tx_data.clone());
                    }
                    CHANNEL
                        .btc_transactions_tx
                        .send(BTCTransactionState {
                            transactions: current_transactions.clone(),
                        })
                        .map_err(|e| format!("Failed to send transaction data: {}", e))?;
                }
            } else {
                return Err("Missing wallet field".to_string());
            }
        }
        _ => {
            return Err("Non-text message received".to_string());
        }
    }
    Ok(())
}