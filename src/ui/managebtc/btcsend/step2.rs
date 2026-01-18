// src/ui/managebtc/btcsend/step2.rs

use dioxus::prelude::*;
use crate::context::{BtcContext, GlobalContext};

/// Helper to truncate a float to a string with max N decimals without rounding.
fn truncate_to_string(val: f64, decimals: usize) -> String {
    let s = format!("{:.1$}", val, decimals);
    if let Some(dot_idx) = s.find('.') {
        let integer_part = &s[..dot_idx];
        let fractional_part = &s[dot_idx + 1..];
        let trimmed_fraction = fractional_part.trim_end_matches('0');
        
        if trimmed_fraction.is_empty() {
            format!("{}.00", integer_part)
        } else if trimmed_fraction.len() == 1 {
            format!("{}.{}0", integer_part, trimmed_fraction)
        } else {
            format!("{}.{}", integer_part, trimmed_fraction)
        }
    } else {
        format!("{}.00", s)
    }
}

#[component]
pub fn view() -> Element {
    let btc_ctx = use_context::<BtcContext>();
    let global = use_context::<GlobalContext>();
    
    let mut btc_sign_transaction = btc_ctx.btc_sign_transaction;
    let rates = global.rates.read();
    let exchange_rate = rates.get("BTC/USD").copied().unwrap_or(0.0) as f64;

    // Simplified balance reading for BTC only
    let (btc_balance, _address, _key_deleted) = btc_ctx.bitcoin_wallet.read().clone();

    let mut btc_in = use_signal(|| {
        btc_sign_transaction.read()
            .send_transaction.as_ref()
            .and_then(|s| s.amount.clone())
            .unwrap_or_default()
    });
    
    let mut usd_in = use_signal(|| {
        let saved_btc = btc_sign_transaction.read()
            .send_transaction.as_ref()
            .and_then(|s| s.amount.clone())
            .unwrap_or_default();
        
        if let Ok(val) = saved_btc.parse::<f64>() {
            truncate_to_string(val * exchange_rate, 2) // USD usually 2 decimals
        } else {
            String::new()
        }
    });

    let mut clear_error = move || {
        btc_sign_transaction.with_mut(|state| {
            if let Some(ref mut send) = state.send_transaction {
                send.error = None;
            }
        });
    };

    let on_btc_input = move |evt: FormEvent| {
        let val = evt.value();
        btc_in.set(val.clone());
        clear_error();

        if let Ok(amount) = val.parse::<f64>() {
            let usd = amount * exchange_rate;
            usd_in.set(truncate_to_string(usd, 2));
        } else {
            usd_in.set("".to_string());
        }
    };

    let on_usd_input = move |evt: FormEvent| {
        let val = evt.value();
        usd_in.set(val.clone());
        clear_error();

        if let Ok(amount) = val.parse::<f64>() {
            if exchange_rate > 0.0 {
                let btc = amount / exchange_rate;
                btc_in.set(truncate_to_string(btc, 8)); // BTC supports up to 8 decimals
            }
        } else {
            btc_in.set("".to_string());
        }
    };

    let on_next_click = move |_| {
        let amount_str = btc_in().trim().to_string();
        
        if amount_str.is_empty() {
            btc_sign_transaction.with_mut(|state| {
                if let Some(ref mut send) = state.send_transaction {
                    send.error = Some("Amount cannot be empty.".to_string());
                }
            });
            return;
        }

        if let Ok(amount) = amount_str.parse::<f64>() {
            if amount <= 0.0 {
                 btc_sign_transaction.with_mut(|state| {
                    if let Some(ref mut send) = state.send_transaction {
                        send.error = Some("Amount must be greater than zero.".to_string());
                    }
                });
            } else if amount > btc_balance {
                 btc_sign_transaction.with_mut(|state| {
                    let err = format!("Insufficient funds: {} BTC available.", truncate_to_string(btc_balance, 8));
                    if let Some(ref mut send) = state.send_transaction {
                        send.error = Some(err);
                    }
                });
            } else {
                btc_sign_transaction.with_mut(|state| {
                    if let Some(ref mut send) = state.send_transaction {
                        send.amount = Some(truncate_to_string(amount, 8));
                        send.step = 3; 
                        send.error = None;
                    }
                });
            }
        } else {
            btc_sign_transaction.with_mut(|state| {
                if let Some(ref mut send) = state.send_transaction {
                    send.error = Some("Invalid amount format.".to_string());
                }
            });
        }
    };

    let current_error = btc_sign_transaction.read()
        .send_transaction.as_ref()
        .and_then(|s| s.error.clone());

    let get_border = |val: String| -> &'static str {
        let trimmed = val.trim();
        if !trimmed.is_empty() && trimmed.parse::<f64>().is_ok() {
             "1px solid #10B981" 
        } else if !trimmed.is_empty() {
             "1px solid #ef4444"
        } else {
             "1px solid #444"
        }
    };

   rsx! {
    div {
        style: "display: flex; 
            flex-direction: column; 
            width: 100%; 
            align-items: center;",

        div { style: "font-size: 1.5rem; margin: 0; margin-bottom: 1rem;", "Enter Amount" }
        div { 
            style: "font-size: 1rem; color: #888; margin-bottom: 1.5rem;", 
            "Enter the amount in BTC or USD." 
        }

        // === Primary Input (BTC) ===
        div { style: "width: 100%; max-width: 25rem;",
            label { style: "font-size: 0.875rem; margin-bottom: 0.25rem; display: block;", "Amount (BTC)" }
            input {
                style: "width: 100%; height: 2rem; padding: 0.3125rem; background-color: transparent;  border: {get_border(btc_in())}; border-radius: 0.25rem; font-size: 1.25rem; display: block;",
                value: "{btc_in()}",
                oninput: on_btc_input
            }
        }

        // === USD Input ===
        div { style: "width: 100%; max-width: 25rem; margin-top: 1rem;",
            label { style: "font-size: 0.875rem; margin-bottom: 0.25rem; display: block;", "Amount (USD)" }
            input {
                style: "width: 100%; height: 2rem; padding: 0.3125rem; background-color: transparent; border: {get_border(usd_in())}; border-radius: 0.25rem; font-size: 1.25rem; display: block;",
                value: "{usd_in()}",
                oninput: on_usd_input
            }
        }

        // === Info Row (Flush left under inputs) ===
        div {
            style: "display: flex; flex-direction: column; width: 100%; max-width: 25rem; font-size: 0.875rem; color: #888; margin-top: 1rem;",
            
            div {
                style: "display: flex; flex-direction: row; gap: 0.5rem; text-align: left;",
                span { "Available Balance:" }
                span { style: "font-weight: bold;", "{truncate_to_string(btc_balance, 8)} BTC" }
            }

            div {
                style: "display: flex; flex-direction: row; gap: 0.5rem; text-align: left;",
                span { "Exchange Rate:" }
                span { style: "font-weight: bold;", "${exchange_rate}" }
            }
        }

        if let Some(err) = current_error {
            div { style: "color: #ff4d4d; font-size: 0.875rem; font-weight: bold; margin-top: 0.5rem;", "{err}" }
        }

        button {
            style: "width: 8.75rem; height: 2.25rem; background-color: #333; color: white; border: none; 
                border-radius: 1.375rem; font-size: 1rem; display: flex; cursor: pointer; 
                justify-content: center; align-items: center; margin-top: 3rem;",
            onclick: on_next_click,
            "Continue"
        }
    }
}
}