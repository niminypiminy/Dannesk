// src/ui/managebtc/btcbalance.rs

use dioxus::prelude::*;
use crate::context::{GlobalContext, BtcContext};
use crate::utils::add_commas;
use crate::utils::btcsvg::BtcIcon; 
use crate::ui::managebtc::btcbalance::bitcoin_wallet_operations::BitcoinWalletOperations;
use crate::utils::upsvg::UpIcon;
use crate::utils::downsvg::DownIcon;

pub mod bitcoin_wallet_operations;


#[component]
pub fn view() -> Element {
    let global = use_context::<GlobalContext>();
    let btc_ctx = use_context::<BtcContext>();
    
    // SIGNALS
    let mut btc_modal = btc_ctx.btc_modal;
    let mut btc_sign_tx = btc_ctx.btc_sign_transaction;

    // DATA
    let (btc_amount, address, key_is_deleted) = btc_ctx.bitcoin_wallet.read().clone();
    let rates = global.rates.read();
    let btc_usd_rate = rates.get("BTC/USD").copied().unwrap_or(0.0) as f64;
    let (_, _, hide_balance) = global.theme_user.read().clone();

    let total_usd = btc_amount * btc_usd_rate;

    // Formatting for USD Display
    let (int_part, frac_part) = if hide_balance {
        ("****".to_string(), "".to_string())
    } else {
        let formatted_int = add_commas(total_usd.floor() as i64);
        let formatted_frac = format!(".{:02}", (total_usd.fract() * 100.0).floor() as i64);
        (formatted_int, formatted_frac)
    };

    // Formatting for BTC Display (8 decimal places for Bitcoin)
    let formatted_raw_btc = format!("{:.8}", btc_amount);

   // Async handlers
    let on_remove_wallet = {
        let ws_tx = global.ws_tx.clone();
        let address = address.clone();
        move |_| {
            if let Some(addr) = address.clone() {
                tokio::spawn(BitcoinWalletOperations::remove_wallet(addr, ws_tx.clone()));
            }
        }
    };

    let on_delete_key = {
        let address = address.clone();
        move |_| {
            if let Some(addr) = address.clone() {
                tokio::spawn(BitcoinWalletOperations::delete_key(addr));
            }
        }
    };

   let on_receive_click = move |_| {
    btc_modal.with_mut(|state| {
        // Bookmark XRP as the origin
        state.last_view = Some(crate::channel::BTCActiveView::BTC);
        state.view_type = crate::channel::BTCActiveView::Receive;
    });
};
     let on_send_click = move |_| {
        // 1. Set the bookmark
        btc_modal.with_mut(|state| {
        state.last_view = Some(crate::channel::BTCActiveView::BTC);
        state.view_type = crate::channel::BTCActiveView::Send; 

    });
    // 2. Initialize the Send state
        btc_sign_tx.with_mut(|state| {
            state.send_transaction = Some(crate::channel::BTCSignTransaction {
                step: 1,
                error: None,
                recipient: None, 
                amount: None,
                fee: "".to_string(),
                asset: "BTC".to_string()
            });
        });
    };

  
rsx! {
    div {
        // Parent: Using flex: 1 to fill the center space without conflicting with sidebars
        style: "display: flex; flex-direction: column; align-items: center; flex: 1; font-family: monospace; padding-top: 2rem; box-sizing: border-box;",

        div {
            // Inner Wrapper: Fixed width (24rem) to ensure no sidebar overlap and pixel-perfect centering
            style: "width: 24rem; flex-shrink: 0; display: flex; flex-direction: column; align-items: center; box-sizing: border-box;",

            // BLOCK 1: MAIN BALANCE
            div {
                style: "display: flex; justify-content: center; margin-bottom: 2.5rem; box-sizing: border-box;",
                h1 {
                    // CRITICAL: line-height: 1 ensures no fractional pixel leaks on older drivers
                    style: "display: flex; align-items: baseline; font-size: 6rem; line-height: 1; margin: 0; box-sizing: border-box;",
                    if !hide_balance {
                        span { style: "font-weight: bold; margin-right: 0.5rem;", "$" }
                        span { style: "font-weight: bold;", "{int_part}" }
                        span { style: "opacity: 0.8;", "{frac_part}" }
                    } else {
                        span { style: "font-weight: normal; letter-spacing: 0.5rem;", "****" }
                    }
                }
            }

            // BLOCK 2: BTC DETAILS
            div {
                style: "display: flex; flex-direction: column; align-items: center; margin-bottom: 2.5rem; box-sizing: border-box;",
                h5 {
                    style: "display: flex; flex-direction: column; align-items: center; opacity: 0.7; font-weight: normal; margin: 0; box-sizing: border-box;",
                    span { 
                        style: "display: flex; align-items: center; gap: 0.5rem; font-size: 1.25rem; margin-bottom: 0.25rem; box-sizing: border-box;", 
                        if hide_balance { "****" } else { "{formatted_raw_btc}" }
                        BtcIcon {}
                    }
                    span { style: "font-size: 0.9rem; opacity: 0.5; box-sizing: border-box;", "Rate: ${btc_usd_rate:.2}" }
                }
            }

            // BLOCK 3: ACTION BUTTONS
            div {
                style: "display: flex; flex-direction: row; justify-content: center; gap: 1rem; margin-bottom: 2.5rem; box-sizing: border-box;",
                button {
                    style: "display: flex; justify-content: center; align-items: center; width: 6rem; height: 3.5rem; border: 1px solid #444; border-radius: 2rem; background: none; color: #888; cursor: pointer; box-sizing: border-box;",
                    onclick: on_send_click,
                    UpIcon {}
                }
                button {
                    style: "display: flex; justify-content: center; align-items: center; width: 6rem; height: 3.5rem; border: 1px solid #444; border-radius: 2rem; background: none; color: #888; cursor: pointer; box-sizing: border-box;",
                    onclick: on_receive_click,
                    DownIcon {}
                }
            }

            // BLOCK 4: FOOTER LINKS
            div {
                style: "display: flex; flex-direction: row; justify-content: center; gap: 2.5rem; font-size: 0.9rem; box-sizing: border-box; padding: 1rem 0;",
                span { 
                    style: "text-decoration: underline; cursor: pointer; opacity: 0.6; box-sizing: border-box;", 
                    onclick: on_remove_wallet, 
                    "Remove Wallet" 
                }
                span { 
                    style: format!(
                        "{}",
                        if key_is_deleted { "color: #ffadadff;" } else { "text-decoration: underline; cursor: pointer; opacity: 0.6;" }
                    ),
                    onclick: move |e| { if !key_is_deleted { on_delete_key(e); } },
                    if key_is_deleted { "Key Purged" } else { "Delete Key" }
                }
            }
        }
    }
}
}