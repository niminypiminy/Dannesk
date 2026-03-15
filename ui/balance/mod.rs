use dioxus_native::prelude::*;
use crate::context::{GlobalContext, XrpContext, RlusdContext, EuroContext, BtcContext, SgdContext};
use crate::utils::add_commas;

#[component]
pub fn render_balance() -> Element {
    rsx! {
        style { {r#"
            .balance-container {
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                width: 100%;
                font-family: 'JetBrains Mono', monospace;
                gap: 4px;
            }
            .balance-label {
                font-size: 0.7rem;
                color: var(--text-secondary);
                opacity: 0.6;
                letter-spacing: 1px;
                margin-bottom: 4px;
                text-transform: uppercase;
                white-space: nowrap;
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
        "#} }

        div { class: "balance-container",
            div { class: "balance-label", "TOTAL_VALUATION // USD" }
            
            // This is the optimized leaf component
            BalanceDisplay {}
        }
    }
}

#[component]
fn BalanceDisplay() -> Element {
    let global = use_context::<GlobalContext>();
    let xrp_ctx = use_context::<XrpContext>();
    let rlusd_ctx = use_context::<RlusdContext>();
    let euro_ctx = use_context::<EuroContext>();
    let btc_ctx = use_context::<BtcContext>();
    let sgd_ctx = use_context::<SgdContext>();

    // Subscribe to balance updates
    let (xrp_amount, _, _) = xrp_ctx.wallet_balance.read().clone();
    let (rlusd_amount, _, _) = rlusd_ctx.rlusd.read().clone();
    let (euro_amount, _, _) = euro_ctx.euro.read().clone();
    let (btc_amount, _, _) = btc_ctx.bitcoin_wallet.read().clone();
    let (sgd_amount, _, _) = sgd_ctx.sgd.read().clone();

    // Subscribe to rate updates
    let rates = global.rates.read();
    
    // Fixed: converted f32 to f64 via .into()
    let xrp_usd_rate: f64 = rates.get("XRP/USD").copied().unwrap_or(0.0).into();
    let btc_usd_rate: f64 = rates.get("BTC/USD").copied().unwrap_or(0.0).into();
    let eur_usd_rate: f64 = rates.get("EUR/USD").copied().unwrap_or(0.0).into();
    let sgd_usd_rate: f64 = rates.get("SGD/USD").copied().unwrap_or(0.0).into();

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

    rsx! {
        h1 { class: "total-amount",
            if !hide_balance {
                span { class: "currency-symbol", "$" }
            }
            span { class: "int-part", "{int_part}" }
            if !hide_balance {
                span { class: "frac-part", "{frac_part}" }
            }
        }
    }
}