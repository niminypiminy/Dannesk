use serde_json::{json, Value};
use tokio_tungstenite::tungstenite::Message;
use crate::ws::connection::ConnectionManager;
use crate::channel::{CHANNEL, WSCommand, BTCTransactionState, BTCTransactionData, BitcoinTransactionStatus};
use std::collections::HashMap;

pub async fn execute(
    connection: &mut ConnectionManager,
    bitcoin_current_wallet: &mut String,
    cmd: WSCommand,
) -> Result<(), String> {
    if let Some(wallet) = cmd.wallet {
        *bitcoin_current_wallet = wallet.clone();
        let msg_json = json!({ "command": "get_bitcoin_cached_balance", "wallet": wallet });
        if let Err(e) = connection.send(Message::Text(msg_json.to_string())).await {
            return Err(format!("Failed to send command: {}", e));
        }
        Ok(())
    } else {
        Err("Missing wallet parameter".to_string())
    }
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
                if command != Some("get_bitcoin_cached_balance") {
                    return Ok(());
                }

                let bitcoin_wallet_rx = CHANNEL.bitcoin_wallet_rx.clone();
                let (_current_balance, _wallet_opt, private_key_deleted) = bitcoin_wallet_rx.borrow().clone();

                // Process balance
                let balance_btc = if let Some(balance) = data.get("balance").and_then(|b| b.as_str()) {
                    if balance == "0" && data.get("balance").is_none() {
                        0.0
                    } else if let Ok(balance) = balance.parse::<f64>() {
                        balance / 100_000_000.0 // Convert satoshis to BTC
                    } else {
                        return Err(format!("Invalid balance format: {}", balance));
                    }
                } else {
                    return Err("Missing balance field".to_string());
                };

                // Process transactions
                let mut transactions_map = HashMap::new();
                let _transaction_count = if let Some(transactions) = data.get("transaction").and_then(|t| t.as_array()) {
                    for tx in transactions {
                        let txid = tx.get("txid").and_then(|h| h.as_str()).map(|s| s.to_string());
                        if txid.is_none() {
                            continue;
                        }
                        let txid = txid.unwrap();
                        let status = match tx.get("status").and_then(|s| s.as_str()) {
                            Some("pending") => BitcoinTransactionStatus::Pending,
                            Some("confirmed") => BitcoinTransactionStatus::Success, // Map "confirmed" to Success
                            Some("success") => BitcoinTransactionStatus::Success,
                            Some("failed") => BitcoinTransactionStatus::Failed,
                            Some("cancelled") => BitcoinTransactionStatus::Cancelled,
                            _ => {
                                continue;
                            }
                        };
                        let tx_data = BTCTransactionData {
                            txid,
                            status,
                            amount: tx.get("amount").and_then(|a| a.as_str()).unwrap_or("0").to_string(),
                            fees: tx.get("fees").and_then(|f| f.as_str()).unwrap_or("0").to_string(),
                            receiver_addresses: tx.get("receiver_addresses")
                                .and_then(|r| r.as_array())
                                .map(|arr| arr.iter().filter_map(|a| a.as_str().map(|s| s.to_string())).collect())
                                .unwrap_or_default(),
                            sender_addresses: tx.get("sender_addresses")
                                .and_then(|s| s.as_array())
                                .map(|arr| arr.iter().filter_map(|a| a.as_str().map(|s| s.to_string())).collect())
                                .unwrap_or_default(),
                            timestamp: tx.get("timestamp").and_then(|t| t.as_str()).unwrap_or_default().to_string(),
                        };
                        transactions_map.insert(tx_data.txid.clone(), tx_data);
                    }
                    transactions_map.len()
                } else {
                    0
                };

                // Update bitcoin_wallet_tx channel
                if let Err(e) = CHANNEL.bitcoin_wallet_tx.send((balance_btc, Some(wallet.to_string()), private_key_deleted)) {
                    return Err(format!("Failed to send Bitcoin wallet balance: {}", e));
                }

                // Update btc_transactions_tx channel
                let transaction_state = BTCTransactionState { transactions: transactions_map };
                if let Err(e) = CHANNEL.btc_transactions_tx.send(transaction_state) {
                    return Err(format!("Failed to send Bitcoin transactions: {}", e));
                }

                Ok(())
            } else {
                return Err("Missing wallet field".to_string());
            }
        }
        _ => {
            Err("Non-text message received".to_string())
        }
    }
}