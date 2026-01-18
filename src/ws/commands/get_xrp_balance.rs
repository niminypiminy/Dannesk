use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;
use crate::channel::CHANNEL;

pub async fn execute(
    _connection: &mut crate::ws::connection::ConnectionManager,
    _current_wallet: &mut String,
    _cmd: crate::channel::WSCommand,
) -> Result<(), String> {
    // No client-initiated execution for xrp_balance
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
                if command != Some("xrp_balance") {
                    return Ok(());
                }

                let wallet_balance_rx = CHANNEL.wallet_balance_rx.clone();
                let (_current_balance, _wallet_opt, private_key_deleted) = wallet_balance_rx.borrow().clone();

                let balance_xrp = if let Some(balance) = data.get("balance").and_then(|b| b.as_str()) {
                    if balance == "0" && data.get("balance").is_none() {
                        0.0
                    } else if let Ok(balance) = balance.parse::<f64>() {
                        balance / 1_000_000.0
                    } else {
                        return Err(format!("Invalid balance format: {}", balance));
                    }
                } else {
                    return Err("Missing balance field".to_string());
                };

                CHANNEL
                    .wallet_balance_tx
                    .send((balance_xrp, Some(wallet.to_string()), private_key_deleted))
                    .map_err(|e| format!("Failed to send wallet balance: {}", e))?;
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