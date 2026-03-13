// ws/commands/payment.rs
// This module handles the blob creation for XRP, RLUSD, EURO, and SGD payments
use crate::channel::WSCommand;
use xrpl::wallet::Wallet;
use xrpl::models::transactions::payment::{Payment, PaymentFlag};
use xrpl::models::{Amount, IssuedCurrencyAmount, XRPAmount};
use xrpl::transaction::sign;
use rippled_binary_codec::serialize::serialize_tx;
use std::borrow::Cow;

const RLUSD_ISSUER: &str = "rMxCKbEDwqr76QuheSUMdEGf4B9xJ8m5De";
const EUROP_ISSUER: &str = "rMkEuRii9w9uBMQDnWV5AA43gvYZR9JxVK";
const XSGD_ISSUER: &str = "rK67JczCpaYXVtfw3qJVmqwpSfa1bYTptw";

/// Converts an XRP amount string (e.g., "12.000001" or "12000") to an exact integer drops string.
/// Handles up to 6 decimal places (XRPL precision), truncating excess. No floating-point used.
fn xrp_str_to_drops(xrp_str: &str) -> Result<String, String> {
    if xrp_str.is_empty() || !xrp_str.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '-') {
        return Err("Invalid XRP amount format: must be numeric with optional decimal.".to_string());
    }

    let negative = if xrp_str.starts_with('-') { -1 } else { 1 };
    let abs_str = xrp_str.trim_start_matches('-');

    let parts: Vec<&str> = abs_str.split('.').collect();
    if parts.len() > 2 {
        return Err("Invalid XRP amount: too many decimal points.".to_string());
    }

    let integer_part = parts[0].trim_start_matches('0'); // Remove leading zeros for safety
    let integer_str = if integer_part.is_empty() { "0" } else { integer_part };

    let mut fractional_part = String::new();
    if parts.len() == 2 {
        fractional_part = parts[1].to_string();
    }

    // Pad or truncate fractional to exactly 6 digits (XRPL XRP precision)
    while fractional_part.len() < 6 {
        fractional_part.push('0');
    }
    if fractional_part.len() > 6 {
        // Simple truncate (add rounding logic here if needed: e.g., if 7th digit >= '5', increment last digit)
        fractional_part.truncate(6);
    }

    // Parse to u128 for safety (u64 max covers XRPL limits: ~10^17 drops)
    let integer_drops: u128 = integer_str.parse().map_err(|_| "Invalid integer part.".to_string())?;
    let fractional_drops: u128 = fractional_part.parse().map_err(|_| "Invalid fractional part.".to_string())?;

    let total_drops = integer_drops * 1_000_000 + fractional_drops;
    if negative < 0 && total_drops > 0 {
        return Err("Negative XRP amounts not supported.".to_string());
    }

    // Zero amounts are invalid for payments, but we'll check >0 below
    Ok(total_drops.to_string())
}

fn get_asset_config(wallet_type: &str) -> Option<(&'static str, &'static str)> {
    match wallet_type {
        "RLUSD" => Some(("524C555344000000000000000000000000000000", RLUSD_ISSUER)),
        "EUROP" => Some(("4555524F50000000000000000000000000000000", EUROP_ISSUER)),
        "XSGD" => Some(("5853474400000000000000000000000000000000", XSGD_ISSUER)),
        // Add more assets here as needed, e.g.:
        // "NEW_TOKEN" => Some(("HEX_HERE", NEW_ISSUER)),
        _ => None,
    }
}

/// Helper to create IssuedCurrencyAmount for non-XRP assets
fn create_issued_amount(wallet_type: &str, amount_str: &str) -> Result<Amount<'static>, String> {
    let (currency_hex, issuer) = get_asset_config(wallet_type)
        .ok_or_else(|| format!("Unsupported issued currency: {}", wallet_type))?;

    let amount_value = amount_str.parse::<f64>().map_err(|e| format!("Failed to parse amount: {}", e))?;
    if amount_value <= 0.0 {
        return Err("Amount must be greater than zero.".to_string());
    }
    // Format amount to string with up to 15 decimal places (XRPL precision for issued currencies)
    let formatted_amount = format!("{:.15}", amount_value)
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string();

    Ok(Amount::IssuedCurrencyAmount(IssuedCurrencyAmount {
        currency: Cow::Owned(currency_hex.to_string()),
        issuer: Cow::Owned(issuer.to_string()),
        value: Cow::Owned(formatted_amount),
    }))
}

pub async fn construct_blob(
    wallet_obj: &Wallet,
    cmd: &WSCommand,
    sequence: u32,
    fee: String,
) -> Result<String, String> {
    let recipient = cmd.recipient.as_ref().ok_or("Missing recipient")?;
    let amount_str = cmd.amount.as_ref().ok_or("Missing amount")?;
    let wallet_type = cmd.wallet_type.as_ref().ok_or("Missing wallet_type")?;

    // Parse and validate amount based on wallet_type
    let amount = match wallet_type.as_str() {
        "XRP" => {
            let amount_drops = xrp_str_to_drops(amount_str)?;
            if amount_drops == "0" {
                return Err("Amount must be greater than zero.".to_string());
            }
            Amount::XRPAmount(XRPAmount(Cow::Owned(amount_drops)))
        }
        _ => create_issued_amount(wallet_type.as_str(), amount_str)?,
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