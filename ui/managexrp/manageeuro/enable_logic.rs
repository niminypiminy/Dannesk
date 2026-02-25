// src/ui/managereuro/enable_logic.rs

use tokio::sync::mpsc::Sender;
use crate::channel::{CHANNEL, WSCommand, ProgressState, SignTransactionState};
use zeroize::Zeroizing;
use arboard::Clipboard;

pub struct EuroEnableLogic;

impl EuroEnableLogic {
    pub async fn process(
        mode: String,
        passphrase: String,      
        mnemonic: String,        
        bip39_pass: String,      
        wallet_address: String,
        ws_tx: Sender<WSCommand>,
    ) {
        // 1. Secure inputs immediately
        let p_guard = Zeroizing::new(passphrase);
        let m_guard = Zeroizing::new(mnemonic);
        let b_guard = Zeroizing::new(bip39_pass);

        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 0.0,
            message: "Enabling Euro Trustline...".to_string(),
        }));

        // 2. Prepare Optional Data (BIP39)
        let bip39_opt = if b_guard.trim().is_empty() { 
            None 
        } else { 
            Some(b_guard.clone()) 
        };

        // 3. Enforce XOR Logic (Passphrase vs Seed)
        let (passphrase, seed) = match mode.as_str() {
            "passphrase" => {
                let p = if p_guard.is_empty() { None } else { Some(p_guard.clone()) };
                (p, None) 
            },
            "seed" => {
                let s = if m_guard.trim().is_empty() { None } else { Some(m_guard.clone()) };
                (None, s) 
            },
            _ => (None, None), 
        };

        // 4. Construct Command for Enabling Euro
        let cmd = WSCommand {
            command: "submit_transaction".to_string(),
            wallet: Some(wallet_address),
            recipient: None,
            amount: None,
            passphrase,
            seed,
            trustline_limit: Some("1000000".to_string()),
            fee: None,
            tx_type: Some("trustset_euro".to_string()), 
            taker_pays: None,
            taker_gets: None,
            flags: None,
            wallet_type: None,
            bip39: bip39_opt,
        };

        // 5. Dispatch
        match ws_tx.try_send(cmd) {
            Ok(_) => {
                // Clear clipboard for security
                if let Ok(mut ctx) = Clipboard::new() {
                    let _ = ctx.set_text("");
                }
                // Reset transaction state
                let _ = CHANNEL.sign_transaction_tx.send(SignTransactionState {
                    send_transaction: None, 
                });
                // We don't necessarily need to change view here, 
                // just let the progress update finish.
            }
            Err(e) => {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: format!("Dispatch Error: {}", e),
                }));
            }
        }
    }
}