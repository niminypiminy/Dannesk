// src/ui/managebtc/btcsend/step3.rs

use dioxus::prelude::*;
use crate::context::BtcContext;

#[component]
pub fn view() -> Element {
    let btc_ctx = use_context::<BtcContext>();
    let mut btc_sign_transaction = btc_ctx.btc_sign_transaction;

    let mut fee_in = use_signal(|| {
        btc_sign_transaction.read()
            .send_transaction.as_ref()
            .and_then(|s| Some(s.fee.clone()))
            .unwrap_or_else(|| "200".to_string())
    });

    let current_error = btc_sign_transaction.read()
        .send_transaction.as_ref()
        .and_then(|s| s.error.clone());

    // --- Dynamic Border Logic ---
    let get_border = move |val: String| -> &'static str {
        let trimmed = val.trim();
        if trimmed.is_empty() {
            return "1px solid #444"; // Default gray
        }
        
        match trimmed.parse::<u64>() {
            Ok(fee) if fee >= 200 => "1px solid #10B981", // Success Green
            _ => "1px solid #ef4444",                    // Error Red
        }
    };

    let on_fee_input = move |evt: FormEvent| {
        fee_in.set(evt.value());
        btc_sign_transaction.with_mut(|state| {
            if let Some(ref mut send) = state.send_transaction {
                send.error = None;
            }
        });
    };

    let on_next_click = move |_| {
        let fee_str = fee_in().trim().to_string();
        
        match fee_str.parse::<u64>() {
            Ok(fee_val) => {
                if fee_val < 200 {
                    btc_sign_transaction.with_mut(|state| {
                        if let Some(ref mut send) = state.send_transaction {
                            send.error = Some("Minimum fee is 200 Satoshis.".to_string());
                        }
                    });
                } else {
                    btc_sign_transaction.with_mut(|state| {
                        if let Some(ref mut send) = state.send_transaction {
                            send.fee = fee_val.to_string();
                            send.step = 4; 
                            send.error = None;
                        }
                    });
                }
            }
            Err(_) => {
                btc_sign_transaction.with_mut(|state| {
                    if let Some(ref mut send) = state.send_transaction {
                        send.error = Some("Please enter a valid whole number.".to_string());
                    }
                });
            }
        }
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; width: 100%; align-items: center;",

            div { style: "font-size: 1.5rem; margin-bottom: 1rem;", "Transaction Fee" }
            div { 
                style: "font-size: 1rem; margin-bottom: 1.5rem; color: #888;", 
                "Specify the miner fee in Satoshis." 
            }

            div { style: "width: 100%; max-width: 25rem;",
                label { style: "font-size: 0.875rem; margin-bottom: 0.5rem; display: block;", "Fee (Satoshis)" }
                input {
                    // Applied get_border here
                    style: "width: 100%; height: 2rem; padding: 0.3125rem; background-color: transparent; border: {get_border(fee_in())}; border-radius: 0.25rem; font-size: 1.25rem; display: block;",
                    r#type: "number",
                    placeholder: "200",
                    value: "{fee_in()}",
                    oninput: on_fee_input
                }
                div { 
                    style: "font-size: 0.875rem; margin-top: 0.5rem; color: #888;",
                    "Minimum required: 200 sats"
                }
            }

            if let Some(err) = current_error {
                div { style: "color: #ff4d4d; font-size: 0.875rem; font-weight: bold; margin-top: 1rem;", "{err}" }
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