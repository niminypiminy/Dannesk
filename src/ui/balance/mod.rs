use dioxus_native::prelude::*;
use crate::context::{GlobalContext, XrpContext, RlusdContext, EuroContext, BtcContext};
use crate::utils::add_commas;
use crate::ui::ticker;
use crate::ui::changepin;
use crate::utils::styles::{terminal_action};

#[derive(Clone, Copy, PartialEq)]
enum LocalPage {
    Dashboard,
    SecurityUpdate,
}

#[component]
pub fn render_balance() -> Element {
    let global = use_context::<GlobalContext>();
    let xrp_ctx = use_context::<XrpContext>();
    let rlusd_ctx = use_context::<RlusdContext>();
    let euro_ctx = use_context::<EuroContext>();
    let btc_ctx = use_context::<BtcContext>();

    let mut current_page = use_signal(|| LocalPage::Dashboard);

    // If we are on the Security page, return that view immediately
    if current_page() == LocalPage::SecurityUpdate {
        return rsx! {
            changepin::view { 
                on_back: move |_| current_page.set(LocalPage::Dashboard) 
            }
        };
    }

    // --- Dashboard Logic ---
    let crypto_connected = *global.crypto_ws_status.read();
    let exchange_connected = *global.exchange_ws_status.read();
    let is_connected = crypto_connected && exchange_connected;

    let (xrp_amount, _, _) = xrp_ctx.wallet_balance.read().clone();
    let (rlusd_amount, _, _) = rlusd_ctx.rlusd.read().clone();
    let (euro_amount, _, _) = euro_ctx.euro.read().clone();
    let (btc_amount, _, _) = btc_ctx.bitcoin_wallet.read().clone();

    let rates = global.rates.read();
    let xrp_usd_rate: f64 = rates.get("XRP/USD").copied().unwrap_or(0.0) as f64;
    let btc_usd_rate: f64 = rates.get("BTC/USD").copied().unwrap_or(0.0) as f64;

    let (_, hide_balance) = global.theme_user.read().clone();

    let total_usd = if hide_balance { 0.0 } else {
        (xrp_amount * xrp_usd_rate) + rlusd_amount + euro_amount + (btc_amount * btc_usd_rate)
    };

    let (int_part, frac_part) = if hide_balance {
        ("****".to_string(), "".to_string())
    } else {
        (
            add_commas(total_usd.floor() as i64),
            format!(".{:02}", (total_usd.fract() * 100.0).floor() as i64)
        )
    };

    let status_text = if is_connected { "CONNECTED" } else { "DISCONNECTED" };
    let status_color = if is_connected { "var(--status-ok)" } else { "var(--status-warn)" };

    rsx! {
        style { {r#"
            .balance-main-container {
                display: flex;
                flex-direction: column;
                width: 100%;
                max-width: 800px;
                margin: 0 auto;
                justify-content:center;
                padding-top: 8vh;
                padding-left: 2rem;
                padding-right: 2rem;
                box-sizing: border-box;
            }
            .balance-header {
                display: flex;
                justify-content: space-between;
                align-items: flex-end;
                border-bottom: 1px solid var(--border);
                padding-bottom: 0.5rem;
                margin-bottom: 2rem;
            }
            .balance-label { 
                font-size: 0.7rem; 
                color: var(--text-secondary); 
                letter-spacing: 0.25rem; 
                font-weight: 600;
            }
            .balance-amount { 
                display: flex; 
                align-items: baseline; 
                font-size: clamp(2.5rem, 6vw, 5rem); 
                line-height: 1; 
                margin: 0; 
                font-family: 'JetBrains Mono', monospace;
            }
            .currency-symbol { color: var(--text-secondary); margin-right: 0.75rem; font-size: 0.4em; }
            .int-part { font-weight: 700; color: var(--text); }
            .frac-part { color: var(--text-secondary); font-size: 0.4em; margin-left: 2px; }
            .status-badge { display: flex; align-items: center; gap: 0.5rem; }
            .status-dot { width: 6px; height: 6px; border-radius: 50%; }
            .status-text { font-size: 0.6rem; color: var(--text-secondary); font-weight: 700; }
            .ticker-spacing { margin-top: 2rem; width: 100%; }
            .action-footer { margin-top: 4rem; display: flex; justify-content: flex-start; }
        "#} }

        div { class: "balance-main-container",
            div { class: "balance-header",
                div { class: "balance-label", "BALANCE_TOTAL_USD" }
                div { class: "status-badge",
                    span { 
                        class: "status-dot",
                        style: "background: {status_color}; box-shadow: 0 0 8px {status_color};" 
                    }
                    span { class: "status-text", "{status_text}" }
                }
            }

            h1 { class: "balance-amount",
                if !hide_balance {
                    span { class: "currency-symbol", "USD" }
                    span { class: "int-part", "{int_part}" }
                    span { class: "frac-part", "{frac_part}" }
                } else {
                    span { style: "color: var(--accent); letter-spacing: 0.5rem; opacity: 0.5;", "****" }
                }
            }

            div { class: "ticker-spacing",
                ticker::render_ticker {}
            }

            div { class: "action-footer",
                {terminal_action("SECURITY_PIN", true, move |_| current_page.set(LocalPage::SecurityUpdate))}
            }
        }
    }
}