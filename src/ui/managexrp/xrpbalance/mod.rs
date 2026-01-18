// src/ui/render_xrp_balance.rs

use dioxus::prelude::*;
use crate::context::{GlobalContext, XrpContext};
use crate::utils::add_commas;
use crate::ui::managexrp::xrpbalance::wallet_operations::WalletOperations;
use crate::utils::xrpsvg::XrpIcon;
use crate::utils::upsvg::UpIcon;
use crate::utils::downsvg::DownIcon;


pub mod wallet_operations;

pub fn render_xrp_balance() -> Element {
    let global = use_context::<GlobalContext>();
    let xrp_ctx = use_context::<XrpContext>();
    let mut sign_tx = xrp_ctx.sign_transaction;
    let mut xrp_modal = xrp_ctx.xrp_modal;

let (xrp_amount, address, key_is_deleted) = xrp_ctx.wallet_balance.read().clone();

    let rates = global.rates.read();
    let xrp_usd_rate = rates.get("XRP/USD").copied().unwrap_or(0.0) as f64;

let (is_dark, _, hide_balance) = global.theme_user.read().clone();

    let total_usd = xrp_amount * xrp_usd_rate;

     let (int_part, frac_part) = if hide_balance {
        ("****".to_string(), "".to_string())
    } else {
        let formatted_int = add_commas(total_usd.floor() as i64);
        // Ensure we get exactly .XX
        let formatted_frac = format!(".{:02}", (total_usd.fract() * 100.0).floor() as i64);
        (formatted_int, formatted_frac)
    };

        let formatted_raw_xrp = format!("{:.6}", xrp_amount);


    // Async handlers
    let on_remove_wallet = {
        let ws_tx = global.ws_tx.clone();
        let address = address.clone();
        move |_| {
            if let Some(addr) = address.clone() {
                tokio::spawn(WalletOperations::remove_wallet(addr, ws_tx.clone()));
            }
        }
    };

    let on_delete_key = {
        let address = address.clone();
        move |_| {
            if let Some(addr) = address.clone() {
                tokio::spawn(WalletOperations::delete_key(addr));
            }
        }
    };

   let on_receive_click = move |_| {
    xrp_modal.with_mut(|state| {
        // Bookmark XRP as the origin
        state.last_view = Some(crate::channel::ActiveView::XRP);
        state.view_type = crate::channel::ActiveView::Receive;
    });
};

    let on_send_click = move |_| {
        // 1. Set the bookmark
        xrp_modal.with_mut(|state| {
        state.last_view = Some(crate::channel::ActiveView::XRP);
        state.view_type = crate::channel::ActiveView::Send; 
    });
    // 2. Initialize the Send state
        sign_tx.with_mut(|state| {
            state.send_transaction = Some(crate::channel::SignTransaction {
                step: 1,
                error: None,
                recipient: None, 
                amount: None,
                asset: "XRP".to_string()
            });
        });
    };



rsx! {
    div {
        // Use flex-grow: 1 to fill space between sidebars without forcing a 100% width conflict
        style: "display: flex; flex-direction: column; align-items: center; flex: 1; font-family: monospace; padding-top: 2rem; box-sizing: border-box;",

        div {
            // Constrain width to 24rem (well within the 32rem max) to ensure no sidebar overlap
            // Using flex-shrink: 0 prevents Taffy from squishing this container
            style: "width: 24rem; flex-shrink: 0; display: flex; flex-direction: column; align-items: center; box-sizing: border-box;",

            // BLOCK 1: MAIN BALANCE
            div {
                style: "display: flex; justify-content: center; margin-bottom: 2.5rem;",
                h1 {
                    style: "display: flex; align-items: baseline; font-size: 6rem; line-height: 1; margin: 0;",
                    if !hide_balance {
                        span { style: "font-weight: bold; margin-right: 0.5rem;", "$" }
                        span { style: "font-weight: bold;", "{int_part}" }
                        span { style: "opacity: 0.8;", "{frac_part}" }
                    } else {
                        span { style: "font-weight: normal; letter-spacing: 0.5rem;", "****" }
                    }
                }
            }

            // BLOCK 2: DETAILS
            div {
                style: "display: flex; flex-direction: column; align-items: center; margin-bottom: 2.5rem;",
                h5 {
                    style: "display: flex; flex-direction: column; align-items: center; opacity: 0.7; font-weight: normal; margin: 0;",
                    span { 
                        style: "display: flex; align-items: center; gap: 0.5rem; font-size: 1.25rem; margin-bottom: 0.25rem;", 
                        if hide_balance { "****" } else { "{formatted_raw_xrp}" }
                        XrpIcon { dark: is_dark }
                    }
                    span { style: "font-size: 0.9rem; opacity: 0.5;", "Rate: ${xrp_usd_rate:.4}" }
                }
            }

            // BLOCK 3: ACTION BUTTONS
            div {
                style: "display: flex; flex-direction: row; justify-content: center; gap: 1rem; margin-bottom: 2.5rem;",
                button {
                    style: "display: flex; justify-content: center; align-items: center; width: 6rem; height: 3.5rem; border: 1px solid #444; border-radius: 2rem; background: none; color: #888; cursor: pointer;",
                    onclick: on_send_click,
                    UpIcon {}
                }
                button {
                    style: "display: flex; justify-content: center; align-items: center; width: 6rem; height: 3.5rem; border: 1px solid #444; border-radius: 2rem; background: none; color: #888; cursor: pointer;",
                    onclick: on_receive_click,
                    DownIcon {}
                }
            }

            // BLOCK 4: FOOTER LINKS
            div {
                style: "display: flex; flex-direction: row; justify-content: center; gap: 2.5rem; font-size: 0.9rem; padding: 1rem 0;",
                span { 
                    style: "text-decoration: underline; cursor: pointer; opacity: 0.6;", 
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