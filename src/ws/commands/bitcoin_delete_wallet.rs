use serde_json::{json, Value};
use tokio_tungstenite::tungstenite::Message;
use crate::ws::connection::ConnectionManager;
use crate::channel::{CHANNEL, WSCommand, ProgressState};

pub async fn execute(
    connection: &mut ConnectionManager,
    bitcoin_current_wallet: &mut String,
    cmd: WSCommand,
) -> Result<(), String> {
    if let Some(wallet) = cmd.wallet.clone() {
        let msg_json = json!({"command": "delete_bitcoin_wallet", "wallet": wallet});
        connection.send(Message::Text(msg_json.to_string())).await?;

        if wallet == *bitcoin_current_wallet {
            *bitcoin_current_wallet = String::new();
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

            if data.get("command").and_then(|c| c.as_str()) != Some("delete_bitcoin_wallet") {
                return Ok(());
            }

            if let Some(wallet) = data.get("wallet").and_then(|w| w.as_str()) {
                if wallet != bitcoin_current_wallet {
                    return Ok(());
                }

                if let Some(error) = data.get("error").and_then(|e| e.as_str()) {
                    let _ = CHANNEL.progress_tx
                        .send(Some(ProgressState {
                            progress: 1.0,
                            message: format!("Failed to delete wallet: {}", error),
                        }))
                        .map_err(|e| format!("Failed to send progress: {}", e))?;
                } else if data.get("status").and_then(|s| s.as_str()) == Some("deleted") {
                    let _ = CHANNEL.progress_tx
                        .send(Some(ProgressState {
                            progress: 1.0,
                            message: "Wallet deleted successfully".to_string(),
                        }))
                        .map_err(|e| format!("Failed to send progress: {}", e))?;
                }
            } else {
                return Err("Missing wallet field".to_string());
            }
        }
        _ => {}
    }
    Ok(())
}