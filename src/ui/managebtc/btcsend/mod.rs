// src/ui/managebtc/btcsend/mod.rs

use dioxus::prelude::*;
use crate::context::BtcContext;
use crate::channel::{BTCSignTransactionState};
use crate::utils::styles;
use arboard::Clipboard; 

pub mod step1;
pub mod step2;
pub mod step3; 
pub mod step4;
pub mod step5;
pub mod sendlogic;

#[component]
pub fn view() -> Element {
    let btc_ctx = use_context::<BtcContext>();
    let mut btc_sign_transaction = btc_ctx.btc_sign_transaction;
    let mut btc_modal = btc_ctx.btc_modal;
    
    let sign_state = btc_sign_transaction.read();
    let current_send = &sign_state.send_transaction;

    let on_back_click = move |_| {
    if let Ok(mut ctx) = Clipboard::new() {
        let _ = ctx.set_text("");
    }

    btc_sign_transaction.with_mut(|state: &mut BTCSignTransactionState| {
        if let Some(ref mut send) = state.send_transaction {
            if send.step == 1 {
                // --- CLEAN ARCHITECTURE ROUTING ---
                btc_modal.with_mut(|m| {
                    // Go back to the bookmarked view (unwrapping the Option)
                    m.view_type = m.last_view.clone().unwrap();
                });
                state.send_transaction = None;
                // ----------------------------------
            } else {
                send.step -= 1;
            }
            }
        });
    };

    rsx! {
        style { {r#"
            .send-container {
                display: flex;
                flex-direction: column;
                width: 100%;
                position: relative;
            }
            .content-wrapper {
                flex: 1;
                display: flex;
                flex-direction: column;
                justify-content: center;
                width: 100%;
            }
            .back-button-container {
                position: absolute;
                top: 0.75rem;
                left: 0.75rem;
                cursor: pointer;
                z-index: 10;
            }
        "#} }

        div { class: "send-container",
            div {
                class: "back-button-container",
                onclick: on_back_click,
                styles::previous_icon_button { text_color: "#fff".to_string() }
            }

            div { class: "content-wrapper",
                // Mirroring the if let Some logic from import
                if let Some(send_state) = current_send {
                    match send_state.step {
                        1 => rsx! { step1::view {} },
                        2 => rsx! { step2::view {} }, // 2. Add the match arm
                        3 => rsx! { step3::view {} }, // 2. Add the match arm
                    4 => rsx! { step4::view {} }, // 2. Added Step 3
                        5 => rsx! { step5::view {} }, // 3. View added
                        _ => rsx! { div { "Step {send_state.step} not implemented" } }
                       
                    }
                }
            }
        }
    }
}