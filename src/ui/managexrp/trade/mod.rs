use dioxus::prelude::*;
use crate::context::XrpContext;
use crate::channel::{SignTradeState};
use crate::utils::styles;
use arboard::Clipboard; 

pub mod step1;
pub mod step2;
pub mod step3; 
pub mod tradelogic;

#[component]
pub fn view() -> Element {
    let xrp_ctx = use_context::<XrpContext>();
    let mut trade = xrp_ctx.trade;
    let mut xrp_modal = xrp_ctx.xrp_modal;
    
    let trade_state = trade.read();
    let current_send = &trade_state.send_trade;

    let on_back_click = move |_| {
    if let Ok(mut ctx) = Clipboard::new() {
        let _ = ctx.set_text("");
    }

    trade.with_mut(|state: &mut SignTradeState| {
        if let Some(ref mut send) = state.send_trade {
            if send.step == 1 {
                
                xrp_modal.with_mut(|m| {
                    // Go back to the bookmarked view 
                    m.view_type = m.last_view.clone().unwrap();
                });
                state.send_trade = None;
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
                if let Some(trade_state) = current_send {
                    match trade_state.step {
                        1 => rsx! { step1::view {} },
                        2 => rsx! { step2::view {} }, // We will build step2 next
                        3 => rsx! { step3::view {} },
                        _ => rsx! { div { "Step {trade_state.step} not implemented" } }
                    }
    }
}
        }
    }
}