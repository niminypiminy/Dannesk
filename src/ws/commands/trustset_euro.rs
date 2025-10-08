// ws/commands/trustset.rs
use crate::channel::WSCommand;
use xrpl::wallet::Wallet;
use xrpl::models::transactions::trust_set::{TrustSet, TrustSetFlag};
use xrpl::models::{IssuedCurrencyAmount, XRPAmount};
use xrpl::transaction::sign;
use rippled_binary_codec::serialize::serialize_tx;
use std::borrow::Cow;

pub async fn construct_blob(
    wallet_obj: &Wallet,
    cmd: &WSCommand,
    sequence: u32,
    fee: String,
) -> Result<String, String> {
    const EUR_ISSUER: &str = "rMkEuRii9w9uBMQDnWV5AA43gvYZR9JxVK";
    let trustline_limit_value = cmd.trustline_limit.clone().unwrap_or_else(|| "1000000".to_string());
    let trustline_limit = IssuedCurrencyAmount {
        currency: Cow::Owned("4555524F50000000000000000000000000000000".to_string()), // EUR
        issuer: Cow::Owned(EUR_ISSUER.to_string()),
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