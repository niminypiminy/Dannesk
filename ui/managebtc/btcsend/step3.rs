// src/ui/managebtc/btcsend/step3.rs

use dioxus_native::prelude::*;
use crate::context::BtcContext;
use crate::utils::styles::terminal_action;

#[component]
pub fn view() -> Element {
    let btc_ctx = use_context::<BtcContext>();
    let mut btc_sign_transaction = btc_ctx.btc_sign_transaction;

    // Use a local buffer signal exactly like Step 1
    let mut fee_buffer = use_signal(|| {
        btc_sign_transaction.read()
            .send_transaction.as_ref()
            .and_then(|s| Some(s.fee.clone()))
            .unwrap_or_else(|| "200".to_string())
    });

    let current_error = btc_sign_transaction.read()
        .send_transaction.as_ref()
        .and_then(|s| s.error.clone());

    let on_next_click = move |_| {
        let fee_str = fee_buffer().trim().to_string();
        
        match fee_str.parse::<u64>() {
            Ok(fee_val) => {
                if fee_val < 200 {
                    btc_sign_transaction.with_mut(|state| {
                        if let Some(ref mut send) = state.send_transaction {
                            send.error = Some("ERR: BELOW_MINIMUM_RELAY_FEE".to_string());
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
                        send.error = Some("ERR: NON_INTEGER_VALUE".to_string());
                    }
                });
            }
        }
    };

    rsx! {
        style { {r#"
            .send-step-container {
                display: flex;
                flex-direction: column;
                width: 100%;
                max-width: 800px;
                margin: 0 auto;
                font-family: 'JetBrains Mono', monospace;
                padding: 2rem;
            }
            .step-header { border-bottom: 1px solid var(--border); padding-bottom: 1rem; margin-bottom: 2.5rem; }
            .step-title { font-size: 0.7rem; color: var(--text-secondary); letter-spacing: 2px; }
            .input-section { margin-bottom: 2rem; }
            .input-label-row { display: flex; justify-content: space-between; align-items: baseline; margin-bottom: 0.75rem; }
            .input-label { font-size: 0.65rem; color: var(--accent); border-left: 2px solid var(--accent); padding-left: 8px; }
            .fee-info-micro { font-size: 0.6rem; color: var(--text-secondary); opacity: 0.6; }
            .terminal-input-wrapper { display: flex; align-items: center; background: var(--input-bg); border: 1px solid var(--border); padding: 0.8rem 1rem; }
            .bracket { color: var(--text-secondary); opacity: 0.4; font-weight: bold; }
            .inner-input { flex: 1; background: transparent; border: none; outline: none; color: var(--text); font-family: inherit; font-size: 1rem; padding: 0 1rem; }
            .unit-tag { font-size: 0.7rem; color: var(--text-secondary); padding-left: 0.5rem; }
            .error-box { background: rgba(239, 68, 68, 0.1); border-left: 3px solid var(--status-warn); padding: 0.75rem 1rem; margin-top: 1rem; font-size: 0.75rem; color: var(--status-warn); }
            .footer-nav { margin-top: 2rem; display: flex; justify-content: flex-end; align-items: center; gap: 2rem; }
        "#} }

        div { class: "send-step-container",
            div { class: "step-header",
                div { class: "step-title", "TRANSACTION_INITIALIZATION // STEP_03 // NETWORK_FEE // BITCOIN_MAINNET" }
            }

            div { class: "input-section",
                div { class: "input-label-row",
                    div { class: "input-label", "MINER_FEE" }
                    div { class: "fee-info-micro", "MIN_AMOUNT: 200 SATS" }
                }
                div { class: "terminal-input-wrapper",
                    span { class: "bracket", "[" }
                    input {
                        class: "inner-input",
                        // Removed r#type: "number" as it's buggy in native
                        value: "{fee_buffer()}",
                        oninput: move |e| {
                            let val = e.value();
                            // Filter only digits to prevent alpha characters in fee
                            let clean_val: String = val.chars().filter(|c| c.is_ascii_digit()).collect();
                            fee_buffer.set(clean_val);
                            
                            btc_sign_transaction.with_mut(|state| {
                                if let Some(ref mut send) = state.send_transaction {
                                    send.error = None;
                                }
                            });
                        },
                    }
                    span { class: "unit-tag", "SATS" }
                    span { class: "bracket", "]" }
                }

                if let Some(err) = current_error {
                    div { class: "error-box", "SIGNAL_INTERRUPT: {err}" }
                }
            }

            div { class: "footer-nav",
               
                {terminal_action("CONTINUE", true, on_next_click)}
            }
        }
    }
}