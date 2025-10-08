// ws/commands/offer_create.rs
use crate::channel::WSCommand;
use xrpl::wallet::Wallet;
use xrpl::models::transactions::offer_create::{OfferCreate, OfferCreateFlag};
use xrpl::models::transactions::{CommonFields, TransactionType};
use xrpl::models::{Amount, IssuedCurrencyAmount, XRPAmount};
use xrpl::transaction::sign;
use rippled_binary_codec::serialize::serialize_tx;
use std::borrow::Cow;

// Define a struct to hold currency metadata
#[derive(Clone, Debug)]
struct CurrencyInfo {
    hex: &'static str,
    issuer: &'static str,
}

// Static mapping of currencies to their metadata
fn get_currency_info(currency: &str) -> Option<CurrencyInfo> {
    match currency {
        "RLUSD" => Some(CurrencyInfo {
            hex: "524C555344000000000000000000000000000000",
            issuer: "rMxCKbEDwqr76QuheSUMdEGf4B9xJ8m5De",
        }),
        "EUROP" => Some(CurrencyInfo {
            hex: "4555524F50000000000000000000000000000000",
            issuer: "rMkEuRii9w9uBMQDnWV5AA43gvYZR9JxVK",
        }),
        _ => None, // XRP or unknown currency
    }
}

pub async fn construct_blob(
    wallet_obj: &Wallet,
    cmd: &WSCommand,
    sequence: u32,
    fee: String,
) -> Result<String, String> {
    // Extract taker_pays and taker_gets
    let taker_pays = cmd.taker_pays.as_ref().ok_or("Missing taker_pays")?;
    let taker_gets = cmd.taker_gets.as_ref().ok_or("Missing taker_gets")?;

    // Helper function to convert amount and currency to Amount type
    let to_amount = |amount: &str, currency: &str| -> Result<Amount, String> {
        if currency == "XRP" {
            let amount_drops = (amount
                .parse::<f64>()
                .map_err(|e| format!("Failed to parse amount for {}: {}", currency, e))? * 1_000_000.0)
                .to_string();
            Ok(Amount::XRPAmount(XRPAmount(Cow::Owned(amount_drops))))
        } else {
            let currency_info = get_currency_info(currency)
                .ok_or_else(|| format!("Unknown currency: {}", currency))?;
            Ok(Amount::IssuedCurrencyAmount(IssuedCurrencyAmount {
                currency: Cow::Owned(currency_info.hex.to_string()),
                issuer: Cow::Owned(currency_info.issuer.to_string()),
                value: Cow::Owned(amount.to_string()),
            }))
        }
    };

    // Convert taker_pays to Amount
    let taker_pays_amount = to_amount(&taker_pays.0, &taker_pays.1)?;

    // Convert taker_gets to Amount
    let taker_gets_amount = to_amount(&taker_gets.0, &taker_gets.1)?;

    // Handle flags
    let mut flags: Vec<OfferCreateFlag> = vec![];
    if let Some(cmd_flags) = &cmd.flags {
        for flag in cmd_flags {
            match flag.as_str() {
                "tfFillOrKill" => flags.push(OfferCreateFlag::TfFillOrKill),
                "tfImmediateOrCancel" => flags.push(OfferCreateFlag::TfImmediateOrCancel),
                _ => (), // Ignore unknown flags
            }
        }
    }

    // Create CommonFields
    let common_fields = CommonFields {
        transaction_type: TransactionType::OfferCreate,
        account: Cow::Owned(wallet_obj.classic_address.clone()),
        fee: Some(XRPAmount(Cow::Owned(fee))),
        sequence: Some(sequence),
        flags: flags.into(),
        account_txn_id: None,
        last_ledger_sequence: None,
        signing_pub_key: None,
        source_tag: None,
        ticket_sequence: None,
        memos: None,
        network_id: None,
        signers: None,
        txn_signature: None,
    };

    // Create the OfferCreate transaction
    let mut offer_create = OfferCreate {
        common_fields,
        taker_gets: taker_gets_amount,
        taker_pays: taker_pays_amount,
        expiration: None,
        offer_sequence: None,
    };

    // Sign the transaction
    sign::<OfferCreate, OfferCreateFlag>(&mut offer_create, wallet_obj, false)
        .map_err(|e| format!("Failed to sign offer_create: {:?}", e))?;

    // Serialize to JSON
    let tx_json = serde_json::to_string(&offer_create)
        .map_err(|e| format!("Failed to serialize offer_create: {}", e))?;

    // Encode to hex blob
    let tx_blob = serialize_tx(tx_json, false)
        .ok_or_else(|| "Failed to encode offer_create to hex".to_string())?;

    Ok(tx_blob)
}