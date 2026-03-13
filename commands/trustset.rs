// ws/commands/trustset.rs
use crate::channel::WSCommand;
use xrpl::wallet::Wallet;
use xrpl::models::transactions::trust_set::{TrustSet, TrustSetFlag};
use xrpl::models::{IssuedCurrencyAmount, XRPAmount};
use xrpl::transaction::sign;
use rippled_binary_codec::serialize::serialize_tx;
use std::borrow::Cow;

// Define known issuers
const RLUSD_ISSUER: &str = "rMxCKbEDwqr76QuheSUMdEGf4B9xJ8m5De";
const EUROP_ISSUER: &str = "rMkEuRii9w9uBMQDnWV5AA43gvYZR9JxVK";
const XSGD_ISSUER: &str = "rK67JczCpaYXVtfw3qJVmqwpSfa1bYTptw";

// Add others like SGD here...

pub async fn construct_blob(
    wallet_obj: &Wallet,
    cmd: &WSCommand,
    sequence: u32,
    fee: String,
) -> Result<String, String> {
    
    // Default to a high limit if not provided
    let trustline_limit_value = cmd.trustline_limit.clone().unwrap_or_else(|| "1000000".to_string());
    
    // We need an identifier to know which trustline to create.
    // Reusing `wallet_type` is perfectly fine, or you could add `asset` to WSCommand.
    let asset_type = cmd.wallet_type.as_deref().ok_or("Missing asset type for trustset")?;

    // Determine Currency Hex and Issuer based on the asset type
    let (currency_hex, issuer_address) = match asset_type {
        "RLUSD" => ("524C555344000000000000000000000000000000", RLUSD_ISSUER),
        "EUROP"  => ("4555524F50000000000000000000000000000000", EUROP_ISSUER),
        "XSGD"  => ("5853474400000000000000000000000000000000", XSGD_ISSUER),
        _ => return Err(format!("Unsupported asset type for trustset: {}", asset_type)),
    };

    let trustline_limit = IssuedCurrencyAmount {
        currency: Cow::Owned(currency_hex.to_string()),
        issuer: Cow::Owned(issuer_address.to_string()),
        value: Cow::Owned(trustline_limit_value),
    };

    let mut trust_set = TrustSet::new(
        Cow::Owned(wallet_obj.classic_address.clone()),
        None,
        Some(XRPAmount(Cow::Owned(fee))),
        Some(vec![TrustSetFlag::TfSetNoRipple].into()),
        None,
        None,
        Some(sequence),
        None,
        None,
        None,
        trustline_limit,
        None,
        None,
    );

    sign::<TrustSet, TrustSetFlag>(&mut trust_set, wallet_obj, false)
        .map_err(|e| format!("Failed to sign trustline: {:?}", e))?;
    
    let tx_json = serde_json::to_string(&trust_set)
        .map_err(|e| format!("Failed to serialize trustline: {}", e))?;
    
    let tx_blob = serialize_tx(tx_json, false).ok_or_else(|| {
        "Failed to encode trustline to hex".to_string()
    })?;
    
    Ok(tx_blob)
}