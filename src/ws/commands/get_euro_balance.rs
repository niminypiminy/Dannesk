use serde_json::{json, Value};
use tokio_tungstenite::tungstenite::Message;
use crate::ws::connection::ConnectionManager;
use crate::channel::{CHANNEL, WSCommand};

pub async fn execute(
    connection: &mut ConnectionManager,
    _current_wallet: &mut String,
    cmd: WSCommand,
) -> Result<(), String> {
    // Allow client-initiated Euro balance requests
    if let Some(wallet) = &cmd.wallet {
        let msg_json = json!({ "command": "get_euro_balance", "wallet": wallet });
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
                if command != Some("get_euro_balance") {
                    return Ok(());
                }

                let (_current_euro_balance, _current_has_euro, current_trustline_limit) = CHANNEL.euro_rx.borrow().clone();

                let euro_balance = if let Some(euro) = data.get("euro_balance").and_then(|r| r.as_str()) {
                    if let Ok(euro) = euro.parse::<f64>() {
                        euro
                    } else {
                        return Err(format!("Invalid euro_balance format: {}", euro));
                    }
                } else {
                    0.0
                };

                let has_euro = euro_balance != 0.0;

                CHANNEL
                    .euro_tx
                    .send((euro_balance, has_euro, current_trustline_limit))
                    .map_err(|e| format!("Failed to send Euro balance: {}", e))?;
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