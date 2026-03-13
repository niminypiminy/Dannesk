use serde_json::{json, Value};
use tokio_tungstenite::tungstenite::Message;
use crate::ws::connection::ConnectionManager;
use crate::channel::{CHANNEL, WSCommand};

#[derive(Clone, Debug, Copy, PartialEq)]
enum Asset {
    XRP,
    RLUSD,
    EURO,
    XSGD,
}

impl Asset {
    fn from_command(command: &str) -> Option<Self> {
        match command {
            "xrp_balance"        => Some(Asset::XRP),
            "get_rlusd_balance"  => Some(Asset::RLUSD),
            "get_euro_balance"   => Some(Asset::EURO),
            "get_xsgd_balance"   => Some(Asset::XSGD),
            _ => None,
        }
    }

    /// Maps the asset to its specific JSON key in the payload
    fn json_key(&self) -> &'static str {
        match self {
            Asset::XRP   => "balance",
            Asset::RLUSD => "rlusd_balance",
            Asset::EURO  => "euro_balance",
            Asset::XSGD  => "xsgd_balance",
        }
    }

    /// Handles the specific channel sending logic for each asset
    fn update_channel(&self, wallet: &str, balance: f64) -> Result<(), String> {
        match self {
            Asset::XRP => {
                // XRP channel tuple: (balance, wallet_address, private_key_deleted)
                let (_, _, pk_deleted) = CHANNEL.wallet_balance_rx.borrow().clone();
                CHANNEL.wallet_balance_tx
                    .send((balance, Some(wallet.to_string()), pk_deleted))
                    .map_err(|e| e.to_string())
            }
            _ => {
                // Issued currencies tuple: (balance, has_asset, trustline_limit)
                let rx = match self {
                    Asset::RLUSD => &CHANNEL.rlusd_rx,
                    Asset::EURO  => &CHANNEL.euro_rx,
                    Asset::XSGD  => &CHANNEL.sgd_rx,
                    _ => unreachable!(),
                };
                
                let tx = match self {
                    Asset::RLUSD => &CHANNEL.rlusd_tx,
                    Asset::EURO  => &CHANNEL.euro_tx,
                    Asset::XSGD  => &CHANNEL.sgd_tx,
                    _ => unreachable!(),
                };

                // CRITICAL FIX: Clone both the 'has_asset' flag and the 'limit'
                // We only replace the balance (the first element).
                let (_, has_asset, limit) = rx.borrow().clone();
                
                tx.send((balance, has_asset, limit))
                    .map_err(|e| e.to_string())
            }
        }
    }
}

// ====================== Main Logic ======================

pub async fn execute(
    connection: &mut ConnectionManager,
    _current_wallet: &mut String,
    cmd: WSCommand,
) -> Result<(), String> {
    let wallet = cmd.wallet.as_ref().ok_or("Missing wallet parameter")?;
    
    // Determine which command to send based on the WSCommand type/context
    // Usually, client-initiated balance refreshes are for issued currencies.
    // XRP is typically pushed by the backend, but we can support it here if needed.
    let command = match cmd.command.as_str() {
        "get_rlusd_balance" => "get_rlusd_balance",
        "get_euro_balance"  => "get_euro_balance",
        "get_xsgd_balance"  => "get_xsgd_balance",
        _ => return Ok(()), // Ignore unknown balance requests
    };

    let msg = json!({ "command": command, "wallet": wallet });
    connection.send(Message::text(msg.to_string())).await
}

pub async fn process_response(message: Message, current_wallet: &str) -> Result<(), String> {
    let Message::Text(text) = message else {
        return Err("Non-text message received".to_string());
    };

    let data: Value = serde_json::from_str(&text)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let wallet = data.get("wallet").and_then(|w| w.as_str())
        .ok_or("Missing wallet field")?;

    if wallet != current_wallet {
        return Ok(());
    }

    let command_str = data.get("command").and_then(|c| c.as_str())
        .ok_or("Missing command field")?;

    if let Some(asset) = Asset::from_command(command_str) {
        let raw_balance = data.get(asset.json_key())
            .and_then(|v| v.as_str())
            .ok_or_else(|| format!("Missing {} field", asset.json_key()))?;

        let mut balance = raw_balance.parse::<f64>()
            .map_err(|_| format!("Invalid balance format: {}", raw_balance))?;

        // XRP specific: Convert drops to unit
        if asset == Asset::XRP {
            balance /= 1_000_000.0;
        }

        asset.update_channel(wallet, balance)
            .map_err(|e| format!("Failed to send {:?} update: {}", asset, e))?;
    }

    Ok(())
}