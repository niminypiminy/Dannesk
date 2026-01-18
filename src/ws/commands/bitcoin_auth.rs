// src/ws/commands/bitcoin_auth.rs

use crate::decrypt::decrypt_data;
use crate::utils::json_storage::read_json; // Use your utility
use zeroize::Zeroizing;
use bitcoin::bip32::{Xpriv, DerivationPath};
use bitcoin::network::Network;
use bitcoin::address::Address;
use bitcoin::key::PrivateKey;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::CompressedPublicKey;
use bip39::Mnemonic;
use std::str::FromStr;
use serde::Deserialize;
use crate::channel::{CHANNEL, ProgressState};

#[derive(Debug)]
pub struct BitcoinWallet {
    pub address: String,
    pub private_key: String, // WIF format
}

// Matches the structure used in btcimportlogic.rs
#[derive(Deserialize)]
struct EncryptedWalletData {
    address: String,
    encrypted_phrase: String,
    salt: String,
    iv: String,
}

pub fn authenticate_wallet(
    passphrase: Option<Zeroizing<String>>, 
    seed: Option<Zeroizing<String>>,       
    bip39: Option<Zeroizing<String>>,
    wallet_address: &str,
) -> Result<BitcoinWallet, String> {
    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
        progress: 0.3,
        message: "Authenticating wallet".to_string(),
    }));

    let mnemonic_phrase: Zeroizing<String> = match (passphrase, seed) {
        (None, Some(s)) => s,
        (Some(p), None) => {
            let input = p;

            // --- NEW FILE-BASED AUTHENTICATION ---
            // Read from encrypt.json instead of the system keyring
            let stored_data: EncryptedWalletData = read_json("btc_encrypt.json").map_err(|e| {
                let err = format!("Error: Encrypted wallet file not found or corrupted: {}", e);
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: "Error: Could not find encrypted credentials.".to_string(),
                }));
                err
            })?;

            // Verify the file belongs to the wallet the user is currently trying to use
            if stored_data.address != wallet_address {
                return Err("Error: Stored encrypted data does not match current wallet address.".to_string());
            }

            let decrypted_seed = decrypt_data(
                input.clone(), 
                stored_data.encrypted_phrase, 
                stored_data.salt, 
                stored_data.iv
            ).map_err(|e| {
                let err = format!("Error: Decryption failed: {}", e);
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: "Error: Incorrect passphrase.".to_string(),
                }));
                err
            })?;
            // --------------------------------------
            
            Zeroizing::new(decrypted_seed)
        }
        _ => {
            let err = "Error: Must provide either passphrase or seed".to_string();
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: err.clone(),
            }));
            return Err(err);
        }
    };

    // ... Rest of the derivation logic remains exactly as you had it ...
    let mnemonic = Mnemonic::from_str(mnemonic_phrase.as_str()).map_err(|e| {
        let err = format!("Error: Invalid mnemonic: {}", e);
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: err.clone(),
        }));
        err
    })?;

    let seed_passphrase = bip39.as_deref().map(|s| s.as_str()).unwrap_or("");
    let seed_bytes = mnemonic.to_seed(seed_passphrase);

    let network = Network::Bitcoin;
    let secp = Secp256k1::new();
    let xpriv = Xpriv::new_master(network, &seed_bytes).map_err(|e| {
        let err = format!("Error: Failed to create master key: {}", e);
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: err.clone(),
        }));
        err
    })?;

    let derivation_path = DerivationPath::from_str("m/84'/0'/0'/0/0").unwrap();
    let child_xpriv = xpriv.derive_priv(&secp, &derivation_path).map_err(|e| {
        let err = format!("Error: Failed to derive private key: {}", e);
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: err.clone(),
        }));
        err
    })?;

    let private_key = child_xpriv.to_priv();
    let public_key = private_key.public_key(&secp);
    let compressed_pubkey = CompressedPublicKey(public_key.inner);
    let derived_address = Address::p2wpkh(&compressed_pubkey, network);

    if derived_address.to_string() != wallet_address {
        return Err("Error: Derived address does not match.".to_string());
    }

    let private_key_wif = PrivateKey {
        compressed: true,
        network: bitcoin::network::NetworkKind::Main,
        inner: private_key.inner,
    }.to_wif();

    Ok(BitcoinWallet {
        address: wallet_address.to_string(),
        private_key: private_key_wif,
    })
}