use crate::decrypt::decrypt_data;
use keyring::Entry;
use zeroize::Zeroize;
use bitcoin::bip32::{Xpriv, DerivationPath};
use bitcoin::network::Network;
use bitcoin::address::Address;
use bitcoin::key::PrivateKey;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::CompressedPublicKey;
use bip39::Mnemonic;
use std::str::FromStr;
use crate::channel::{CHANNEL, ProgressState};

#[derive(Debug)]
pub struct BitcoinWallet {
    pub address: String,
    pub private_key: String, // WIF format
}

pub fn authenticate_wallet(
    passphrase: Option<String>,
    seed: Option<String>,
    wallet_address: &str,
) -> Result<BitcoinWallet, String> {
    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
        progress: 0.3,
        message: "Authenticating wallet".to_string(),
    }));

    let mnemonic_phrase = match (passphrase, seed) {
        (None, Some(s)) => s,
        (Some(p), None) => {
            let mut input = p;
            let entry = Entry::new("bitcoin_wallet", wallet_address).map_err(|e| {
                let err = format!("Error: Keyring entry not found: {}", e);
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: err.clone(),
                }));
                err
            })?;
            let encrypted_data = entry.get_password().map_err(|e| {
                let err = format!("Error: Keyring entry not found: {}", e);
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: err.clone(),
                }));
                err
            })?;
            let (encrypted, salt, iv) = serde_json::from_str::<(String, String, String)>(&encrypted_data)
                .map_err(|e| {
                    let err = format!("Error: Invalid keyring data: {}", e);
                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                        progress: 1.0,
                        message: err.clone(),
                    }));
                    err
                })?;
            let decrypted_seed = decrypt_data(input.clone(), encrypted, salt, iv).map_err(|e| {
                let err = format!("Error: Decryption failed: {}", e);
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: err.clone(),
                }));
                err
            })?;
            input.zeroize();
            decrypted_seed
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

    let mnemonic = Mnemonic::from_str(&mnemonic_phrase).map_err(|e| {
        let err = format!("Error: Invalid mnemonic: {}", e);
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: err.clone(),
        }));
        err
    })?;

    let seed = mnemonic.to_seed("");
    let network = Network::Bitcoin;
    let secp = Secp256k1::new();
    let xpriv = Xpriv::new_master(network, &seed).map_err(|e| {
        let err = format!("Error: Failed to create master key: {}", e);
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: err.clone(),
        }));
        err
    })?;
    let derivation_path = DerivationPath::from_str("m/84'/0'/0'/0/0").map_err(|e| {
        let err = format!("Error: Invalid derivation path: {}", e);
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: err.clone(),
        }));
        err
    })?;
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
        let err = format!(
            "Error: Derived address {} does not match provided address {}",
            derived_address, wallet_address
        );
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: err.clone(),
        }));
        return Err(err);
    }

    let private_key_wif = PrivateKey {
        compressed: true,
        network: bitcoin::network::NetworkKind::Main,
        inner: private_key.inner,
    }
    .to_wif();

    Ok(BitcoinWallet {
        address: wallet_address.to_string(),
        private_key: private_key_wif,
    })
}