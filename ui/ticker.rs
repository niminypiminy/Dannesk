use dioxus_native::prelude::*;
use crate::context::GlobalContext;

pub fn render_ticker() -> Element {
    let global = use_context::<GlobalContext>();
    let rates = global.rates.read();
    
    let assets = vec![
        ("BTC/USD", "$", 2),
        ("XRP/USD", "$", 4),
        ("EUR/USD", "$", 2),
        ("XRP/EUR", "€", 4),
        ("BTC/EUR", "€", 2),
    ];

   rsx! {
        style { {r#"
            .ticker-grid {
                display: grid;
                /* Five columns, tight alignment */
                grid-template-columns: repeat(3, 1fr); 
                gap: 1px; /* The gap creates the "grid lines" look */
                background: var(--border); /* Borders via grid gap */
                border: 1px solid var(--border);
                margin-top: 4vh;
                width: 100%;
                max-width: 800px;
            }

            .market-card {
                background: var(--bg-primary);
                padding: 0.75rem 1.2rem;
                display: flex;
                justify-content: space-between; 
                align-items: center;
           }

            .market-card:hover {
                background: var(--bg-card);
            }

            .market-name {
                font-size: 0.7rem;
                color: var(--text-secondary);
                letter-spacing: 0.1rem;
                font-weight: 500;
            }

            .market-value {
                font-family: 'JetBrains Mono', monospace;
                font-size: 1rem;
                font-weight: 500;
                color: var(--text);
            }

        "#} }
        div { class: "ticker-grid",
            for (name, symbol, precision) in assets {
                {
                    let rate = rates.get(name).copied().unwrap_or(0.0);
                    rsx! {
                        MarketCard { 
                            name, 
                            value: format!("{}{:.precision$}", symbol, rate),
                        }
                    }
                }
            }
            
            MarketCard { name: "RLUSD", value: "$1.00".to_string() }
        }
    }
}

#[component]
fn MarketCard(name: &'static str, value: String) -> Element {
    rsx! {
        div { class: "market-card",
            span { class: "market-name", "{name}" }
            span { class: "market-value", "{value}" }
        }
    }
}