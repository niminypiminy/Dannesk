// src/ui/managexrp/xrpimport/xrpimportlogic.rs

use tokio::sync::mpsc::Sender;
use xrpl::wallet::Wallet;
use bip39::{Language, Mnemonic};
use ripple_address_codec::{encode_seed, Ed25519};
use crate::encrypt::encrypt_data;
use crate::utils::json_storage::write_json; // Use your utility
use crate::channel::{CHANNEL, WSCommand, ProgressState, XRPWalletProcessState, XRPModalState, ActiveView};
use arboard::Clipboard; 
use zeroize::{Zeroize, Zeroizing};
use serde::Serialize;

#[derive(Serialize)]
struct EncryptedWalletData {
    address: String,
    encrypted_phrase: String,
    salt: String,
    iv: String,
}

pub struct XRPImportLogic;

impl XRPImportLogic {
   pub async fn process(
    mnemonic_phrase: Zeroizing<String>,
    bip39_pass: Zeroizing<String>,
    encryption_pass: Zeroizing<String>,
    ws_tx: Sender<WSCommand>,
) {
    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
        progress: 0.0,
        message: "Starting XRP wallet import...".to_string(),
    }));

    let m_thread = mnemonic_phrase.clone();
    let b_thread = bip39_pass.clone();
    let e_thread = encryption_pass.clone();

    let crypto_result = tokio::task::spawn_blocking(move || -> Result<(String, String, String, String), String> {
        let mnemonic = Mnemonic::parse_in(Language::English, m_thread.as_str())
            .map_err(|e| format!("Invalid mnemonic: {}", e))?;

        let seed_bytes = mnemonic.to_seed(b_thread.as_str());
        
        // XRP specific derivation (Ed25519)
        let mut entropy: [u8; 16] = seed_bytes[0..16].try_into().expect("BIP39 error");
        let mut base58_seed = encode_seed(&entropy, &Ed25519);
        
        entropy.zeroize(); 

        let wallet = Wallet::new(&base58_seed, 0)
            .map_err(|e| {
                base58_seed.zeroize();
                format!("Wallet creation failed: {}", e)
            })?;

        let address = wallet.classic_address.clone();
        base58_seed.zeroize();

        // Encrypt mnemonic with AES-256
        let (enc, salt, iv) = encrypt_data(e_thread, m_thread)
            .map_err(|e| format!("Encryption failed: {}", e))?;

        Ok((address, enc, salt, iv))
    }).await;

    match crypto_result {
        Ok(Ok((address, encrypted, salt, iv))) => {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 0.5,
                message: "Saving encrypted XRP credentials...".to_string(),
            }));

            // --- FILE-BASED STORAGE ---
            let wallet_data = EncryptedWalletData {
                address: address.clone(),
                encrypted_phrase: encrypted,
                salt,
                iv,
            };

            // Using xrp_encrypt.json for asset isolation
            if let Err(e) = write_json("xrp_encrypt.json", &wallet_data) {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: format!("File System Error: {}", e),
                }));
                return;
            }

            let _ = ws_tx.try_send(WSCommand {
                command: "import_wallet".to_string(), // Backend likely expects this for XRP
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

            let _ = CHANNEL.xrp_wallet_process_tx.send(XRPWalletProcessState {
                import_wallet: None,
                create_wallet: None,
            });

            let _ = CHANNEL.xrp_modal_tx.send(XRPModalState {
                view_type: ActiveView::XRP,
                last_view: None,
            });
        }
        Ok(Err(e)) => {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: format!("Error: {}", e),
            }));
        }
        _ => {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: "Internal thread error".to_string(),
            }));
        }
    }
}
}