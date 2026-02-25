// src/ui/managebtc/btcimport/btcimportlogic.rs

use std::str::FromStr;
use tokio::sync::mpsc::Sender;
use bitcoin::bip32::{Xpriv, DerivationPath};
use bitcoin::{Network, CompressedPublicKey};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::address::Address;
use bip39::{Language, Mnemonic};
// Removed: keyring::Entry
use arboard::Clipboard;
use zeroize::{Zeroize, Zeroizing};
use serde::Serialize;

use crate::encrypt::encrypt_data;
use crate::utils::json_storage::write_json; // Import your utility
use crate::channel::{
    CHANNEL, WSCommand, ProgressState, BTCModalState, BTCActiveView, BTCWalletProcessState
};

// Define a struct to keep the JSON structure clean
#[derive(Serialize)]
struct EncryptedWalletData {
    address: String,
    encrypted_phrase: String,
    salt: String,
    iv: String,
}

pub struct BTCImportLogic;

impl BTCImportLogic {
    pub async fn process(
        mnemonic_phrase: Zeroizing<String>,
        bip39_pass: Zeroizing<String>,
        encryption_pass: Zeroizing<String>,
        ws_tx: Sender<WSCommand>,
    ) {
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 0.0,
            message: "Starting Bitcoin wallet import...".to_string(),
        }));

        let m_thread = mnemonic_phrase.clone();
        let b_thread = bip39_pass.clone();
        let e_thread = encryption_pass.clone();

        let crypto_result = tokio::task::spawn_blocking(move || -> Result<(String, String, String, String), String> {
            let mnemonic = Mnemonic::parse_in(Language::English, m_thread.as_str())
                .map_err(|e| format!("Invalid mnemonic: {}", e))?;

            // Your AES-256 logic
            let (enc, salt, iv) = encrypt_data(e_thread, m_thread)
                .map_err(|e| format!("Encryption failed: {}", e))?;

            let mut seed = mnemonic.to_seed(b_thread.as_str());
            let network = Network::Bitcoin;
            let secp = Secp256k1::new();

            let xpriv = Xpriv::new_master(network, &seed)
                .map_err(|e| {
                    seed.zeroize(); 
                    format!("Failed to create master key: {}", e)
                })?;

            seed.zeroize();

            let derivation_path = DerivationPath::from_str("m/84'/0'/0'/0/0")
                .map_err(|_| "Invalid derivation path".to_string())?;
            
            let child_xpriv = xpriv.derive_priv(&secp, &derivation_path)
                .map_err(|e| format!("Derivation failed: {}", e))?;

            let public_key = child_xpriv.to_priv().public_key(&secp);
            let compressed_pubkey = CompressedPublicKey(public_key.inner);
            let address = Address::p2wpkh(&compressed_pubkey, network);

            Ok((address.to_string(), enc, salt, iv))
        }).await;

        match crypto_result {
            Ok(Ok((address, encrypted, salt, iv))) => {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 0.5,
                    message: "Saving encrypted credentials...".to_string(),
                }));

                // --- NEW DETERMINISTIC STORAGE LOGIC ---
                let wallet_data = EncryptedWalletData {
                    address: address.clone(),
                    encrypted_phrase: encrypted,
                    salt,
                    iv,
                };

                // This uses your utility to write to 'encrypt.json' 
                // It overwrites if it already exists per your requirement.
                if let Err(e) = write_json("btc_encrypt.json", &wallet_data) {
                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                        progress: 1.0,
                        message: format!("File System Error: {}", e),
                    }));
                    return; 
                }
                // ----------------------------------------

                let _ = ws_tx.try_send(WSCommand {
                    command: "import_bitcoin_wallet".to_string(),
                    wallet: Some(address),
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
                });

                if let Ok(mut ctx) = Clipboard::new() {
                    let _ = ctx.set_text("");
                }

                let _ = CHANNEL.btc_wallet_process_tx.send(BTCWalletProcessState {
                    import_wallet: None,
                    create_wallet: None,
                });

                let _ = CHANNEL.btc_modal_tx.send(BTCModalState {
                    view_type: BTCActiveView::BTC,
                    last_view: None,
                });
            }
            Ok(Err(e)) => {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: format!("Error: {}", e),
                }));
            }
            Err(e) => {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: format!("Internal Thread Error: {}", e),
                }));
            }
        }
    }
}