use dioxus::prelude::*;
use crate::context::GlobalContext;

#[component]
pub fn view() -> Element {
    let global = use_context::<GlobalContext>();
    let crypto_connected = *global.crypto_ws_status.read();
    let exchange_connected = *global.exchange_ws_status.read();

    let crypto_color = if crypto_connected { "#10B981" } else { "#ef4444" };
    let exchange_color = if exchange_connected { "#10B981" } else { "#ef4444" };

    let is_all_good = crypto_connected && exchange_connected;
    let subtitle_text = if is_all_good {
        "All systems are operational. Real-time data is active."
    } else {
        "Connection interrupted. Attempting to restore data streams..."
    };
    
    let subtitle_color = if is_all_good { "#999" } else { "#ef4444" };

    rsx! {
        div {
            // Main Container: Centers everything horizontally
            style: "display: flex; flex-direction: column; width: 100%; align-items: center;",

            // --- Title Section ---
            // Centered text to match XRP example
            div { style: "font-size: 1.5rem; font-weight: 500; margin-bottom: 0.25rem; text-align: center;", 
                "System Status" 
            }
            div { 
                style: "font-size: 1rem; color: {subtitle_color}; margin-bottom: 2rem; margin-top:1rem; text-align: center;", 
                "{subtitle_text}" 
            }

            // Status Rows Container: Centered dots and text
            div {
                style: "display: flex; flex-direction: column; gap: 1.25rem; align-items: center;",

                // Crypto Status Row
                div { 
                    style: "display: flex; align-items: center; gap: 1rem;",
                    div { 
                        style: "width: 0.65rem; height: 0.65rem; border-radius: 50%; background-color: {crypto_color}; box-shadow: 0 0 8px {crypto_color}44;"
                    }
                    span { style: "font-size: 1rem; color: {subtitle_color};", "Crypto Exchange Stream" }
                }

                // Exchange Status Row
                div { 
                    style: "display: flex; align-items: center; gap: 1rem;",
                    div { 
                        style: "width: 0.65rem; height: 0.65rem; border-radius: 50%; background-color: {exchange_color}; box-shadow: 0 0 8px {exchange_color}44;"
                    }
                    span { style: "font-size: 1rem; color: {subtitle_color};", "Exchange Rate Stream" }
                }
            }
        }
    }
}