use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;
use crate::ws::connection::ConnectionManager;
use crate::channel::{CHANNEL, WSCommand, ProgressState};
use crate::ws::commands::{validation, wallet_auth, transaction_builder, transaction_sender};


#[derive(Clone)]
struct LedgerData {
    sequence: u32,
    fee: String,
}

pub async fn execute(
    connection: &mut ConnectionManager,
    current_wallet: &mut String,
    cmd: WSCommand,
) -> Result<(), String> {
    // Validate inputs
    let (tx_type, wallet, _passphrase) = validation::validate_inputs(&cmd, current_wallet)?;

    // Fetch ledger data
    let (sequence, fee) = crate::ws::commands::ledger::fetch_ledger_data(connection, &wallet)
        .await
        .map_err(|_| {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: "Error: Transaction failed".to_string(),
            }));
            "Error: Transaction failed".to_string()
        })?;
    let ledger_data = LedgerData { sequence, fee };

    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
        progress: 0.4,
        message: "constructing blob".to_string(),
    }));

    // 1. Authenticate wallet
    // FIX: Cloning here now produces Zeroizing<String>, maintaining security.
    let wallet_obj = wallet_auth::authenticate_wallet(
        cmd.passphrase.clone(), 
        cmd.seed.clone(), 
        cmd.bip39.clone(), 
        &wallet
    )?;

    // 2. Construct transaction blob
    let tx_blob = transaction_builder::construct_blob(
        &wallet_obj,
        &cmd, 
        &tx_type,
        ledger_data.sequence,
        ledger_data.fee.clone(),
    ).await?;

  
    transaction_sender::send_transaction(connection, &wallet, &tx_type, tx_blob).await?;

    Ok(())
}

pub async fn process_response(message: Message, _current_wallet: &str) -> Result<(), String> {
    static FAILED: &str = "Error: Transaction validation failed";
    match message {
        Message::Text(text) => {
            let data: Value = serde_json::from_str(&text).map_err(|_| {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: FAILED.to_string(),
                }));
                FAILED.to_string()
            })?;

            // Handle acknowledgment response
            if data.get("command").and_then(|c| c.as_str()) == Some("submit_transaction")
                && data.get("status").and_then(|s| s.as_str()) == Some("submitted")
            {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 0.8,
                    message: "Awaiting confirmation from Blockchain".to_string(),
                }));
                return Ok(());
            }

            // Handle transaction_ack
            if data.get("command").and_then(|c| c.as_str()) == Some("transaction_ack") {
                return Ok(());
            }

            // Handle submit_transaction_response
            if data.get("command").and_then(|c| c.as_str()) != Some("submit_transaction_response") {
                return Ok(());
            }

            let result = data.get("result").ok_or_else(|| {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: FAILED.to_string(),
                }));
                FAILED.to_string()
            })?;

            let transaction_result = result.get("transaction_result").and_then(|t| t.as_str());
            if transaction_result != Some("tesSUCCESS") {
                let error_message = if transaction_result == Some("tecKILLED") {
                    "The order failed to execute at that price".to_string()
                } else {
                    FAILED.to_string()
                };
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: error_message.clone(),
                }));
                return Err(error_message);
            }

            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: "Your transaction was successful".to_string(),
            }));

            Ok(())
        }
        _ => Ok(()),
    }
}