use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;
use crate::channel::CHANNEL;

pub async fn execute(
    _connection: &mut crate::ws::connection::ConnectionManager,
    _bitcoin_current_wallet: &mut String,
    _cmd: crate::channel::WSCommand,
) -> Result<(), String> {
    // No client-initiated execution for bitcoin_fees
    Ok(())
}

pub async fn process_response(message: Message, _bitcoin_current_wallet: &str) -> Result<(), String> {
    match message {
        Message::Text(text) => {
            let data: Value = serde_json::from_str(&text)
                .map_err(|e| format!("Failed to parse JSON: {}", e))?;

            let command = data.get("command").and_then(|c| c.as_str());
            if command != Some("bitcoin_fees") {
                return Ok(());
            }

            let fees = data.get("fees").ok_or_else(|| {
                "Missing fees field".to_string()
            })?;

            let fee_2_blocks = fees.get("2_blocks")
                .and_then(|f| f.as_str())
                .and_then(|f| f.parse::<f64>().ok())
                .ok_or_else(|| {
                    "Invalid or missing 2_blocks fee".to_string()
                })?;

            let fee_5_blocks = fees.get("5_blocks")
                .and_then(|f| f.as_str())
                .and_then(|f| f.parse::<f64>().ok())
                .ok_or_else(|| {
                    "Invalid or missing 5_blocks fee".to_string()
                })?;

            let fee_10_blocks = fees.get("10_blocks")
                .and_then(|f| f.as_str())
                .and_then(|f| f.parse::<f64>().ok())
                .ok_or_else(|| {
                    "Invalid or missing 10_blocks fee".to_string()
                })?;

            let fee_20_blocks = fees.get("20_blocks")
                .and_then(|f| f.as_str())
                .and_then(|f| f.parse::<f64>().ok())
                .ok_or_else(|| {
                    "Invalid or missing 20_blocks fee".to_string()
                })?;

            // Optional additional fee (if present, for future-proofing)
            let optional_fee = fees.get("optional")
                .and_then(|f| f.as_str())
                .and_then(|f| f.parse::<f64>().ok());

            CHANNEL
                .bitcoin_fee_tx
                .send((fee_2_blocks, fee_5_blocks, fee_10_blocks, fee_20_blocks, optional_fee))
                .map_err(|e| format!("Failed to send Bitcoin fees: {}", e))?;
        }
        _ => {
            return Err("Non-text message received".to_string());
        }
    }
    Ok(())
}