use crate::channel::WSCommand;
use crate::ws::commands::bitcoin_ledger::UTXO;
use crate::ws::commands::bitcoin_auth::BitcoinWallet;
use bitcoin::blockdata::transaction::{Transaction, TxIn, TxOut, OutPoint};
use bitcoin::blockdata::script::ScriptBuf;
use bitcoin::address::Address;
use bitcoin::key::PrivateKey;
use bitcoin::secp256k1::{Secp256k1, Message};
use bitcoin::sighash::{SighashCache, EcdsaSighashType};
use bitcoin::consensus::encode::serialize_hex;
use bitcoin::absolute::LockTime;
use bitcoin::transaction::Version;
use bitcoin::amount::Amount;
use bitcoin::CompressedPublicKey;
use std::str::FromStr;

pub fn calculate_unconfirmed_balance(utxos: &[UTXO]) -> u64 {
    let balance: u64 = utxos.iter().map(|utxo| utxo.amount).sum();
    balance
}

pub fn select_utxos(utxos: &[UTXO], amount: u64, fee: u64) -> Result<Vec<UTXO>, String> {
    let target = amount + fee;

    // Validate unconfirmed balance
    let unconfirmed_balance = calculate_unconfirmed_balance(utxos);
    if unconfirmed_balance < target {
        return Err(format!(
            "Insufficient unconfirmed balance: needed {} satoshis, available {} satoshis",
            target, unconfirmed_balance
        ));
    }

    // Select UTXOs in order (first-fit approach)
    let mut selected: Vec<UTXO> = Vec::new();
    let mut total: u64 = 0;
    for utxo in utxos.iter() {
        if total < target {
            selected.push(utxo.clone());
            total += utxo.amount;
        }
    }

    if total < target {
        return Err(format!(
            "Insufficient funds after UTXO selection: needed {} satoshis, got {} satoshis",
            target, total
        ));
    }

    Ok(selected)
}

pub async fn construct_transaction(
    wallet_obj: &BitcoinWallet,
    cmd: &WSCommand,
    _tx_type: &str,
    utxos: Vec<UTXO>,
    fee: String,
) -> Result<String, String> {
    // Check recipient
    let recipient = cmd.recipient.as_ref().ok_or("Missing recipient".to_string())?;

    // Check amount
    let amount_str = cmd.amount.as_ref().ok_or("Missing amount".to_string())?;

    // Parse and validate amount
    let amount_btc = amount_str.parse::<f64>().map_err(|e| {
        format!("Failed to parse amount as float: {}", e)
    })?;
    if amount_btc <= 0.0 {
        return Err("Amount must be greater than zero.".to_string());
    }
    // Convert BTC to satoshis
    let amount = (amount_btc * 100_000_000.0).round() as u64;
    if amount == 0 {
        return Err("Amount must be greater than zero after conversion.".to_string());
    }

    // Parse fee
    let fee = fee.parse::<u64>().map_err(|e| {
        format!("Failed to parse fee: {}", e)
    })?;

    // Validate unconfirmed balance
    let unconfirmed_balance = calculate_unconfirmed_balance(&utxos);
    let target = amount + fee;
    if unconfirmed_balance < target {
        return Err(format!(
            "Insufficient unconfirmed balance: needed {} satoshis, available {} satoshis",
            target, unconfirmed_balance
        ));
    }

    // Select UTXOs
    let selected_utxos = select_utxos(&utxos, amount, fee)?;
    let total_input: u64 = selected_utxos.iter().map(|utxo| utxo.amount).sum();

    // Parse recipient address
    let recipient_addr = Address::from_str(recipient).map_err(|e| {
        format!("Invalid recipient address: {}", e)
    })?;
    let recipient_addr = recipient_addr.require_network(bitcoin::Network::Bitcoin).map_err(|e| {
        format!("Invalid network for recipient: {}", e)
    })?;

    // Create transaction inputs
    let inputs: Vec<TxIn> = selected_utxos
        .iter()
        .map(|utxo| {
            Ok(TxIn {
                previous_output: OutPoint {
                    txid: bitcoin::Txid::from_str(&utxo.txid).map_err(|e| {
                        format!("Invalid txid {}: {}", utxo.txid, e)
                    })?,
                    vout: utxo.vout,
                },
                script_sig: ScriptBuf::new(),
                sequence: bitcoin::Sequence::MAX,
                witness: bitcoin::Witness::new(),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;

    // Create transaction
    let mut tx = Transaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO,
        input: inputs,
        output: vec![
            TxOut {
                value: Amount::from_sat(amount),
                script_pubkey: recipient_addr.script_pubkey(),
            },
        ],
    };

    // Add change output if necessary
    let change = total_input - amount - fee;
    if change > 0 {
        let change_addr = Address::from_str(&wallet_obj.address).map_err(|e| {
            format!("Invalid wallet address: {}", e)
        })?;
        let change_addr = change_addr.require_network(bitcoin::Network::Bitcoin).map_err(|e| {
            format!("Invalid network for wallet: {}", e)
        })?;
        tx.output.push(TxOut {
            value: Amount::from_sat(change),
            script_pubkey: change_addr.script_pubkey(),
        });
    }

    // Sign transaction (P2WPKH)
    let mut witnesses = Vec::new();
    let secp = Secp256k1::new();
    let private_key = PrivateKey::from_wif(&wallet_obj.private_key).map_err(|e| {
        format!("Invalid private key: {}", e)
    })?;
    let secret_key = private_key.inner;
    let public_key = private_key.public_key(&secp);
    let compressed_public_key = CompressedPublicKey(public_key.inner);

    for (i, _) in tx.input.iter().enumerate() {
        let prev_output_script = Address::p2wpkh(&compressed_public_key, bitcoin::Network::Bitcoin).script_pubkey();
        let mut sighhash_cache = SighashCache::new(&tx);
        let sighash = sighhash_cache
            .p2wpkh_signature_hash(
                i,
                &prev_output_script,
                Amount::from_sat(selected_utxos[i].amount),
                EcdsaSighashType::All,
            )
            .map_err(|e| {
                format!("Failed to compute sighash for input {}: {}", i, e)
            })?;
        let message = Message::from(sighash);
        let signature = secp.sign_ecdsa(&message, &secret_key);
        let signature_with_sighash = {
            let mut sig = signature.serialize_der().to_vec();
            sig.push(EcdsaSighashType::All as u8);
            sig
        };
        witnesses.push(bitcoin::Witness::from_slice(&[
            &signature_with_sighash[..],
            &compressed_public_key.to_bytes()[..],
        ]));
    }

    // Apply witnesses
    for (input, witness) in tx.input.iter_mut().zip(witnesses.into_iter()) {
        input.witness = witness;
    }

    // Serialize transaction
    let tx_hex = serialize_hex(&tx);
    Ok(tx_hex)
}