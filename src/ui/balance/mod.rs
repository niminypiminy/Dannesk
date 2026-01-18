use dioxus::prelude::*;
use chrono::{Local, Timelike};
use crate::context::{GlobalContext, XrpContext, RlusdContext, EuroContext, BtcContext};
use crate::utils::add_commas; 
use crate::ui::sidebar;
use crate::ui::settings; 
use crate::channel::SettingsView;

#[component]
pub fn render_balance() -> Element {
    let global = use_context::<GlobalContext>();
    let xrp_ctx = use_context::<XrpContext>();
    let rlusd_ctx = use_context::<RlusdContext>();
    let euro_ctx = use_context::<EuroContext>();
    let btc_ctx = use_context::<BtcContext>();

    // 1. READ THE SIGNAL (This is automatically updated by your context.rs coroutine)
let settings_state = global.settings_modal.read();
    // 2. THE GATE: Swap to settings view if a setting is active.
    // We check view_type to see WHICH setting, and last_view to ensure it's "active".
    if settings_state.last_view.is_some() && matches!(settings_state.view_type, SettingsView::Name | SettingsView::Security | SettingsView::Network) {
        return rsx! { settings::render_settings {} };
    }

    // 3. DEFAULT BALANCE UI (The Dashboard)
    let (xrp_amount, _, _) = xrp_ctx.wallet_balance.read().clone();
    let (rlusd_amount, _, _) = rlusd_ctx.rlusd.read().clone();
    let (euro_amount, _, _) = euro_ctx.euro.read().clone();
    let (btc_amount, _, _) = btc_ctx.bitcoin_wallet.read().clone();

    let rates = global.rates.read();
    let xrp_usd_rate: f64 = rates.get("XRP/USD").copied().unwrap_or(0.0) as f64;
    let btc_usd_rate: f64 = rates.get("BTC/USD").copied().unwrap_or(0.0) as f64;

    let (_, user_name, hide_balance) = global.theme_user.read().clone();

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

    let now = Local::now().hour();
    let greeting = match now {
        5..=11 => "Good morning",
        12..=16 => "Good afternoon",
        _ => "Good evening",
    };

    rsx! {
        style { {r#"
            .balance-main-container {
                display: flex; flex-direction: column; align-items: center; justify-content: center;
                width: 100%; height: 100%; position: relative; font-family: monospace;
            }
            .right-dock-container {
    position: absolute; 
    right: 2rem; 
    display: flex; 
    flex-direction: column; 
    
    /* 1. Reduce the gap between icons/items */
    gap: 0.5rem; 
    
    /* 2. Reduce horizontal padding (left/right) while keeping vertical padding */
    padding: 1rem 0.5rem; 
    
    background-color: rgba(30, 30, 30, 0.8); 
    border-radius: 2rem;
    border: 1px solid rgba(255, 255, 255, 0.1); 
    align-items: center;
        "#} }

        div { class: "balance-main-container",
            div { 
                style: "margin-bottom: 1.5rem; font-size: 1.5rem; opacity: 0.7;", 
                "{greeting}, {user_name}" 
            }

            h1 {
                style: "display: flex; align-items: baseline; font-size: 6rem; line-height: 1.1; margin: 0;",
                if !hide_balance {
                    span { style: "font-weight: bold; margin-right: 0.5rem;", "$" }
                    span { style: "font-weight: bold;", "{int_part}" }
                    span { style: "opacity: 0.8;", "{frac_part}" }
                } else {
                    span { style: "font-weight: normal; letter-spacing: 0.5rem;", "****" }
                }
            }

            // RIGHT DOCK (Sidebar buttons)
           div { class: "right-dock-container",
                sidebar::render_balance_toggle {}
                sidebar::render_theme_toggle {}
                sidebar::render_settings_toggle {} 
            }
        }
    }
}