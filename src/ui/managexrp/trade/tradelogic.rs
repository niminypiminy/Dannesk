// src/ui/managexrp/trade/tradelogic.rs

use tokio::sync::mpsc::Sender;
use crate::channel::{CHANNEL, WSCommand, ProgressState, XRPModalState, SignTradeState, ActiveView};
use zeroize::Zeroizing;
use arboard::Clipboard;

pub struct TradeLogic;

impl TradeLogic {
    #[allow(clippy::too_many_arguments)]
    pub async fn process(
        mode: String,
        passphrase: String,
        mnemonic: String,
        bip39_pass: String,
        // Trade specific variables
        base_asset: String,
        quote_asset: String,
        amount: String,
        limit_price: String,
        flags: Vec<String>,
        // Wallet / System
        wallet_address: String,
        ws_tx: Sender<WSCommand>,
        last_view: Option<ActiveView>,
    ) {
        // 1. Secure inputs immediately
        let p_guard = Zeroizing::new(passphrase);
        let m_guard = Zeroizing::new(mnemonic);
        let b_guard = Zeroizing::new(bip39_pass);

        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 0.0,
            message: "Preparing Trade...".to_string(),
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

        // 4. Calculate Trade Values
        // Amount = How much Base asset we want
        // Price = How much Quote asset per 1 Base asset
        let amount_f = amount.parse::<f64>().unwrap_or(0.0);
        let price_f = limit_price.parse::<f64>().unwrap_or(0.0);
        
        // Offer Amount = Total Quote asset required (Amount * Price)
        let offer_amount_val = amount_f * price_f;
        let offer_amount = offer_amount_val.to_string();

        // 5. Structure TakerPays vs TakerGets
        // In this UI flow, we are "Buying" Base using Quote.
        // TakerPays: What the network pays me (I receive Base)
        // TakerGets: What I pay the network (I give Quote)
        // Note: The WSCommand struct expects tuples for these fields based on your egui code.
        let taker_pays = Some((amount.clone(), base_asset));
        let taker_gets = Some((offer_amount, quote_asset));

        // 6. Construct Command
        let cmd = WSCommand {
            command: "submit_transaction".to_string(),
            wallet: Some(wallet_address),
            recipient: None,
            amount: None, 
            passphrase,
            seed,
            bip39: bip39_opt,
            trustline_limit: None,
            fee: None,
            tx_type: Some("offer_create".to_string()),
            taker_pays,
            taker_gets,
            flags: Some(flags),
            wallet_type: None, 
        };


        // 7. Dispatch
        match ws_tx.try_send(cmd) {
            Ok(_) => {
                if let Ok(mut ctx) = Clipboard::new() {
                    let _ = ctx.set_text("");
                }
                
                let _ = CHANNEL.trade_tx.send(SignTradeState {
                    send_trade: None,
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