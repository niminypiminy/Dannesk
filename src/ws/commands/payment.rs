// ws/commands/payment.rs
// This module handles the blob creation for XRP, RLUSD, and EURO payments
use crate::channel::WSCommand;
use xrpl::wallet::Wallet;
use xrpl::models::transactions::payment::{Payment, PaymentFlag};
use xrpl::models::{Amount, IssuedCurrencyAmount, XRPAmount};
use xrpl::transaction::sign;
use rippled_binary_codec::serialize::serialize_tx;
use std::borrow::Cow;

pub async fn construct_blob(
    wallet_obj: &Wallet,
    cmd: &WSCommand,
    sequence: u32,
    fee: String,
) -> Result<String, String> {
    const RLUSD_ISSUER: &str = "rMxCKbEDwqr76QuheSUMdEGf4B9xJ8m5De";
    const EUR_ISSUER: &str = "rMkEuRii9w9uBMQDnWV5AA43gvYZR9JxVK";
    let recipient = cmd.recipient.as_ref().ok_or("Missing recipient")?;
    let amount_str = cmd.amount.as_ref().ok_or("Missing amount")?;
    let wallet_type = cmd.wallet_type.as_ref().ok_or("Missing wallet_type")?;

    // Parse and validate amount
    let amount_value = amount_str
        .parse::<f64>()
        .map_err(|e| format!("Failed to parse amount: {}", e))?;
    if amount_value <= 0.0 {
        return Err("Amount must be greater than zero.".to_string());
    }

    // Construct the Amount based on wallet_type
    let amount = match wallet_type.as_str() {
        "XRP" => {
            let amount_drops = (amount_value * 1_000_000.0).to_string();
            Amount::XRPAmount(XRPAmount(Cow::Owned(amount_drops)))
        }
        "RLUSD" => {
            // Format amount to string with up to 15 decimal places (XRPL precision for issued currencies)
            let formatted_amount = format!("{:.15}", amount_value)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string();
            Amount::IssuedCurrencyAmount(IssuedCurrencyAmount {
                currency: Cow::Owned("524C555344000000000000000000000000000000".to_string()), // RLUSD
                issuer: Cow::Owned(RLUSD_ISSUER.to_string()),
                value: Cow::Owned(formatted_amount),
            })
        }
        "EURO" => {
            // Format amount to string with up to 15 decimal places (XRPL precision for issued currencies)
            let formatted_amount = format!("{:.15}", amount_value)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string();
            Amount::IssuedCurrencyAmount(IssuedCurrencyAmount {
                currency: Cow::Owned("4555524F50000000000000000000000000000000".to_string()), // EUR
                issuer: Cow::Owned(EUR_ISSUER.to_string()),
                value: Cow::Owned(formatted_amount),
            })
        }
        _ => return Err(format!("Unsupported wallet_type: {}", wallet_type)),
    };

    // Create the Payment transaction
    let mut payment = Payment::new(
        Cow::Owned(wallet_obj.classic_address.clone()),
        None,
        Some(XRPAmount(Cow::Owned(fee))),
        None,
        None,
        None,
        Some(sequence),
        None,
        None,
        None,
        amount,
        Cow::Owned(recipient.clone()),
        None,
        None,
        None,
        None,
        None,
    );

    // Sign and serialize the transaction
    sign::<Payment, PaymentFlag>(&mut payment, wallet_obj, false)
        .map_err(|e| format!("Failed to sign transaction: {:?}", e))?;
    let tx_json = serde_json::to_string(&payment)
        .map_err(|e| format!("Failed to serialize transaction: {}", e))?;
    let tx_blob = serialize_tx(tx_json, false).ok_or_else(|| {
        "Failed to encode transaction to hex".to_string()
    })?;

    Ok(tx_blob)
}