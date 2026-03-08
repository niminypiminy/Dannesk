use dioxus_native::prelude::*;
use crate::context::GlobalContext;
use crate::channel::SideBarView;
use crate::utils::styles::previous_icon_button;

#[derive(Clone, Copy)]
pub struct AssetConfig {
    pub name: &'static str,
    pub symbol: &'static str,
    pub precision: usize,
}

pub struct AssetGroup {
    pub title: &'static str,
    pub assets: &'static [AssetConfig],
}

const MARKET_GROUPS: &[AssetGroup] = &[
    AssetGroup {
        title: "USD Markets",
        assets: &[
            AssetConfig { name: "BTC/USD", symbol: "$", precision: 2 },
            AssetConfig { name: "XRP/USD", symbol: "$", precision: 4 },
            AssetConfig { name: "EUR/USD", symbol: "$", precision: 4 },
            AssetConfig { name: "SGD/USD", symbol: "$", precision: 4 },
        ],
    },
    AssetGroup {
        title: "EUR Markets",
        assets: &[
            AssetConfig { name: "BTC/EUR", symbol: "€", precision: 2 },
            AssetConfig { name: "XRP/EUR", symbol: "€", precision: 4 },
            AssetConfig { name: "USD/EUR", symbol: "€", precision: 4 },
            AssetConfig { name: "SGD/EUR", symbol: "€", precision: 4 },
        ],
    },
    AssetGroup {
        title: "SGD Markets",
        assets: &[
            AssetConfig { name: "BTC/SGD", symbol: "S$", precision: 2 },
            AssetConfig { name: "XRP/SGD", symbol: "S$", precision: 4 },
            AssetConfig { name: "USD/SGD", symbol: "S$", precision: 4 },
            AssetConfig { name: "EUR/SGD", symbol: "S$", precision: 4 },
        ],
    },
];

#[component]
pub fn view() -> Element {
    let mut global = use_context::<GlobalContext>();
    let rates = global.rates.read();

    let on_back_click = move |_| {
        global.sidebar_view.with_mut(|v| *v = SideBarView::None);
    };

    rsx! {
        style { {r#"
            .ticker-outer-viewport {
               display: flex; 
                flex-direction: row; 
                width: 100%; 
                flex: 1;
                align-items: center;
            }

            .back-button-container {
                position: absolute;
                top: 1.25rem;
                left: 1.25rem;
                cursor: pointer;
                z-index: 100;
            }

            .ticker-main-container {
                display: flex;
                flex-direction: column;
                max-width: 820px; /* Matched to your balance example */
                margin: 0 auto;
                padding-left: 2rem;
                padding-right: 2rem;
                font-family: 'JetBrains Mono', monospace;
            }

            .ticker-header {
                display: flex;
                justify-content: space-between;
                align-items: flex-end;
                border-bottom: 1px solid var(--border);
                padding-bottom: 0.5rem;
                margin-bottom: 2rem;
            }

            .ticker-label {
                font-size: 0.7rem;
                color: var(--text-secondary);
                letter-spacing: 0.25rem;
                font-weight: 600;
                white-space: nowrap;
            }

            .markets-grid {
                display: grid;
                grid-template-columns: repeat(3, 1fr);
                gap: 1.5rem;
                width: 100%;
            }

            .group-container {
                display: flex;
                flex-direction: column;
                gap: 0.45rem;
            }

            .group-title {
                font-size: 0.65rem;
                color: var(--accent);
                letter-spacing: 2px;
                margin-bottom: 0.75rem;
                text-transform: uppercase;
                white-space: nowrap;
            }

            .market-row {
                background: var(--bg-secondary);
                border: 1px solid var(--border);
                border-radius: 4px;
                padding: 0.75rem 1rem;
                gap: 1.5rem;
                display: flex;
                justify-content: space-between;
                align-items: center;
            }

            .market-name {
                font-size: 0.85rem;
                color: var(--accent);
                font-weight: 600;
                white-space: nowrap;
                flex-shrink: 0;
            }

            .market-value {
                font-size: 0.92rem;
                color: var(--text);
                text-align: right;
                white-space: nowrap;
            }
        "#} }

        div { class: "ticker-outer-viewport",
            // 1. PINNED TOP-LEFT
            div { 
                class: "back-button-container",
                onclick: on_back_click,
                previous_icon_button { text_color: "var(--text)".to_string() }
            }

            // 2. CENTERED CONTENT (Matches balance layout)
            div { class: "ticker-main-container",
                div { class: "ticker-header",
                    div { class: "ticker-label", "MARKET RATES" }
                }

                div { class: "markets-grid",
                    for group in MARKET_GROUPS {
                        div { class: "group-container",
                            div { class: "group-title", "{group.title}" }
                            for asset in group.assets {
                                {
                                    let rate = rates.get(asset.name).copied().unwrap_or(0.0);
                                    let formatted = if rate > 0.0 {
                                        format!("{}{:.precision$}", asset.symbol, rate, precision = asset.precision)
                                    } else {
                                        "—".to_string()
                                    };
                                    rsx! {
                                        div { class: "market-row",
                                            span { class: "market-name", "{asset.name}" }
                                            span { class: "market-value", "{formatted}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}