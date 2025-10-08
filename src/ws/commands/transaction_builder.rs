// ws/commands/transaction_builder.rs
use crate::channel::{CHANNEL, ProgressState};
use crate::ws::commands::{payment, trustset, offer_create, trustset_euro};
use crate::channel::WSCommand;
use xrpl::wallet::Wallet;

pub async fn construct_blob(
    wallet_obj: &Wallet,
    cmd: &WSCommand,
    tx_type: &str,
    sequence: u32,
    fee: String,
) -> Result<String, String> {
    let tx_blob = match tx_type {
        "payment" => payment::construct_blob(wallet_obj, cmd, sequence, fee).await,
        "trustset" => trustset::construct_blob(wallet_obj, cmd, sequence, fee).await,
        "offer_create" => offer_create::construct_blob(wallet_obj, cmd, sequence, fee).await,
        "trustset_euro" => trustset_euro::construct_blob(wallet_obj, cmd, sequence, fee).await,
        _ => {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: "Error: Failed to build transaction blob".to_string(),
            }));
            return Err("Error: Failed to build transaction blob".to_string());
        }
    }
    .map_err(|_| {
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: "Error: Transaction failed".to_string(),
        }));
        "Error: Transaction failed".to_string()
    })?;
    Ok(tx_blob)
}