// src/ui/managebtc/btcsend/step1.rs

use dioxus::prelude::*;
use crate::context::BtcContext;

#[component]
pub fn view() -> Element {
    let btc_ctx = use_context::<BtcContext>();
    let mut btc_sign_transaction = btc_ctx.btc_sign_transaction;

    // Initialize local buffer from global state
    let mut addr_buffer = use_signal(|| {
        btc_sign_transaction.read()
            .send_transaction.as_ref()
            .and_then(|s| s.recipient.clone())
            .unwrap_or_default()
    });

    // Reactive border color based on validation
    let border_style = use_memo(move || {
        let val = addr_buffer();
        let trimmed = val.trim();
        
        if trimmed.starts_with("bc1") && trimmed.len() > 5 {
            "1px solid #10B981" // Green
        } else if !trimmed.is_empty() {
            "1px solid #ef4444" // Red
        } else {
            "1px solid #444" // Default
        }
    });

   let on_next_click = move |_| {
        let addr = addr_buffer().trim().to_string();
        let len = addr.len();

        if addr.is_empty() {
            btc_sign_transaction.with_mut(|state| {
                if let Some(ref mut send) = state.send_transaction {
                    send.error = Some("Recipient address cannot be empty.".to_string());
                }
            });
            return;
        } 
        
        if !addr.starts_with("bc1") || len < 25 || len > 60 {
            btc_sign_transaction.with_mut(|state| {
                if let Some(ref mut send) = state.send_transaction {
                    send.error = Some("Invalid BTC address: Must start with 'bc1' and be between 25-60 characters.".to_string());
                }
            });
            return;
        }

        btc_sign_transaction.with_mut(|state| {
            if let Some(ref mut send) = state.send_transaction {
                send.recipient = Some(addr); 
                send.error = None;
                send.step = 2;
            }
        });
    };

    let current_error = btc_sign_transaction.read()
        .send_transaction.as_ref()
        .and_then(|s| s.error.clone());

    rsx! {
        div {
            style: " display: flex; 
                flex-direction: column; 
                width: 100%; 
                align-items: center;",

            div { style: "font-size: 1.5rem; margin: 0; margin-bottom: 1rem;", "Recipient Address" }
            div { style: "font-size: 1rem; color: #888; margin-bottom: 1.5rem;", "Enter the BTC address you wish to send funds to." }

            input {
                style: "width: 100%; max-width: 33rem; height: 2rem; padding: 0.3125rem; background-color: transparent; border: {border_style}; border-radius: 0.25rem; font-size: 1.25rem; margin-bottom: 1rem; ",
                value: "{addr_buffer()}",
                oninput: move |e| {
                    // LOGIC FIX: Strip newlines immediately to prevent Vello layout shifts
                    let clean_val = e.value().replace(['\n', '\r'], "");
                    addr_buffer.set(clean_val);
                    
                    btc_sign_transaction.with_mut(|state| {
                        if let Some(ref mut send) = state.send_transaction {
                            send.error = None;
                        }
                    });
                }
            }

            if let Some(err) = current_error {
                div { style: "color: #ff4d4d; margin-bottom: 1rem; font-size: 0.875rem; font-weight: bold;", "{err}" }
            }

            button {
                style: "width: 8.75rem; height: 2.25rem; background-color: #333; color: white; border: none; 
        border-radius: 1.375rem; font-size: 1rem; display: flex; cursor: pointer; justify-content: center; align-items: center; margin-top: 1rem;",
                onclick: on_next_click,
                "Continue"
            }
        }
    }
}