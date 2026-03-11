use dioxus_native::prelude::*;
use crate::utils::styles::terminal_action;

const TRADING_PAIRS: &[(&str, &str, &str)] = &[
    ("XRP/RLUSD", "XRP", "RLUSD"),
    ("RLUSD/XRP", "RLUSD", "XRP"),
    ("XRP/EUROP", "XRP", "EUROP"),
    ("EUROP/XRP", "EUROP", "XRP"),
    ("XRP/XSGD", "XRP", "XSGD"),
    ("XSGD/XRP", "XSGD", "XRP"),
    ("RLUSD/XSGD", "RLUSD", "XSGD"),
    ("XSGD/RLUSD", "XSGD", "RLUSD"),
    ("EUROP/XSGD", "EUROP", "XSGD"),
    ("XSGD/EUROP", "XSGD", "EUROP"),
    ("RLUSD/EUROP", "RLUSD", "EUROP"),
    ("EUROP/RLUSD", "EUROP", "RLUSD"),
];

#[component]
pub fn MarketOrderForm(
    // Signals for local UI bindings
    mut search_query: Signal<String>,
    mut is_searching: Signal<bool>,
    mut selected_base: Signal<String>,
    mut selected_quote: Signal<String>,
    mut amount_sig: Signal<String>,
    mut price_sig: Signal<String>,
    active_pct: Signal<f64>,
    flags_sig: Signal<Vec<String>>,
    
    // Derived states
    has_selection: bool,
    market_rate: f64,
    
    // Event Handlers for logic that lives in the parent
    on_amount_input: EventHandler<FormEvent>,
    on_price_input: EventHandler<FormEvent>,
    on_slippage_select: EventHandler<f64>,
    on_flag_toggle: EventHandler<String>,
    on_next_click: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        style { {r#"
            /* ... (Keep your exact same CSS block here) ... */
            .trade-step-container { display: flex; flex-direction: column; width: 100%; max-width: 800px; margin: 0 auto; font-family: 'JetBrains Mono', monospace; padding: 2rem; }
            .step-header { border-bottom: 1px solid var(--border); padding-bottom: 1rem; margin-bottom: 2rem; }
            .step-title { font-size: 0.7rem; color: var(--text-secondary); letter-spacing: 2px; }
            .input-section { margin-bottom: 1.5rem; width: 100%; }
            .input-label-row { margin-bottom: 0.75rem; }
            .input-label { font-size: 0.65rem; color: var(--accent); border-left: 2px solid var(--accent); padding-left: 8px; }
            .terminal-input-wrapper { display: grid; grid-template-columns: auto 1fr auto; align-items: center; background: var(--input-bg);  border: 1px solid var(--border); padding: 0.8rem 1rem; }
            .bracket { color: var(--text-secondary); opacity: 0.4; font-weight: bold; }
            .inner-input { background: transparent; border: none; outline: none; color: var(--text); font-family: inherit; font-size: 1rem; padding: 0 1rem; width: 100%; }
            .row-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 1.5rem; }
            .search-results-box { display: flex; flex-direction: column; border: 1px solid var(--border); max-height: 150px; overflow-y: auto; }
            .search-item-btn { text-align: left; padding: 0.75rem 1rem; background: var(--input-bg); color: var(--text); border: none; border-bottom: 1px solid var(--border); font-family: inherit; }
            .toggle-group { display: grid; grid-template-columns: repeat(3, 1fr); gap: 0.5rem; }
            .flags-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 0.5rem; }
            .toggle-btn { padding: 0.7rem 0; font-size: 0.75rem; font-family: inherit; background: transparent; border: 1px solid var(--border); color: var(--text-secondary); text-align: center; }
            .toggle-btn-active { border-color: var(--accent); color: var(--accent); background: rgba(80, 250, 123, 0.05); }
            .market-rate-container { display: grid; place-items: center; margin: 1rem 0; }
            .market-rate-readout { font-size: 0.7rem; color: var(--text-secondary); letter-spacing: 1px; }
            .action-footer { display: grid; grid-template-columns: 1fr; margin-top: 1rem; }
        "#} }

        div { class: "trade-step-container",

            div { class: "step-header",
                div { class: "step-title", "MARKET_ORDER_ENTRY // STEP_01" }
            }

            // --- TRADING PAIR SELECTION ---
            div { class: "input-section",
                div { class: "input-label-row", div { class: "input-label", "TARGET_PAIR" } }
                
                if has_selection && !is_searching() {
                    button {
                        class: "terminal-input-wrapper",
                        onclick: move |_| { 
                            is_searching.set(true); 
                            search_query.set(String::new());
                        },
                        span { class: "bracket", "[" }
                        span { style: "color: var(--accent); font-weight: bold; text-align: center;", "{selected_base} / {selected_quote}" }
                        span { class: "bracket", "]" }
                    }
                } else {
                    div {
                        div { class: "terminal-input-wrapper",
                            span { class: "bracket", ">" }
                            input {
                                class: "inner-input", 
                                value: "{search_query}",
                                placeholder: "QUERY_ASSET...",
                                oninput: move |e| search_query.set(e.value()),
                            }
                            span { class: "bracket", "_" }
                        }
                        if !search_query().is_empty() || is_searching() {
                            div { class: "search-results-box",
                                for (pair, b, q) in TRADING_PAIRS.iter().filter(|(_, b, _)| b.to_lowercase().contains(&search_query().to_lowercase())) {
                                   button {
                                        class: "search-item-btn",
                                        onclick: move |_| {
                                            selected_base.set((*b).to_string());
                                            selected_quote.set((*q).to_string());
                                            is_searching.set(false);
                                        },
                                        "{pair}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // --- INPUTS ---
            div { class: "input-section row-grid",
                div {
                    div { class: "input-label-row", div { class: "input-label", "AMOUNT" } }
                    div { class: "terminal-input-wrapper",
                        span { class: "bracket", ">>" }
                        input {
                            class: "inner-input",
                            value: "{amount_sig}",
                            placeholder: "0.00",
                            oninput: move |e| on_amount_input.call(e),
                        }
                    }
                }
                div {
                    div { class: "input-label-row", div { class: "input-label", "LIMIT_PRICE" } }
                    div { class: "terminal-input-wrapper",
                        span { class: "bracket", ">>" }
                        input {
                            class: "inner-input",
                            value: "{price_sig}",
                            placeholder: "0.00",
                            oninput: move |e| on_price_input.call(e),
                        }
                    }
                }
            }

            // --- SLIPPAGE ---
            div { class: "input-section",
                div { class: "input-label-row", div { class: "input-label", "SLIPPAGE_TOLERANCE" } }
                div { class: "toggle-group",
                    for (lbl, pct) in [("0.10%", 0.10), ("0.15%", 0.15), ("0.20%", 0.20)] {
                        button {
                            class: if (active_pct() - pct).abs() < 0.001 { "toggle-btn toggle-btn-active" } else { "toggle-btn" },
                            onclick: move |_| on_slippage_select.call(pct),
                            "{lbl}"
                        }
                    }
                }
            }

            // --- FLAGS ---
            div { class: "input-section",
                div { class: "input-label-row", div { class: "input-label", "EXECUTION_FLAGS" } }
                div { class: "flags-grid",
                    for (name, label_text) in [("FillOrKill", "FILL_OR_KILL"), ("ImmediateOrCancel", "IMMEDIATE_OR_CANCEL")] {
                        button {
                            class: if flags_sig().contains(&format!("tf{}", name)) { "toggle-btn toggle-btn-active" } else { "toggle-btn" },
                            onclick: move |_| on_flag_toggle.call(name.to_string()),
                            "{label_text}"
                        }
                    }
                }
            }

            // --- RATE DISPLAY ---
            if market_rate > 0.0 { 
                div { class: "market-rate-container",
                    div { class: "market-rate-readout",
                        "1 {selected_base} ≈ {market_rate:.4} {selected_quote}" 
                    }
                }
            }

            // --- ACTION FOOTER ---
            div { class: "action-footer",
                {terminal_action("REVIEW_TRANSACTION", true, move |e| on_next_click.call(e))}
            }
        }
    }
}