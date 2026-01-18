use dioxus::prelude::*;
use crate::context::{XrpContext, GlobalContext};

const TRADING_PAIRS: &[(&str, &str, &str)] = &[
    ("XRP/RLUSD", "XRP", "RLUSD"),
    ("RLUSD/XRP", "RLUSD", "XRP"),
    ("XRP/EUROP", "XRP", "EUROP"),
    ("EUROP/XRP", "EUROP", "XRP"),
    ("RLUSD/EUROP", "RLUSD", "EUROP"),
    ("EUROP/RLUSD", "EUROP", "RLUSD"),
];

#[component]
pub fn view() -> Element {
    let mut xrp_ctx = use_context::<XrpContext>();
    let global_ctx = use_context::<GlobalContext>();
    
    let mut search_query = use_signal(String::new);
    let mut is_searching = use_signal(|| false); 

    let rates_signal = global_ctx.rates;
    let trade_read = xrp_ctx.trade.read();
    
    let Some(inner) = trade_read.send_trade.as_ref() else {
        return rsx! {};
    };

    let mut selected_base = use_signal(|| inner.base_asset.clone().unwrap_or_default());
    let mut selected_quote = use_signal(|| inner.quote_asset.clone().unwrap_or_default());
    let mut amount_sig = use_signal(|| inner.amount.clone().unwrap_or_default());
    let mut price_sig = use_signal(|| inner.limit_price.clone().unwrap_or_default());
    let mut active_pct = use_signal(|| inner.fee_percentage);
    let mut flags_sig = use_signal(|| inner.flags.clone().unwrap_or_default());

    let has_selection = !selected_base().is_empty() && !selected_quote().is_empty();

    // 1. Optimized Rate Logic
    let market_rate = use_memo(move || {
        let rates_data = rates_signal.read();
        
        let xrp_usd = rates_data.get("XRP/USD").copied().unwrap_or(0.0) as f64;
        let eur_usd = rates_data.get("EUR/USD").copied().unwrap_or(0.0) as f64;
        let xrp_eur = rates_data.get("XRP/EUR").copied().unwrap_or(0.0) as f64;

        match (selected_base().as_str(), selected_quote().as_str()) {
            ("XRP", "RLUSD") => xrp_usd,
            ("RLUSD", "XRP") => if xrp_usd > 0.0 { 1.0 / xrp_usd } else { 0.0 },
            ("XRP", "EUROP") => xrp_eur,
            ("EUROP", "XRP") => if xrp_eur > 0.0 { 1.0 / xrp_eur } else { 0.0 },
            ("RLUSD", "EUROP") => if eur_usd > 0.0 { 1.0 / eur_usd } else { 0.0 },
            ("EUROP", "RLUSD") => eur_usd,
            _ => 0.0,
        }
    });

    let mut update_price = move |amount_str: String, pct: f64| {
        if let Ok(amt) = amount_str.parse::<f64>() {
            let rate = market_rate();
            if rate > 0.0 {
                let calculated = (amt * rate) * (1.0 + (pct / 100.0));
                price_sig.set(format!("{:.4}", calculated));
            }
        } else {
            price_sig.set(String::new());
        }
    };

    let normalized_search = search_query().to_lowercase();

    let is_valid = !selected_base().is_empty() 
        && !selected_quote().is_empty() 
        && !amount_sig().is_empty() 
        && !price_sig().is_empty() 
        && !flags_sig().is_empty();

    // --- STYLES (Updated to use CSS Variables) ---
    // Note: We use var(--border) and var(--text) to auto-switch between light/dark.
    
    let base_container_style = "box-sizing: border-box; width: 100%; display: flex; flex-direction: column;";
    
    // Transparent background, but border color adapts to theme
    let input_base_style = "box-sizing: border-box; display: block; width: 100%; height: 2.5rem; padding: 0.5rem; background-color: transparent; border: 1px solid var(--border); color: var(--text); border-radius: 0.5rem; font-size: 1rem; outline: none;";
    
    let label_style = "display: block; font-size: 0.75rem; color: var(--text-secondary); margin-bottom: 0.35rem; font-weight: 500;";

    rsx! {
        div { style: "display: flex; flex-direction: column; width: 100%; align-items: center; padding-top: 1.5rem;",
            div { style: "width: 100%; max-width: 28rem; display: flex; flex-direction: column; box-sizing: border-box;",

                // --- TRADING PAIR SELECTION ---
                div { style: "width: 100%; margin-bottom: 1.5rem; box-sizing: border-box;",
                    if has_selection && !is_searching() {
                        button {
                            // Using variables for border and text
                            style: "box-sizing: border-box; display: flex; flex-direction: row; justify-content: space-between; align-items: center; width: 100%; padding: 0.5rem 0.75rem; background-color: transparent; border: 1px solid var(--border); border-radius: 0.5rem; cursor: pointer;",
                            onclick: move |_| { 
                                is_searching.set(true); 
                                search_query.set(String::new());
                            },
                            div { style: "display: flex; flex-direction: column; align-items: flex-start;",
                                span { style: "font-size: 0.75rem; color: var(--text-secondary);", "Trading Pair" }
                                span { style: "font-size: 1.125rem; font-weight: bold; color: var(--text);", "{selected_base} / {selected_quote}" }
                            }
                            span { style: "color: var(--text-secondary); font-size: 1.25rem;", "⇄" }
                        }
                    } else {
                        div { style: "{base_container_style}",
                            label { style: "{label_style}", "Select Pair" }
                            input {
                                style: "{input_base_style}", 
                                value: "{search_query}",
                                placeholder: "e.g. XRP",
                                oninput: move |e| search_query.set(e.value()),
                            }
                            if !search_query().is_empty() || is_searching() {
                                div { style: "display: flex; flex-direction: column; margin-top: 0.5rem; border: 1px solid var(--border); border-radius: 0.5rem; overflow: hidden;",
                                    for (pair, b, q) in TRADING_PAIRS.iter().filter(|(p, _, _)| p.to_lowercase().contains(&normalized_search)) {
                                       button {
                                            // bg-card ensures it's white on light mode, dark grey on dark mode
                                            style: "text-align: left; padding: 0.75rem; background: var(--bg-card); color: var(--text); border: none; border-bottom: 1px solid var(--border); cursor: pointer; width: 100%; box-sizing: border-box;",
                                            onclick: move |_| {
                                                selected_base.set((*b).to_string());
                                                selected_quote.set((*q).to_string());
                                                amount_sig.set(String::new());
                                                price_sig.set(String::new());
                                                active_pct.set(0.0);
                                                search_query.set(String::new());
                                                is_searching.set(false);
                                            },
                                            span { style: "font-weight: 500;", "{pair}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // --- INPUTS ---
                div { style: "display: flex; flex-direction: row; width: 100%; margin-bottom: 1.5rem; box-sizing: border-box;",
                    div { style: "flex: 1; padding-right: 0.5rem; box-sizing: border-box;",
                        label { style: "{label_style}", "Amount ({selected_base})" }
                        input {
                            style: "{input_base_style}",
                            r#type: "number",
                            value: "{amount_sig}",
                            oninput: move |e| {
                                amount_sig.set(e.value());
                                update_price(e.value(), active_pct());
                            }
                        }
                    }
                    div { style: "flex: 1; padding-left: 0.5rem; box-sizing: border-box;",
                        label { style: "{label_style}", "Limit Price ({selected_quote})" }
                        input {
                            style: "{input_base_style}",
                            r#type: "number",
                            value: "{price_sig}",
                            oninput: move |e| {
                                price_sig.set(e.value());
                                active_pct.set(0.0); 
                            },
                        }
                    }
                }

                // --- SPREAD BUTTONS ---
                div { style: "width: 100%; margin-bottom: 1.5rem; box-sizing: border-box;",
                    label { style: "{label_style}", "Spread" }
                    div { style: "display: flex; flex-direction: row; box-sizing: border-box;",
                        for (lbl, pct) in [("Auto", 0.0), ("0.10%", 0.10), ("0.15%", 0.15), ("0.20%", 0.20)] {
                            button {
                                // Logic: If active, use theme button color (usually dark grey) + white text.
                                // If inactive, transparent background + secondary text color + theme border.
                                style: format!(
                                    "flex: 1; height: 2.25rem; border-radius: 0.25rem; border: 1px solid; font-size: 0.85rem; font-weight: 500; cursor: pointer; display: flex; align-items: center; justify-content: center; margin: 0 0.15rem; box-sizing: border-box; {}",
                                    if (active_pct() - pct).abs() < 0.001 { 
                                        "background-color: var(--btn); border-color: var(--btn); color: #fff;" 
                                    } else { 
                                        "background-color: transparent; border-color: var(--border); color: var(--text-secondary);" 
                                    }
                                ),
                                onclick: move |_| {
                                    active_pct.set(pct);
                                    update_price(amount_sig(), pct);
                                },
                                "{lbl}"
                            }
                        }
                    }
                }

                // --- FLAGS BUTTONS ---
                div { style: "width: 100%; margin-bottom: 1.5rem; box-sizing: border-box;",
                    label { style: "{label_style}", "Execution Flags" }
                    div { style: "display: flex; flex-direction: row; box-sizing: border-box;",
                        for (name, label_text) in [("FillOrKill", "Fill or Kill"), ("ImmediateOrCancel", "Immediate or Cancel")] {
                            button {
                                style: format!(
                                    "flex: 1; display: flex; align-items: center; justify-content: center; height: 2.25rem; border-radius: 0.25rem; border: 1px solid; cursor: pointer; margin: 0 0.15rem; box-sizing: border-box; {}",
                                    if flags_sig().contains(&format!("tf{}", name)) { 
                                        "background-color: var(--btn); border-color: var(--btn); color: #fff;" 
                                    } else { 
                                        "background-color: transparent; border-color: var(--border); color: var(--text-secondary);" 
                                    }
                                ),
                                onclick: move |_| {
                                    let flag_val = format!("tf{}", name);
                                    let mut current = flags_sig.read().clone();
                                    if current.contains(&flag_val) {
                                        current.retain(|f| f != &flag_val);
                                    } else {
                                        let other = if name == "FillOrKill" { "tfImmediateOrCancel" } else { "tfFillOrKill" };
                                        current.retain(|f| f != other);
                                        current.push(flag_val);
                                    }
                                    flags_sig.set(current);
                                },
                                span { font_size: "0.85rem", "{label_text}" }
                            }
                        }
                    }
                }

               if market_rate() > 0.0 { 
                    div { style: "width: 100%; text-align: center; margin-bottom: 1rem;",
                        span { style: "font-size: 0.75rem; color: var(--text-secondary);", "Current Market Rate: " }
                        span { style: "font-size: 0.75rem; color: var(--text); font-family: monospace;", 
                            "1 {selected_base} ≈ {market_rate():.4} {selected_quote}" 
                        }
                    }
                }

                // --- ACTION BUTTON ---
                button {
                    style: format!(
                        "box-sizing: border-box; width: 100%; height: 3rem; border: none; border-radius: 1.5rem; font-weight: 600; font-size: 1rem; display: flex; align-items: center; justify-content: center; transition: all 0.2s; {}",
                        if is_valid { 
                            "background-color: var(--btn); color: #fff; cursor: pointer;" 
                        } else { 
                            "background-color: var(--bg-secondary); color: var(--text-secondary); opacity: 0.6; cursor: not-allowed;" 
                        }
                    ),
                    disabled: !is_valid,
                    onclick: move |_| {
                        if is_valid {
                            xrp_ctx.trade.with_mut(|s| if let Some(ref mut t) = s.send_trade {
                                t.base_asset = Some(selected_base());
                                t.quote_asset = Some(selected_quote());
                                t.amount = Some(amount_sig());
                                t.limit_price = Some(price_sig());
                                t.fee_percentage = active_pct();
                                t.flags = Some(flags_sig());
                                t.step = 2;
                            });
                        }
                    },
                    "Review Trade"
                }
            }
        }
    }
}