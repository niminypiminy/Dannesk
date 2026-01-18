// Updated submit_transaction.rs (only the relevant part)
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;
use crate::ws::connection::ConnectionManager;
use crate::channel::{CHANNEL, WSCommand, ProgressState};
use crate::ws::commands::{bitcoin_validation, bitcoin_auth, bitcoin_payment, bitcoin_transaction_sender};

pub async fn execute(
    connection: &mut ConnectionManager,
    bitcoin_current_wallet: &mut String,
    cmd: WSCommand,
) -> Result<(), String> {
    // Validate inputs
    let (tx_type, wallet, _passphrase) = bitcoin_validation::validate_inputs(&cmd, bitcoin_current_wallet)?;

    // Fetch UTXO data
    let utxos = crate::ws::commands::bitcoin_ledger::fetch_utxo_data(connection, &wallet)
        .await
        .map_err(|_| {
            let err = "Error: Unable to get UTXO. Transaction Failed.".to_string();
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: err.clone(),
            }));
            err
        })?;

    // Send progress update: constructing transaction
    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
        progress: 0.4,
        message: "constructing transaction".to_string(),
    }));

    // Get fee from cmd.fee
    let fee = cmd.fee.as_ref().ok_or_else(|| {
        let err = "Error: No transaction fee specified".to_string();
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: err.clone(),
        }));
        err
    })?.to_string();
    fee.parse::<u32>().map_err(|_| {
        let err = "Error: Invalid transaction fee format".to_string();
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: err.clone(),
        }));
        err
    })?;

    // Authenticate wallet (now passing bip39)
    let wallet_obj = bitcoin_auth::authenticate_wallet(
        cmd.passphrase.clone(), 
        cmd.seed.clone(), 
        cmd.bip39.clone(), // Pass the optional BIP39
        &wallet
    )?;

    // Construct transaction
    let tx_hex = bitcoin_payment::construct_transaction(&wallet_obj, &cmd, &tx_type, utxos, fee).await?;

    // Send transaction
    bitcoin_transaction_sender::send_transaction(connection, &wallet, &tx_type, tx_hex).await?;

    Ok(())
}

// The process_response function remains unchanged
pub async fn process_response(message: Message, _bitcoin_current_wallet: &str) -> Result<(), String> {
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

            let command = data.get("command").and_then(|c| c.as_str());

            if command == Some("bitcoin_submit_transaction") && data.get("status").and_then(|s| s.as_str()) == Some("submitted") {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 0.8,
                    message: "Almost There. Awaiting confirmation".to_string(),
                }));
                return Ok(());
            }

            if command == Some("bitcoin_transaction_ack") {
                return Ok(());
            }

            if command != Some("submit_bitcoin_transaction_response") {
                return Ok(());
            }

            let result = data.get("result").ok_or_else(|| {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: FAILED.to_string(),
                }));
                FAILED.to_string()
            })?;

            let transaction_result = result.get("status").and_then(|t| t.as_str());

            if transaction_result != Some("success") {
                let error_message = result.get("error")
                    .and_then(|e| e.get("message"))
                    .and_then(|m| m.as_str())
                    .unwrap_or(FAILED)
                    .to_string();
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: error_message.clone(),
                }));
                return Err(error_message);
            }

            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: "Transaction added to Mempool. Status Pending".to_string(),
            }));

            Ok(())
        }
        _ => Ok(())
    }
}