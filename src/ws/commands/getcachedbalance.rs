use serde_json::{json, Value};
use tokio_tungstenite::tungstenite::Message;
use crate::ws::connection::ConnectionManager;
use crate::channel::{CHANNEL, WSCommand, TransactionStatus, TransactionState, TransactionData};

pub async fn execute(
    connection: &mut ConnectionManager,
    _current_wallet: &mut String,
    cmd: WSCommand,
) -> Result<(), String> {
    if let Some(wallet) = &cmd.wallet {
        let msg_json = json!({ "command": "get_cached_balance", "wallet": wallet });
        connection.send(Message::Text(msg_json.to_string())).await?;
        Ok(())
    } else {
        Err("Missing wallet parameter".to_string())
    }
}

pub async fn process_response(message: Message, _current_wallet: &str) -> Result<(), String> {
    match message {
        Message::Text(text) => {
            let data: Value = serde_json::from_str(&text)
                .map_err(|e| format!("Failed to parse JSON: {}", e))?;

            if data.get("command").and_then(|c| c.as_str()) != Some("get_cached_balance") {
                return Ok(());
            }

            if let Some(error) = data.get("error").and_then(|e| e.as_str()) {
                return Err(format!("Server error: {}", error));
            }

            let wallet = data.get("wallet").and_then(|w| w.as_str()).ok_or_else(|| {
                "Missing wallet field".to_string()
            })?;

            let balance_xrp = data.get("balance")
                .and_then(|b| b.as_str())
                .and_then(|b| b.parse::<f64>().ok())
                .map(|b| b / 1_000_000.0)
                .unwrap_or(0.0);

            let rlusd_balance = data.get("rlusd_balance")
                .and_then(|r| r.as_str())
                .and_then(|r| r.parse::<f64>().ok())
                .unwrap_or(0.0);

            let euro_balance = data.get("euro_balance")
                .and_then(|e| e.as_str())
                .and_then(|e| e.parse::<f64>().ok())
                .unwrap_or(0.0);

            let has_rlusd = data.get("has_rlusd").and_then(|h| h.as_bool()).unwrap_or(false);
            let has_euro = data.get("has_euro").and_then(|h| h.as_bool()).unwrap_or(false);
            let trustline_limit = data.get("trustline_limit")
                .and_then(|l| l.as_str())
                .and_then(|l| l.parse::<f64>().ok());
            let trustline_euro_limit = data.get("trustline_euro_limit")
                .and_then(|l| l.as_str())
                .and_then(|l| l.parse::<f64>().ok());

            let transactions_data = if let Some(tx_value) = data.get("transaction") {
                if tx_value.is_null() {
                    Vec::new()
                } else if let Some(tx_array) = tx_value.as_array() {
                    tx_array.iter().filter_map(|tx| {
                        let tx_id = tx.get("hash").and_then(|h| h.as_str())?.to_string();
                        let status = match tx.get("status").and_then(|s| s.as_str()) {
                            Some("success") => TransactionStatus::Success,
                            Some("failed") => TransactionStatus::Failed,
                            Some("pending") => TransactionStatus::Pending,
                            Some("cancelled") => TransactionStatus::Cancelled,
                            _ => return None,
                        };
                        Some(TransactionData {
                            tx_id,
                            status,
                            execution_price: tx.get("price").and_then(|p| p.as_str()).unwrap_or("0").to_string(),
                            order_type: tx.get("tx_type").and_then(|t| t.as_str()).unwrap_or_default().to_string().to_lowercase(),
                            timestamp: tx.get("timestamp").and_then(|t| t.as_str()).unwrap_or_default().to_string(),
                            amount: tx.get("amount").and_then(|a| a.as_str()).unwrap_or("0").to_string(),
                            currency: tx.get("currency").and_then(|c| c.as_str()).unwrap_or_default().to_string(),
                            fee: tx.get("fee").and_then(|f| f.as_str()).unwrap_or_default().to_string(),
                            flags: tx.get("flags").and_then(|f| f.as_str()).map(|s| s.to_string()),
                            receiver: tx.get("receiver").and_then(|r| r.as_str()).unwrap_or_default().to_string(),
                            sender: tx.get("sender").and_then(|s| s.as_str()).unwrap_or_default().to_string(),
                        })
                    }).collect()
                } else {
                    return Err("Invalid transaction format".to_string());
                }
            } else {
                Vec::new()
            };

            let (_, _, private_key_deleted) = *CHANNEL.wallet_balance_rx.borrow();
            CHANNEL.wallet_balance_tx
                .send((balance_xrp, Some(wallet.to_string()), private_key_deleted))
                .map_err(|e| format!("Failed to send wallet balance: {}", e))?;

            CHANNEL.rlusd_tx
                .send((rlusd_balance, has_rlusd, trustline_limit))
                .map_err(|e| format!("Failed to send RLUSD balance: {}", e))?;

            CHANNEL.euro_tx
                .send((euro_balance, has_euro, trustline_euro_limit))
                .map_err(|e| format!("Failed to send Euro balance: {}", e))?;

            if !transactions_data.is_empty() {
                let mut current_transactions = CHANNEL.transactions_rx.borrow().transactions.clone();
                for tx_data in transactions_data {
                    current_transactions.insert(tx_data.tx_id.clone(), tx_data.clone());
                }
                CHANNEL.transactions_tx
                    .send(TransactionState { transactions: current_transactions })
                    .map_err(|e| format!("Failed to send transaction data: {}", e))?;
            }

            Ok(())
        }
        _ => {
            Err("Non-text message received".to_string())
        }
    }
}