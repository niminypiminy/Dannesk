use serde_json::{Value};
use tokio_tungstenite::tungstenite::Message;
use crate::channel::{CHANNEL, TransactionStatus, TransactionData};

pub async fn execute(
    _connection: &mut crate::ws::connection::ConnectionManager,
    _current_wallet: &mut String,
    _cmd: crate::channel::WSCommand,
) -> Result<(), String> {
    // No client-initiated execution for get_transaction
    Ok(())
}

pub async fn process_response(message: Message, current_wallet: &str) -> Result<(), String> {
    match message {
        Message::Text(text) => {
            let data: Value = serde_json::from_str(&text)
                .map_err(|e| format!("Failed to parse JSON: {}", e))?;

            if let Some(wallet) = data.get("wallet").and_then(|w| w.as_str()) {
                if wallet != current_wallet {
                    return Ok(());
                }

                let command = data.get("command").and_then(|c| c.as_str());
                if command != Some("get_transaction") {
                    return Ok(());
                }

                let transactions_data = if let Some(tx) = data.get("transaction") {
                    if tx.is_null() {
                        Vec::new()
                    } else {
                        let tx_id = tx.get("hash").and_then(|h| h.as_str()).unwrap_or_default().to_string();
                        let status = match tx.get("status").and_then(|s| s.as_str()) {
                            Some("success") => TransactionStatus::Success,
                            Some("failed") => TransactionStatus::Failed,
                            Some("pending") => TransactionStatus::Pending,
                            Some("cancelled") => TransactionStatus::Cancelled,
                            _ => {
                                return Err("Invalid transaction status".to_string());
                            }
                        };
                        let execution_price = tx.get("price")
                            .and_then(|p| p.as_str())
                            .unwrap_or("0")
                            .to_string();
                        let order_type = tx.get("tx_type")
                            .and_then(|t| t.as_str())
                            .unwrap_or_default()
                            .to_string()
                            .to_lowercase();
                        let timestamp = tx.get("timestamp")
                            .and_then(|t| t.as_str())
                            .unwrap_or_default()
                            .to_string();
                        let amount = tx.get("amount")
                            .and_then(|a| a.as_str())
                            .and_then(|a| a.parse::<f64>().ok())
                            .map(|amount| amount.to_string())
                            .unwrap_or("0.0".to_string());
                        let currency = tx.get("currency")
                            .and_then(|c| c.as_str())
                            .unwrap_or_default()
                            .to_string();
                        let fee = tx.get("fee")
                            .and_then(|f| f.as_str())
                            .unwrap_or_default()
                            .to_string();
                        let flags = tx.get("flags")
                            .and_then(|f| f.as_str())
                            .map(|s| s.to_string());
                        let receiver = tx.get("receiver")
                            .and_then(|r| r.as_str())
                            .unwrap_or_default()
                            .to_string();
                        let sender = tx.get("sender")
                            .and_then(|s| s.as_str())
                            .unwrap_or_default()
                            .to_string();

                        vec![TransactionData {
                            tx_id,
                            status,
                            execution_price,
                            order_type,
                            timestamp,
                            amount,
                            currency,
                            fee,
                            flags,
                            receiver,
                            sender,
                        }]
                    }
                } else {
                    return Err("Missing transaction field".to_string());
                };

                if !transactions_data.is_empty() {
                    let transactions_rx = CHANNEL.transactions_rx.clone();
                    let mut current_transactions = transactions_rx.borrow().transactions.clone();
                    for tx_data in transactions_data {
                        current_transactions.insert(tx_data.tx_id.clone(), tx_data.clone());
                    }
                    CHANNEL
                        .transactions_tx
                        .send(crate::channel::TransactionState {
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