use dioxus_native::prelude::*;
use crate::context::{GlobalContext, XrpContext, RlusdContext, EuroContext, BtcContext, SgdContext};
use crate::utils::add_commas;

#[component]
pub fn render_balance() -> Element {
    let global = use_context::<GlobalContext>();
    let xrp_ctx = use_context::<XrpContext>();
    let rlusd_ctx = use_context::<RlusdContext>();
    let euro_ctx = use_context::<EuroContext>();
    let btc_ctx = use_context::<BtcContext>();
    let sgd_ctx = use_context::<SgdContext>();

    let crypto_connected = *global.crypto_ws_status.read();
    let exchange_connected = *global.exchange_ws_status.read();
    let is_connected = crypto_connected && exchange_connected;

    let (xrp_amount, _, _) = xrp_ctx.wallet_balance.read().clone();
    let (rlusd_amount, _, _) = rlusd_ctx.rlusd.read().clone();
    let (euro_amount, _, _) = euro_ctx.euro.read().clone();
    let (btc_amount, _, _) = btc_ctx.bitcoin_wallet.read().clone();
    let (sgd_amount, _, _) = sgd_ctx.sgd.read().clone();

    let rates = global.rates.read();
    let xrp_usd_rate: f64 = rates.get("XRP/USD").copied().unwrap_or(0.0) as f64;
    let btc_usd_rate: f64 = rates.get("BTC/USD").copied().unwrap_or(0.0) as f64;
    let eur_usd_rate: f64 = rates.get("EUR/USD").copied().unwrap_or(0.0) as f64;
    let sgd_usd_rate: f64 = rates.get("SGD/USD").copied().unwrap_or(0.0) as f64;

    let (_, hide_balance) = global.theme_user.read().clone();

    let total_usd: f64 = if hide_balance {
        0.0
    } else {
        (xrp_amount * xrp_usd_rate)
            + rlusd_amount
            + (euro_amount * eur_usd_rate)
            + (btc_amount * btc_usd_rate)
            + (sgd_amount * sgd_usd_rate)
    };

    let (int_part, frac_part) = if hide_balance {
        ("****".to_string(), "".to_string())
    } else {
        (
            add_commas(total_usd.floor() as i64),
            format!(".{:02}", (total_usd.fract() * 100.0).floor() as i64),
        )
    };

    let status_text = if is_connected { "CONNECTED" } else { "DISCONNECTED" };
    let status_color = if is_connected { "var(--status-ok)" } else { "var(--status-warn)" };

    rsx! {
        style { {r#"
            .balance-container {
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                width: 100%;
                font-family: 'JetBrains Mono', monospace;
                gap: 8px;
            }
            .total-amount {
                margin: 0;
                font-size: clamp(3.8rem, 8.5vw, 6.8rem);
                font-weight: 800;
                display: flex;
                align-items: baseline;
                line-height: 1;
            }
            .currency-symbol { font-size: 0.36em; color: var(--text-secondary); margin-right: 0.65rem; }
            .int-part { color: var(--text); }
            .frac-part { font-size: 0.36em; color: var(--text-secondary); margin-left: 6px; }
            
            .status-line {
                display: flex;
                align-items: center;
                gap: 10px;
                font-size: 0.68rem;
                font-weight: 700;
                letter-spacing: 2px;
                color: var(--text-secondary);
                margin-top: 10px;
            }
            .status-dot { width: 8px; height: 8px; border-radius: 50%; }
        "#} }

        div { class: "balance-container",

            h1 { class: "total-amount",
                if !hide_balance {
                    span { class: "currency-symbol", "$" }
                }
                span { class: "int-part", "{int_part}" }
                if !hide_balance {
                    span { class: "frac-part", "{frac_part}" }
                }
            }

            div { class: "status-line",
                span { 
                    class: "status-dot", 
                    style: "background: {status_color}; box-shadow: 0 0 12px {status_color};" 
                }
                span { "{status_text}" }
            }
        }
    }
}