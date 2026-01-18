use crate::channel::{CHANNEL, ProgressState};
use crate::decrypt::decrypt_data;
use crate::utils::json_storage::read_json; // Import your utility
use xrpl::wallet::Wallet;
use zeroize::{Zeroizing}; 
use bip39::{Mnemonic, Language};
use ripple_address_codec::{encode_seed, Ed25519};
use serde::Deserialize;

type Entropy = [u8; 16];

// Matches the structure used in xrpimportlogic
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
) -> Result<Wallet, String> {
    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
        progress: 0.3,
        message: "Authenticating wallet".to_string(),
    }));

    let mnemonic_phrase: Zeroizing<String> = match (passphrase, seed) {
        (None, Some(s)) => s, 
        (Some(p), None) => {
            // --- NEW FILE-BASED AUTHENTICATION ---
            let stored_data: EncryptedWalletData = read_json("xrp_encrypt.json").map_err(|e| {
                let err_msg = format!("Error: XRP credentials not found: {}", e);
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: "Error: Could not find encrypted credentials.".to_string(),
                }));
                err_msg
            })?;

            // Security check: ensure the file matches the requested wallet
            if stored_data.address != wallet_address {
                let err_msg = "Error: Stored data address mismatch".to_string();
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: err_msg.clone(),
                }));
                return Err(err_msg);
            }
            
            let raw_decrypted = decrypt_data(
                p.clone(), 
                stored_data.encrypted_phrase, 
                stored_data.salt, 
                stored_data.iv
            ).map_err(|_| {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: "Error: Decryption failed (Incorrect passphrase)".to_string(),
                }));
                "Error: Decryption failed".to_string()
            })?;

            Zeroizing::new(raw_decrypted)
        }
        _ => {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: "Error: Must provide either passphrase or seed".to_string(),
            }));
            return Err("Error: Must provide either passphrase or seed".to_string());
        }
    };

    // Use .as_str() to access the protected memory
    let mnemonic = Mnemonic::parse_in_normalized(Language::English, mnemonic_phrase.as_str()).map_err(|_| {
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: "Error: Invalid mnemonic".to_string(),
        }));
        "Error: Invalid mnemonic".to_string()
    })?;

    let seed_passphrase = bip39.as_deref().map(|s| s.as_str()).unwrap_or("");   
    let bip39_seed = mnemonic.to_seed(seed_passphrase);

    let entropy_slice: Entropy = bip39_seed[0..16]
        .try_into()
        .map_err(|_| {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: "Error: Invalid BIP39 seed length".to_string(),
            }));
            "Error: Invalid BIP39 seed length".to_string()
        })?;

    let base58_seed = encode_seed(&entropy_slice, &Ed25519);

    Wallet::new(&base58_seed, 0).map_err(|_| {
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: "Error: Invalid seed".to_string(),
        }));
        "Error: Invalid seed".to_string()
    })
}