use dioxus_native::prelude::*;
use crate::utils::styles::terminal_action;

#[component]
pub fn SendAmountForm(
    asset_label: String,
    network_label: String,
    show_fiat: bool,
    amount_in: Signal<String>,
    fiat_in: Signal<String>,
    formatted_balance: String,
    exchange_rate: f64,
    current_error: Option<String>,
    on_amount_input: EventHandler<FormEvent>,
    on_fiat_input: EventHandler<FormEvent>,
    on_next_click: EventHandler<MouseEvent>,
) -> Element {
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
            .input-section { margin-bottom: 1.5rem; }
            .input-label-row { display: flex; justify-content: flex-start; align-items: baseline; margin-bottom: 0.75rem; gap: 1rem; }
            .input-label { font-size: 0.65rem; color: var(--accent); border-left: 2px solid var(--accent); padding-left: 8px; }
            .input-hint { font-size: 0.6rem; color: var(--text-secondary); opacity: 0.6; }
            .terminal-input-wrapper { display: flex; align-items: center; background: var(--input-bg);  border: 1px solid var(--border); padding: 0.8rem 1rem; }
            .bracket { color: var(--text-secondary); opacity: 0.4; font-weight: bold; }
            .inner-input { flex: 1; background: transparent; border: none; outline: none; color: var(--text); font-family: inherit; font-size: 1rem; padding: 0 1rem; }
            .error-box { background: rgba(239, 68, 68, 0.1); border-left: 3px solid var(--status-warn); padding: 0.75rem 1rem; margin-top: 1rem; font-size: 0.75rem; color: var(--status-warn); }
            .footer-nav { margin-top: 2rem; display: flex; justify-content: flex-end; align-items: center; gap: 2rem; }
            .data-log-row { display: flex; gap: 1rem; font-size: 0.65rem; color: var(--text-secondary); opacity: 0.8; margin-top: 0.5rem; }
            .log-key { color: var(--accent); }
        "#} }

        div { class: "send-step-container",
            div { class: "step-header",
                div { class: "step-title", "TRANSACTION_INITIALIZATION // STEP_02 // AMOUNT // {network_label}" }

            }

            // ASSET AMOUNT
            div { class: "input-section",
                div { class: "input-label-row",
                    div { class: "input-label", "ASSET_AMOUNT" }
                    div { class: "input-hint", "[{asset_label}]" }
                }
                div { class: "terminal-input-wrapper",
                    span { class: "bracket", "[" }
                    input {
                        class: "inner-input",
                        value: "{amount_in()}",
                        placeholder: "0.000000",
                        // Call the event handler
                        oninput: move |e| on_amount_input.call(e),
                    }
                    span { class: "bracket", "]" }
                }
            }

            // FIAT (conditional)
            if show_fiat {
                div { class: "input-section",
                    div { class: "input-label-row",
                        div { class: "input-label", "FIAT_EQUIVALENT" }
                        div { class: "input-hint", "[USD_VALUE]" }
                    }
                    div { class: "terminal-input-wrapper",
                        span { class: "bracket", "[" }
                        input {
                            class: "inner-input",
                            value: "{fiat_in()}",
                            placeholder: "0.00",
                            // Call the event handler
                            oninput: move |e| on_fiat_input.call(e),
                        }
                        span { class: "bracket", "]" }
                    }
                }
            }

            // DATA LOGS
            div { class: "data-log-row",
                span { class: "log-key", "AVAILABLE_LIQUIDITY:" }
                span { "{formatted_balance} {asset_label}" }
            }
            if show_fiat {
                div { class: "data-log-row",
                    span { class: "log-key", "EXCHANGE_RATE:" }
                    span { "${exchange_rate} USD/{asset_label}" }
                }
            }

            if let Some(err) = current_error {
                div { class: "error-box", "SIGNAL_INTERRUPT: {err}" }
            }

            div { class: "footer-nav",
               
                {terminal_action("CONTINUE", true, move |e| on_next_click.call(e))}
              
            }
        }
    }
}