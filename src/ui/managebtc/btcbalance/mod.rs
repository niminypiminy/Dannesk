// src/ui/managebtc/btcbalance.rs

use dioxus_native::prelude::*;
use crate::context::{GlobalContext, BtcContext};
use crate::utils::add_commas;
use crate::ui::managebtc::btcbalance::bitcoin_wallet_operations::BitcoinWalletOperations;
use crate::utils::styles::terminal_action;
use crate::utils::balance_layout::render_balance_layout;

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
    
    // --- CALCULATE UI VALUES BEFORE RSX ---
    let status_color = if key_is_deleted { "var(--status-warn)" } else { "var(--status-ok)" };
    let status_text = if key_is_deleted { "PROTECTED // KEY_OFF_DEVICE" } else { "ACTIVE // KEY_ON_DEVICE" };
    // --------------------------------------

    let rates = global.rates.read();
    let btc_usd_rate = rates.get("BTC/USD").copied().unwrap_or(0.0) as f64;
    let (_, hide_balance) = global.theme_user.read().clone();

    let total_usd = btc_amount * btc_usd_rate;
    
    let (int_part, frac_part) = if hide_balance {
        ("****".to_string(), "".to_string())
    } else {
        (
            add_commas(total_usd.floor() as i64), 
            format!(".{:02}", (total_usd.fract() * 100.0).round() as i64)
        )
    };

    let formatted_raw_btc = if hide_balance { "****".to_string() } else { format!("{:.8}", btc_amount) };

    // --- TERMINAL ACTIONS ---
    let send_btn = terminal_action("SEND", true, move |_| {
        btc_modal.with_mut(|state| {
            state.last_view = Some(crate::channel::BTCActiveView::BTC);
            state.view_type = crate::channel::BTCActiveView::Send; 
        });
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
    });

    let receive_btn = terminal_action("RECEIVE", true, move |_| {
        btc_modal.with_mut(|state| {
            state.last_view = Some(crate::channel::BTCActiveView::BTC);
            state.view_type = crate::channel::BTCActiveView::Receive;
        });
    });

    let purge_btn = terminal_action("PURGE", true, {
        let ws_tx = global.ws_tx.clone();
        let addr = address.clone();
        move |_| { 
            if let Some(a) = addr.clone() { 
                tokio::spawn(BitcoinWalletOperations::remove_wallet(a, ws_tx.clone())); 
            } 
        }
    });

    let delete_btn = terminal_action("DELETE_KEY", true, {
        let addr = address.clone();
        move |_| { 
            if let Some(a) = addr.clone() { 
                tokio::spawn(BitcoinWalletOperations::delete_key(a)); 
            } 
        }
    });

    let optional_delete_btn = if !key_is_deleted { Some(delete_btn) } else { None };

    render_balance_layout(
        "BTC".to_string(),
        int_part,
        frac_part,
        formatted_raw_btc,
        status_color.to_string(),
        status_text.to_string(),
        "BITCOIN // CORE_MAINNET".to_string(),
        send_btn,
        receive_btn,
        purge_btn,
        optional_delete_btn,
    )
}