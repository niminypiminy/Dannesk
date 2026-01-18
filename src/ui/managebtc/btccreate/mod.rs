// src/ui/managexrp/xrpcreate/mod.rs
use dioxus::prelude::*;
use crate::context::BtcContext;
use crate::channel::{BTCActiveView}; 
use crate::utils::styles;
use arboard::Clipboard;  

pub mod step1;
pub mod step2;
pub mod btccreatelogic; 

#[component]
pub fn view() -> Element {
    let btc_ctx = use_context::<BtcContext>();
    
    let mut btc_wallet_process = btc_ctx.btc_wallet_process;
    let mut btc_modal = btc_ctx.btc_modal;
    
    // Existing variable name for RSX compatibility
    let modal_state = btc_wallet_process.read();
    
    let on_back_click = move |_| {
        if let Ok(mut ctx) = Clipboard::new() {
            let _ = ctx.set_text("");
        }

        // 1. Update the wallet data (Step back or Clear)
        btc_wallet_process.with_mut(|state| {
            if let Some(ref mut create) = state.create_wallet {
                if create.step == 1 {
                    state.create_wallet = None; 
                } else {
                    create.step = 1;
                }
            }
        });

        // 2. Navigation: If data is gone, return to dashboard
        if btc_wallet_process.read().create_wallet.is_none() {
            btc_modal.with_mut(|state| {
                state.view_type = BTCActiveView::BTC;
            });
        }
    };

    rsx! {
        style { {r#"
            .import-container {
                display: flex;
                flex-direction: column;
                width: 100%;
                position: relative;
            }
            .content-wrapper {
                flex: 1;
                display: flex;
                flex-direction: column;
                width: 100%;
                justify-content: center; 
            }
            .back-button-container {
                position: absolute;
                top: 0.75rem;
                left: 0.75rem;
                cursor: pointer;
                z-index: 10;
            }
        "#} }

        div { class: "import-container",
            // SHARED BACK BUTTON
            div { 
                class: "back-button-container",
                onclick: on_back_click,
                styles::previous_icon_button { text_color: "#fff".to_string() }
            }

            div { class: "content-wrapper",
                if let Some(create_state) = &modal_state.create_wallet {
                    match create_state.step {
                        1 => rsx! { step1::view {} },
                        2 => rsx! { step2::view {} },
                        _ => rsx! {}
                    }
                }
            }
        }
    }
}