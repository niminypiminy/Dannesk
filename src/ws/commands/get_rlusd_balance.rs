use serde_json::{json, Value};
use tokio_tungstenite::tungstenite::Message;
use crate::ws::connection::ConnectionManager;
use crate::channel::{CHANNEL, WSCommand};

pub async fn execute(
    connection: &mut ConnectionManager,
    _current_wallet: &mut String,
    cmd: WSCommand,
) -> Result<(), String> {
    // Allow client-initiated RLUSD balance requests
    if let Some(wallet) = cmd.wallet {
        let msg_json = json!({ "command": "get_rlusd_balance", "wallet": wallet });
        connection
            .send(Message::Text(msg_json.to_string()))
            .await?;
        Ok(())
    } else {
        Err("Missing wallet parameter".to_string())
    }
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
                if command != Some("get_rlusd_balance") {
                    return Ok(());
                }

                let (_current_rlusd_balance, _current_has_rlusd, current_trustline_limit) = CHANNEL.rlusd_rx.borrow().clone();

                let rlusd_balance = if let Some(rlusd) = data.get("rlusd_balance").and_then(|r| r.as_str()) {
                    if let Ok(rlusd) = rlusd.parse::<f64>() {
                        rlusd
                    } else {
                        return Err(format!("Invalid rlusd_balance format: {}", rlusd));
                    }
                } else {
                    0.0
                };

                let has_rlusd = rlusd_balance != 0.0;

                CHANNEL
                    .rlusd_tx
                    .send((rlusd_balance, has_rlusd, current_trustline_limit))
                    .map_err(|e| format!("Failed to send RLUSD balance: {}", e))?;
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