use dioxus_native::prelude::*;
use crate::context::GlobalContext;
use crate::channel::SideBarView;
use crate::utils::styles::previous_icon_button;

#[component]
pub fn view() -> Element {
    let mut global = use_context::<GlobalContext>();
    
    let crypto_connected = *global.crypto_ws_status.read();
    let exchange_connected = *global.exchange_ws_status.read();

    let crypto_text = if crypto_connected { "CONNECTED" } else { "DISCONNECTED" };
    let crypto_color = if crypto_connected { "var(--status-ok)" } else { "var(--status-warn)" };

    let exchange_text = if exchange_connected { "CONNECTED" } else { "DISCONNECTED" };
    let exchange_color = if exchange_connected { "var(--status-ok)" } else { "var(--status-warn)" };

    let on_back_click = move |_| {
        global.sidebar_view.with_mut(|v| *v = SideBarView::None);
    };

    rsx! {
        style { {r#"
            .network-outer-viewport {
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

            .network-main-container {
                display: flex;
                flex-direction: column;
                width: 100%;
                max-width: 820px;
                margin: 0 auto;
                padding-left: 2rem;
                padding-right: 2rem;
                font-family: 'JetBrains Mono', monospace;
            }

            .network-header {
                display: flex;
                justify-content: space-between;
                align-items: flex-end;
                border-bottom: 1px solid var(--border);
                padding-bottom: 0.5rem;
                margin-bottom: 2rem;
            }

            .network-label {
                font-size: 0.7rem;
                color: var(--text-secondary);
                letter-spacing: 0.25rem;
                font-weight: 600;
                white-space: nowrap;
            }

            .status-grid {
                display: grid;
                grid-template-columns: repeat(2, 1fr);
                gap: 1.5rem;
                width: 100%;
            }

            .system-card {
                background: var(--bg-secondary);
                border: 1px solid var(--border);
                border-radius: 4px;
                padding: 1.25rem;
                display: flex;
                flex-direction: column;
                gap: 1rem;
            }

            .diag-row {
                display: flex;
                flex-direction: column;
                gap: 4px;
            }

            .diag-label {
                font-size: 0.65rem;
                color: var(--accent);
                letter-spacing: 2px;
                text-transform: uppercase;
            }

            .diag-value {
                font-size: 0.9rem;
                font-weight: 700;
                letter-spacing: 1px;
            }

            .diag-subtext {
                font-size: 0.7rem;
                color: var(--text-secondary);
                opacity: 0.8;
                border-top: 1px solid var(--border);
                padding-top: 0.75rem;
                margin-top: 0.25rem;
            }
        "#} }

        div { class: "network-outer-viewport",
            div { 
                class: "back-button-container",
                onclick: on_back_click,
                previous_icon_button { text_color: "var(--text)".to_string() }
            }

            div { class: "network-main-container",
                
                // Unified Header style
                div { class: "network-header",
                    div { class: "network-label", "SERVER_HEALTH" }
                }

                div { class: "status-grid",
                    
                    // CRYPTO WEBSOCKET
                    div { class: "system-card",
                        div { class: "diag-row",
                            div { class: "diag-label", "CRYPTO_WS" }
                            div { 
                                class: "diag-value", 
                                style: "color: {crypto_color}",
                                "{crypto_text}"
                            }
                        }
                        div { class: "diag-subtext", "BLOCKCHAIN: XRPL / BTC / MAINNET" }
                    }

                    // EXCHANGE WEBSOCKET
                    div { class: "system-card",
                        div { class: "diag-row",
                            div { class: "diag-label", "EXCHANGE_WS" }
                            div { 
                                class: "diag-value", 
                                style: "color: {exchange_color}",
                                "{exchange_text}"
                            }
                        }
                        div { class: "diag-subtext", "SOURCES: BINANCE / UPBIT / RATES" }
                    }
                }
            }
        }
    }
}