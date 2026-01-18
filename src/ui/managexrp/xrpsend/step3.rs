// src/ui/managexrp/xrpsend/step3.rs

use dioxus::prelude::*;
use crate::context::{XrpContext, GlobalContext};

#[component]
pub fn view() -> Element {
    let xrp_ctx = use_context::<XrpContext>();
    let global = use_context::<GlobalContext>();
    
    let mut sign_transaction = xrp_ctx.sign_transaction;
    let rates = global.rates.read();
    let exchange_rate = rates.get("XRP/USD").copied().unwrap_or(0.0) as f64;
    
    let sign_state = sign_transaction.read();
    let send_data = sign_state.send_transaction.as_ref();

    let recipient = send_data.and_then(|s| s.recipient.clone()).unwrap_or_else(|| "N/A".into());
    let amount = send_data.and_then(|s| s.amount.clone()).unwrap_or_else(|| "0".into());
    let asset = send_data.map(|s| s.asset.clone()).unwrap_or_else(|| "XRP".into());

    let usd_amount = if asset == "XRP" {
        if let Ok(amt) = amount.parse::<f64>() {
            format!("{:.2}", amt * exchange_rate)
        } else {
            "0.00".into()
        }
    } else {
        amount.clone()
    };

    let on_confirm_click = move |_| {
        sign_transaction.with_mut(|state| {
            if let Some(ref mut send) = state.send_transaction {
                send.step = 4;
                send.error = None;
            }
        });
    };


    rsx! {
        div {
            style: "display: flex; flex-direction: column; width: 100%; align-items: center;",


            // Grid container - 33rem max width to match Step 1 input width
            div {
                style: "width: 100%; max-width: 33rem; background-color: #1a1a1a; border: 1px solid #333; border-radius: 0.25rem; display: flex; flex-direction: column;",
                
                ReviewRow { label: "Recipient".to_string(), value: recipient, is_alt: false }
                ReviewRow { label: format!("Amount ({asset})"), value: format!("{amount} {asset}"), is_alt: true }
                
                if asset != "RLUSD" {
                    ReviewRow { label: "Amount (USD)".to_string(), value: format!("${usd_amount}"), is_alt: false }
                }
                
                ReviewRow { 
                    label: "Network".to_string(), 
                    value: "XRP Ledger".to_string(), 
                    is_alt: asset != "RLUSD"
                }
            }

            // Warning Text
            div { 
                style: "width: 100%; max-width: 33rem; padding: 1.5rem 0;",
                p { 
                    style: "font-size: 0.875rem; color: #777; text-align: center; font-family: monospace; line-height: 1.4; margin: 0;",
                    "Verify the recipient address carefully. Ledger transactions cannot be undone."
                }
            }

            // Button - EXACT COPY of Step 1 and Step 4
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
            // Using padding-top/bottom to define height exactly for the native renderer
            style: "display: flex; flex-direction: row; justify-content: space-between; align-items: center; padding: 1.25rem 1rem; background-color: {bg}; border-bottom: 1px solid #2a2a2a;",
            span { style: "font-size: 1rem; color: #999; font-family: monospace;", "{label}" }
            span { 
                style: "font-size: 1rem; color: white; font-weight: bold; text-align: right; flex: 1; margin-left: 2rem; word-break: break-all; font-family: monospace;", 
                "{value}" 
            }
        }
    }
}