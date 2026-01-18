// src/ui/managebtc/btcsend/step4.rs

use dioxus::prelude::*;
use crate::context::{BtcContext, GlobalContext};

#[component]
pub fn view() -> Element {
    let btc_ctx = use_context::<BtcContext>();
    let global = use_context::<GlobalContext>();
    
    let mut btc_sign_transaction = btc_ctx.btc_sign_transaction;
    let rates = global.rates.read();
    let exchange_rate = rates.get("BTC/USD").copied().unwrap_or(0.0) as f64;
    
    let sign_state = btc_sign_transaction.read();
    let send_data = sign_state.send_transaction.as_ref();

    let recipient = send_data.and_then(|s| s.recipient.clone()).unwrap_or_else(|| "N/A".into());
    let amount = send_data.and_then(|s| s.amount.clone()).unwrap_or_else(|| "0".into());
    let fee = send_data.and_then(|s| Some(s.fee.clone())).unwrap_or_else(|| "0".into());
    
    let usd_amount = if let Ok(amt) = amount.parse::<f64>() {
        format!("{:.2}", amt * exchange_rate)
    } else {
        "0.00".into()
    };

    let on_confirm_click = move |_| {
        btc_sign_transaction.with_mut(|state| {
            if let Some(ref mut send) = state.send_transaction {
                send.step = 5;
                send.error = None;
            }
        });
    };

   rsx! {
        div {
            // Updated to match XRP flex centering
            style: "display: flex; flex-direction: column; width: 100%; align-items: center;",

            // Grid container - 33rem max width to match XRP layout
            div {
                style: "width: 100%; max-width: 33rem; background-color: #1a1a1a; border: 1px solid #333; border-radius: 0.25rem; display: flex; flex-direction: column;",
                
                ReviewRow { label: "Recipient".to_string(), value: recipient, is_alt: false }
                ReviewRow { label: "Amount (BTC)".to_string(), value: format!("{amount} BTC"), is_alt: true }
                ReviewRow { label: "Amount (USD)".to_string(), value: format!("${usd_amount}"), is_alt: false }
                ReviewRow { label: "Fee (Satoshis)".to_string(), value: format!("{fee} sats"), is_alt: true }
                ReviewRow { 
                    label: "Network".to_string(), 
                    value: "Bitcoin".to_string(), 
                    is_alt: false 
                }
            }

            // Warning Text - Adjusted padding and font-size to match XRP
            div { 
                style: "width: 100%; max-width: 33rem; padding: 1.5rem 0;",
                p { 
                    style: "font-size: 0.875rem; color: #777; text-align: center; font-family: monospace; line-height: 1.4; margin: 0;",
                    "Verify the recipient address carefully. Bitcoin transactions cannot be undone."
                }
            }

            // Button - EXACT COPY of XRP styling (pill shape, specific dimensions)
            button {
                style: "width: 8.75rem; height: 2.25rem; background-color: #333; color: white; border: none; border-radius: 1.375rem; font-size: 1rem; display: flex; cursor: pointer; justify-content: center; align-items: center; margin-top: 1rem;",
                onclick: on_confirm_click,
                "Continue"
            }
        }
    }
}

#[component]
fn ReviewRow(label: String, value: String, is_alt: bool) -> Element {
    let bg = if is_alt { "#222" } else { "#1a1a1a" };
    
    rsx! {
        div {
            // Increased padding and font-size to 1rem to match XRP's ReviewRow
            style: "display: flex; flex-direction: row; justify-content: space-between; align-items: center; padding: 1.25rem 1rem; background-color: {bg}; border-bottom: 1px solid #2a2a2a;",
            span { style: "font-size: 1rem; color: #999; font-family: monospace;", "{label}" }
            span { 
                style: "font-size: 1rem; color: white; font-weight: bold; text-align: right; flex: 1; margin-left: 2rem; word-break: break-all; font-family: monospace;", 
                "{value}" 
            }
        }
    }
}