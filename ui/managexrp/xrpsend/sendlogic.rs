// src/ui/managexrp/xrpsend/sendlogic.rs

use tokio::sync::mpsc::Sender;
use crate::channel::{CHANNEL, WSCommand, ProgressState, XRPModalState, SignTransactionState, ActiveView};
use zeroize::Zeroizing;
use arboard::Clipboard;

pub struct XRPSendLogic;

impl XRPSendLogic {
    pub async fn process(
        mode: String,
        passphrase: String,      
        mnemonic: String,        
        bip39_pass: String,      
        recipient: String,
        amount: String,
        wallet_address: String,
        asset: String,
        ws_tx: Sender<WSCommand>,
        last_view: Option<ActiveView>, 
    ) {
        // 1. Secure inputs immediately
        // Note: Zeroizing::new() takes ownership and zeros the input on drop.
        let p_guard = Zeroizing::new(passphrase);
        let m_guard = Zeroizing::new(mnemonic);
        let b_guard = Zeroizing::new(bip39_pass);

        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 0.0,
            message: "Initiating transaction...".to_string(),
        }));

        // 2. Prepare Optional Data (BIP39)
        // FIX: We clone the *guard* (Zeroizing wrapper), not the raw string inside.
        let bip39_opt = if b_guard.trim().is_empty() { 
            None 
        } else { 
            Some(b_guard.clone()) 
        };

        // 3. Enforce XOR Logic (Passphrase vs Seed)
        // FIX: We move or clone the Zeroizing guards directly.
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

        // 4. Construct Command
        // The fields in WSCommand are now Option<Zeroizing<String>>
        let cmd = WSCommand {
            command: "submit_transaction".to_string(),
            wallet: Some(wallet_address), 
            recipient: Some(recipient),
            amount: Some(amount),
            passphrase, 
            seed,       
            bip39: bip39_opt,
            trustline_limit: None,
            fee: None,
            tx_type: Some("payment".to_string()),
            taker_pays: None,
            taker_gets: None,
            flags: None,
            wallet_type: Some(asset),
        };

        // 5. Dispatch
        match ws_tx.try_send(cmd) {
            Ok(_) => {
                if let Ok(mut ctx) = Clipboard::new() {
                    let _ = ctx.set_text("");
                }
                let _ = CHANNEL.sign_transaction_tx.send(SignTransactionState {
                    send_transaction: None, 
                });
                let _ = CHANNEL.xrp_modal_tx.send(XRPModalState {
                    view_type: last_view.unwrap_or(ActiveView::XRP), 
                    last_view: None,
                });
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