// src/ui/managexrp/xrpsend/step1.rs

use dioxus::prelude::*;
use crate::context::XrpContext;

#[component]
pub fn view() -> Element {
    let xrp_ctx = use_context::<XrpContext>();
    let mut sign_transaction = xrp_ctx.sign_transaction;

    // Initialize local buffer from global state
    let mut addr_buffer = use_signal(|| {
        sign_transaction.read()
            .send_transaction.as_ref()
            .and_then(|s| s.recipient.clone())
            .unwrap_or_default()
    });

    // Reactive border color based on validation
    // Green = Starts with 'r' and has some length
    // Red = Typed something but doesn't start with 'r'
    let border_style = use_memo(move || {
        let val = addr_buffer();
        let trimmed = val.trim();
        
        if trimmed.starts_with('r') && trimmed.len() > 5 { // Basic sanity check
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

        // 1. Check for empty input
        if addr.is_empty() {
            sign_transaction.with_mut(|state| {
                if let Some(ref mut send) = state.send_transaction {
                    send.error = Some("Recipient address cannot be empty.".to_string());
                }
            });
            return;
        } 
        
        // 2. Comprehensive Validation: Prefix ('r') AND Length (25-35)
        if !addr.starts_with('r') || len < 25 || len > 35 {
            sign_transaction.with_mut(|state| {
                if let Some(ref mut send) = state.send_transaction {
                    send.error = Some("Invalid XRP address: Must start with 'r' and be between 25-35 characters.".to_string());
                }
            });
            return;
        }

        // 3. Success - Save to global state and advance to Step 2
        sign_transaction.with_mut(|state| {
            if let Some(ref mut send) = state.send_transaction {
                send.recipient = Some(addr); 
                send.error = None;
                send.step = 2;
            }
        });
    };

    // Pull current error from global state
    let current_error = sign_transaction.read()
        .send_transaction.as_ref()
        .and_then(|s| s.error.clone());

    rsx! {
        div {
            style: " display: flex; 
                flex-direction: column; 
                width: 100%; 
                align-items: center;",

            div { style: "font-size: 1.5rem; margin: 0; margin-bottom: 1rem;", "Recipient Address" }
            div { style: "font-size: 1rem; color: #888; margin-bottom: 1.5rem;", "Enter the XRP address you wish to send funds to." }

            input {
                style: "width: 100%; max-width: 33rem; height: 2rem; padding: 0.3125rem; background-color: transparent; border: {border_style}; border-radius: 0.25rem; font-size: 1.25rem; margin-bottom: 1rem;",
                value: "{addr_buffer()}",
                oninput: move |e| {
                    // LOGIC FIX: Strip newlines immediately to prevent Vello layout shifts
                    let clean_val = e.value().replace(['\n', '\r'], "");
                    addr_buffer.set(clean_val);
                    
                    // Clear error when user types
                    sign_transaction.with_mut(|state| {
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