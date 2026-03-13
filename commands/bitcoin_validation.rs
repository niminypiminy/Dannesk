use crate::channel::{WSCommand, ProgressState, CHANNEL};

pub fn validate_inputs(
    cmd: &WSCommand,
    bitcoin_current_wallet: &str,
) -> Result<(String, String, String), String> {
    let tx_type = cmd.tx_type.as_ref().ok_or_else(|| {
        let error = "Missing tx_type".to_string();
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: error.clone(),
        }));
        error
    })?.to_string();

    let wallet = cmd.wallet.as_ref().ok_or_else(|| {
        let error = "Missing wallet".to_string();
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: error.clone(),
        }));
        error
    })?.to_string();

    if cmd.passphrase.is_none() && cmd.seed.is_none() {
        let error = "Error: Must provide either passphrase or seed".to_string();
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: error.clone(),
        }));
        return Err(error);
    }

    if wallet != bitcoin_current_wallet {
        let error = "Wallet does not match current Bitcoin wallet".to_string();
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: error.clone(),
        }));
        return Err(error);
    }

    if tx_type != "BTC" {
        let error = "Invalid transaction type for Bitcoin".to_string();
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: error.clone(),
        }));
        return Err(error);
    }

    Ok((tx_type, wallet, String::new()))
}