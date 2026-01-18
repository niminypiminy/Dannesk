// src/ui/managexrp/xrpsend/step2.rs

use dioxus::prelude::*;
use crate::context::{XrpContext, GlobalContext, RlusdContext, EuroContext};

/// Helper to truncate a float to a string with max N decimals without rounding.
fn truncate_to_string(val: f64, decimals: usize) -> String {
    let s = format!("{:.1$}", val, decimals);
    if let Some(dot_idx) = s.find('.') {
        let integer_part = &s[..dot_idx];
        let fractional_part = &s[dot_idx + 1..];
        
        // Remove unnecessary trailing zeros
        let trimmed_fraction = fractional_part.trim_end_matches('0');
        
        // If we still want some decimals (like .00 for USD), 
        // we can set a minimum here. For now, let's just make it 
        // look like currency if it's likely a USD value.
        if trimmed_fraction.is_empty() {
            // This ensures "1" becomes "1.00"
            format!("{}.00", integer_part)
        } else if trimmed_fraction.len() == 1 {
            // This ensures "1.1" becomes "1.10"
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
    let xrp_ctx = use_context::<XrpContext>();
    let global = use_context::<GlobalContext>();
    let rlusd_ctx = use_context::<RlusdContext>();
    let euro_ctx = use_context::<EuroContext>();
    
    let mut sign_transaction = xrp_ctx.sign_transaction;
    let rates = global.rates.read();
    let exchange_rate = rates.get("XRP/USD").copied().unwrap_or(0.0) as f64;

    // Get the current asset selection from state
    let asset = sign_transaction.read().send_transaction.as_ref()
        .map(|s| s.asset.clone())
        .unwrap_or_else(|| "XRP".to_string());

    // Branch balance reading based on the asset string
    let (balance, asset_label) = match asset.as_str() {
        "RLUSD" => {
            let (bal, _, _) = rlusd_ctx.rlusd.read().clone();
            (bal, "RLUSD")
        },
        "EURO" => {
            let (bal, _, _) = euro_ctx.euro.read().clone();
            (bal, "EURO")
        },
        _ => {
            let (bal, _, _) = xrp_ctx.wallet_balance.read().clone();
            (bal, "XRP")
        },
    };

    let mut xrp_in = use_signal(|| {
        sign_transaction.read()
            .send_transaction.as_ref()
            .and_then(|s| s.amount.clone())
            .unwrap_or_default()
    });
    
    let mut usd_in = use_signal(|| {
        if asset != "XRP" { return String::new(); }
        let saved_xrp = sign_transaction.read()
            .send_transaction.as_ref()
            .and_then(|s| s.amount.clone())
            .unwrap_or_default();
        
        if let Ok(val) = saved_xrp.parse::<f64>() {
            truncate_to_string(val * exchange_rate, 6)
        } else {
            String::new()
        }
    });

    let mut clear_error = move || {
        sign_transaction.with_mut(|state| {
            if let Some(ref mut send) = state.send_transaction {
                send.error = None;
            }
        });
    };

    // Clone asset for input closure
    let asset_for_input = asset.clone();
    let on_xrp_input = move |evt: FormEvent| {
        let val = evt.value();
        xrp_in.set(val.clone());
        clear_error();

        if asset_for_input == "XRP" {
            if let Ok(amount) = val.parse::<f64>() {
                let usd = amount * exchange_rate;
                usd_in.set(truncate_to_string(usd, 6));
            } else {
                usd_in.set("".to_string());
            }
        }
    };

    let on_usd_input = move |evt: FormEvent| {
        let val = evt.value();
        usd_in.set(val.clone());
        clear_error();

        if let Ok(amount) = val.parse::<f64>() {
            if exchange_rate > 0.0 {
                let xrp = amount / exchange_rate;
                xrp_in.set(truncate_to_string(xrp, 6));
            }
        } else {
            xrp_in.set("".to_string());
        }
    };

    // Clone asset for next click closure
    let asset_for_next = asset.clone();
    let asset_label_for_next = asset_label; // Static str is Copy
    let on_next_click = move |_| {
        let amount_str = xrp_in().trim().to_string();
        
        if amount_str.is_empty() {
            sign_transaction.with_mut(|state| {
                if let Some(ref mut send) = state.send_transaction {
                    send.error = Some("Amount cannot be empty.".to_string());
                }
            });
            return;
        }

        if let Ok(amount) = amount_str.parse::<f64>() {
            let reserve = if asset_for_next == "XRP" { 1.0 } else { 0.0 };
            
            if amount <= 0.0 {
                 sign_transaction.with_mut(|state| {
                    if let Some(ref mut send) = state.send_transaction {
                        send.error = Some("Amount must be greater than zero.".to_string());
                    }
                });
            } else if amount > balance - reserve {
                 sign_transaction.with_mut(|state| {
                    let err = if asset_for_next == "XRP" {
                        "Insufficient funds: 1 XRP reserve required.".to_string()
                    } else {
                        format!("Insufficient funds: {} {} available.", truncate_to_string(balance, 6), asset_label_for_next)
                    };
                    if let Some(ref mut send) = state.send_transaction {
                        send.error = Some(err);
                    }
                });
            } else {
                sign_transaction.with_mut(|state| {
                    if let Some(ref mut send) = state.send_transaction {
                        send.amount = Some(truncate_to_string(amount, 6));
                        send.step = 3; 
                        send.error = None;
                    }
                });
            }
        } else {
            sign_transaction.with_mut(|state| {
                if let Some(ref mut send) = state.send_transaction {
                    send.error = Some("Invalid amount format.".to_string());
                }
            });
        }
    };

    let current_error = sign_transaction.read()
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

    let formatted_balance = truncate_to_string(balance, 6);

rsx! {
        div {
            style: "display: flex; 
                flex-direction: column; 
                width: 100%; 
                align-items: center;",

            div { style: "font-size: 1.5rem; margin: 0; margin-bottom: 1rem;", "Enter Amount" }
            div { 
                style: "font-size: 1rem; color: #888; margin-bottom: 1.5rem;", 
                if asset == "XRP" { "Enter the amount in XRP or USD." } else { "Enter the amount of {asset_label} to send." }
            }

            // === Primary Input ===
            div { style: "width: 100%; max-width: 25rem;",
                label { style: "font-size: 0.875rem; margin-bottom: 0.25rem; display: block;", "Amount ({asset_label})" }
                input {
                    style: "width: 100%; height: 2rem; padding: 0.3125rem; background-color: transparent; border: {get_border(xrp_in())}; border-radius: 0.25rem; font-size: 1.25rem; display: block;",
                    value: "{xrp_in()}",
                    oninput: on_xrp_input
                }
            }

            // === USD Input (Only rendered if XRP is selected) ===
            if asset == "XRP" {
                div { style: "width: 100%; max-width: 25rem; margin-top: 1rem;",
                    label { style: "font-size: 0.875rem; margin-bottom: 0.25rem; display: block;", "Amount (USD)" }
                    input {
                        style: "width: 100%; height: 2rem; padding: 0.3125rem; background-color: transparent; border: {get_border(usd_in())}; border-radius: 0.25rem; font-size: 1.25rem; display: block;",
                        value: "{usd_in()}",
                        oninput: on_usd_input
                    }
                }
            }

            // === Info Row (Flush under last input) ===
         // === Info Row ===
            div {
                style: "display: flex; flex-direction: column; width: 100%; max-width: 25rem; font-size: 0.875rem; color: #888; margin-top: 1rem;",
                
                div {
                    style: "display: flex; flex-direction: row; gap: 0.5rem; text-align: left;",
                    span { "Available Balance:" }
                    span { style: "font-weight: bold;", "{formatted_balance} {asset_label}" }
                }

                if asset == "XRP" {
                    div {
                        style: "display: flex; flex-direction: row; gap: 0.5rem; text-align: left;",
                        span { "Exchange Rate:" }
                        span { style: "font-weight: bold;", "${exchange_rate}" }
                    }
                }
            }

            if let Some(err) = current_error {
                div { style: "color: #ff4d4d; font-size: 0.875rem; font-weight: bold; margin-top: 0.5rem;", "{err}" }
            }

            button {
                style: "width: 8.75rem; height: 2.25rem; background-color: #333; color: white; border: none; 
        border-radius: 1.375rem; font-size: 1rem; display: flex; cursor: pointer; justify-content: center; align-items: center; margin-top: 3rem;",
                onclick: on_next_click,
                "Continue"
            }
        }
    }
}