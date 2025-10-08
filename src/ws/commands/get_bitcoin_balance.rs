use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;
use crate::channel::CHANNEL;

pub async fn execute(
    _connection: &mut crate::ws::connection::ConnectionManager,
    _bitcoin_current_wallet: &mut String,
    _cmd: crate::channel::WSCommand,
) -> Result<(), String> {
    // No client-initiated execution for btc_balance
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
                if command != Some("btc_balance") {
                    return Ok(());
                }

                let bitcoin_wallet_rx = CHANNEL.bitcoin_wallet_rx.clone();
                let (_current_balance, _wallet_opt, private_key_deleted) = bitcoin_wallet_rx.borrow().clone();

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

                CHANNEL
                    .bitcoin_wallet_tx
                    .send((balance_btc, Some(wallet.to_string()), private_key_deleted))
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