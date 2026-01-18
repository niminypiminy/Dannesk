use serde_json::{self, Value};
use crate::channel::{CHANNEL, WSCommand};
use crate::utils::json_storage;
use tokio::sync::mpsc;

pub fn load_wallets(commands_tx: mpsc::Sender<WSCommand>) {
    // Load settings from settings.json
    match json_storage::read_json::<Value>("settings.json") {
        Ok(json) => {
            let name = json["saved_name"].as_str().unwrap_or("anonymous").to_string();
            let hide_balance = json["hide_balance"].as_bool().unwrap_or(false);
            let _ = CHANNEL.theme_user_tx.send((true, name.clone(), hide_balance));
        }
        Err(_) => {}
    }

    // Load XRP wallet from xrp.json
    if json_storage::get_config_path("xrp.json")
        .map(|path| path.exists())
        .unwrap_or(false)
    {
        match json_storage::read_json::<Value>("xrp.json") {
            Ok(json) => {
                let address = json.get("address").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let private_key_deleted = json.get("private_key_deleted").and_then(|v| v.as_bool()).unwrap_or(false);

                // Update XRP wallet channel with initial data
                if !address.is_empty() {
                    let _ = CHANNEL.wallet_balance_tx.send((0.0, Some(address.clone()), private_key_deleted));

                    // Send get_cached_balance command
                    let command = WSCommand {
                        command: "get_cached_balance".to_string(),
                        wallet: Some(address.clone()),
                        recipient: None,
                        amount: None,
                        passphrase: None,
                        trustline_limit: None,
                        fee: None,
                        tx_type: None,
                        taker_pays: None,
                        taker_gets: None,
                        seed: None,
                        flags: None,
                        wallet_type: None,
                        bip39: None,
                    };
                    let _ = commands_tx.try_send(command);
                }
            }
            Err(_) => {}
        }
    }

    // Load Bitcoin wallet from btc.json
    if json_storage::get_config_path("btc.json")
        .map(|path| path.exists())
        .unwrap_or(false)
    {
        match json_storage::read_json::<Value>("btc.json") {
            Ok(json) => {
                let address = json.get("address").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let private_key_deleted = json.get("private_key_deleted").and_then(|v| v.as_bool()).unwrap_or(false);

                // Update BTC wallet channel with initial data
                if !address.is_empty() {
                    let _ = CHANNEL.bitcoin_wallet_tx.send((0.0, Some(address.clone()), private_key_deleted));

                    // Send get_bitcoin_cached_balance command
                    let command = WSCommand {
                        command: "get_bitcoin_cached_balance".to_string(),
                        wallet: Some(address.clone()),
                        recipient: None,
                        amount: None,
                        passphrase: None,
                        trustline_limit: None,
                        fee: None,
                        tx_type: None,
                        taker_pays: None,
                        taker_gets: None,
                        seed: None,
                        flags: None,
                        wallet_type: None,
                        bip39: None,
                    };
                    let _ = commands_tx.try_send(command);
                }
            }
            Err(_) => {}
        }
    }
}

