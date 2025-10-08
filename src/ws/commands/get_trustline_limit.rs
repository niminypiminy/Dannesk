use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;
use crate::channel::CHANNEL;

pub async fn execute(
    _connection: &mut crate::ws::connection::ConnectionManager,
    _current_wallet: &mut String,
    _cmd: crate::channel::WSCommand,
) -> Result<(), String> {
    // No client-initiated execution for get_trustline_limit
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
                if command != Some("get_trustline_limit") {
                    return Ok(());
                }

                let (current_rlusd_balance, _current_has_rlusd, current_trustline_limit) = CHANNEL.rlusd_rx.borrow().clone();

                let has_rlusd = true; // Trustline update implies has_rlusd is true
                let trustline_limit = data
                    .get("trustline_limit")
                    .and_then(|l| l.as_str())
                    .and_then(|l| l.parse::<f64>().ok())
                    .unwrap_or(current_trustline_limit.unwrap_or(0.0));

                CHANNEL
                    .rlusd_tx
                    .send((current_rlusd_balance, has_rlusd, Some(trustline_limit)))
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