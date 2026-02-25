//src/ui/managebtc/btcsend/step2.rs
//dependent upon utils/send_amount_layout

use dioxus_native::prelude::*;
use crate::context::{BtcContext, GlobalContext};
use crate::utils::{format_token_amount, format_usd};
use crate::utils::send_amount_layout::SendAmountForm;

#[component]
pub fn view() -> Element {
    let btc_ctx = use_context::<BtcContext>();
    let global = use_context::<GlobalContext>();
    
    let mut sign_tx = btc_ctx.btc_sign_transaction;
    let rates = global.rates.read();
    let exchange_rate = rates.get("BTC/USD").copied().unwrap_or(0.0) as f64;

    let btc_balance = btc_ctx.bitcoin_wallet.read().0;

    let mut btc_in = use_signal(|| {
        sign_tx.read().send_transaction.as_ref()
            .and_then(|s| s.amount.clone())
            .unwrap_or_default()
    });
    
    let mut usd_in = use_signal(|| {
        let saved = sign_tx.read().send_transaction.as_ref()
            .and_then(|s| s.amount.clone())
            .unwrap_or_default();
        
        if let Ok(val) = saved.parse::<f64>() {
            format_usd(val * exchange_rate)
        } else {
            String::new()
        }
    });

    let mut clear_error = move || {
        sign_tx.with_mut(|state| {
            if let Some(ref mut send) = state.send_transaction {
                send.error = None;
            }
        });
    };

    let on_btc_input = move |evt: FormEvent| {
        let val = evt.value().replace(['\n', '\r'], "");
        btc_in.set(val.clone());
        clear_error();
        if let Ok(amount) = val.parse::<f64>() {
            usd_in.set(format_usd(amount * exchange_rate));
        } else {
            usd_in.set(String::new());
        }
    };

    let on_usd_input = move |evt: FormEvent| {
        let val = evt.value().replace(['\n', '\r'], "");
        usd_in.set(val.clone());
        clear_error();
        if let Ok(fiat) = val.parse::<f64>() {
            if exchange_rate > 0.0 {
                // Bitcoin uses 8 decimals
                btc_in.set(format_token_amount(fiat / exchange_rate, 8));
            }
        } else {
            btc_in.set(String::new());
        }
    };

    let on_next_click = move |_| {
        let amount_str = btc_in().trim().to_string();
        
        if amount_str.is_empty() {
            sign_tx.with_mut(|s| {
                if let Some(ref mut tx) = s.send_transaction {
                    tx.error = Some("ERR: AMOUNT_REQUIRED".to_string());
                }
            });
            return;
        }

        if let Ok(amount) = amount_str.parse::<f64>() {
            if amount <= 0.0 {
                 sign_tx.with_mut(|s| {
                    if let Some(ref mut tx) = s.send_transaction {
                        tx.error = Some("ERR: MIN_VALUE_REQUIRED".to_string());
                    }
                });
            } else if amount > btc_balance {
                 sign_tx.with_mut(|s| {
                    let err = format!("ERR: INSUFFICIENT_FUNDS // MAX: {} BTC", format_token_amount(btc_balance, 8));
                    if let Some(ref mut tx) = s.send_transaction {
                        tx.error = Some(err);
                    }
                });
            } else {
                sign_tx.with_mut(|s| {
                    if let Some(ref mut tx) = s.send_transaction {
                        tx.amount = Some(format_token_amount(amount, 8));
                        tx.step = 3; 
                        tx.error = None;
                    }
                });
            }
        } else {
            sign_tx.with_mut(|s| {
                if let Some(ref mut tx) = s.send_transaction {
                    tx.error = Some("ERR: INVALID_NUMBER_FORMAT".to_string());
                }
            });
        }
    };

    let current_error = sign_tx.read().send_transaction.as_ref()
        .and_then(|s| s.error.clone());

    // Inject the Bitcoin-specific data into the shared layout
    rsx! {
        SendAmountForm {
            asset_label: "BTC".to_string(),
            network_label: "BITCOIN_MAINNET".to_string(),
            show_fiat: true,
            amount_in: btc_in,
            fiat_in: usd_in,
            formatted_balance: format_token_amount(btc_balance, 8),
            exchange_rate: exchange_rate,
            current_error: current_error,
            on_amount_input: on_btc_input,
            on_fiat_input: on_usd_input,
            on_next_click: on_next_click,
        }
    }
}