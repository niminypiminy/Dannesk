// src/ui/ticker.rs
use dioxus::prelude::*;
use crate::context::GlobalContext;

pub fn render_ticker() -> Element {
    let global = use_context::<GlobalContext>();
    let rates = global.rates.read();
    let (is_dark_mode, _, _) = *global.theme_user.read();
    
    // We use a slightly dimmed text color for labels to make the white/black prices pop
    let base_text = if is_dark_mode { "rgba(190, 190, 190, 1)" } else { "rgb(34, 34, 34)" };

    let pairs = vec![
        ("XRP/USD", "$", 4),
        ("BTC/USD", "$", 2),
        ("EUR/USD", "$", 2),
        ("XRP/EUR", "€", 4),
        ("BTC/EUR", "€", 2),
    ];

    rsx! {
        div {
            style: "
                width: 100%; 
                height: 2rem; 
                display: flex; 
                flex-direction: row;
                justify-content: center; 
                align-items: center;
                gap: 2rem; /* Increased gap for better breathing room */
                background-color: transparent;
            ",
            
            for (name, symbol, precision) in pairs {
                {
                    let rate = rates.get(name).copied().unwrap_or(0.0);
                    rsx! {
                        div { 
                            key: "{name}", 
                            style: "display: flex; flex-direction: row; align-items: baseline; gap: 0.5rem;",
                            
                            // Dimmed Label
                            span { 
                                style: "
                                    font-family: monospace; 
                                    font-size: 0.75rem; 
                                    color: {base_text}; 
                                    opacity: 0.5;
                                    text-transform: uppercase;
                                ", 
                                "{name}" 
                            }
                            
                            // High-Contrast Bold Price
                            span { 
                                style: "
                                    font-family: monospace; 
                                    font-size: 1rem; 
                                    color: {base_text}; 
                                    font-weight: 800; /* Extra bold for visibility */
                                ", 
                                "{symbol}{rate:.precision$}"
                            }
                        }
                    }
                }
            }

            // Static items with the same styling for consistency
            StaticPrice { label: "RLUSD/USD", value: "$1.00", color: base_text }
            StaticPrice { label: "EUROP/EUR", value: "€1.00", color: base_text }
        }
    }
}

#[component]
fn StaticPrice(label: &'static str, value: &'static str, color: &'static str) -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: row; align-items: baseline; gap: 0.5rem;",
            span { 
                style: "font-family: monospace; font-size: 0.75rem; color: {color}; opacity: 0.5;", 
                "{label}" 
            }
            span { 
                style: "font-family: monospace; font-size: 1rem; color: {color}; font-weight: 800;", 
                "{value}" 
            }
        }
    }
}