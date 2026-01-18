// src/ui/dashboard.rs (Dioxus version - updated)

use dioxus::prelude::*;
use crate::channel::{CHANNEL, Tab};
use crate::ui::{balance, managexrp, managebtc, ticker, progressbar::ProgressBar};
use crate::context::GlobalContext;

pub fn render_dashboard() -> Element {
    let global = use_context::<GlobalContext>();
    let current_tab = *global.selected_tab.read();
    let progress = global.progress.read().clone();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100%; width: 100%; overflow: hidden;",

            match progress {
                Some(_) => rsx! {
                    ProgressBar {
                        operation_name: "processing...".to_string()
                    }
                },
                None => rsx! {
                    // 1. TICKER (Moved to top)
                    div {
                        style: "width: 100%; padding-top: 2rem; padding-bottom: 1rem;",
                        ticker::render_ticker {}
                    }

                    // 2. MAIN CONTENT AREA
                    div {
                        style: "flex: 1; width: 100%; display: flex;",
                        match current_tab {
                            Tab::Balance => rsx! { balance::render_balance {} },
                            Tab::XRP => rsx! { managexrp::render_manage_xrp {} },
                            Tab::BTC => rsx! { managebtc::render_manage_btc {} },
                        }
                    }

                    // 3. BOTTOM NAVIGATION (Dock)
                   // 3. BOTTOM NAVIGATION (Dock)
div {
    style: "display: flex; justify-content: center; padding-bottom: 2rem; padding-top: 1rem; width: 100%;",
    div {
        style: "
            display: flex;
            gap: 0.4rem;           /* Slightly tighter gap */
            padding: 0.5rem;       /* Uniform padding for better centering */
            background-color: rgba(30, 30, 30, 0.8);
            border-radius: 2rem;
            border: 1px solid rgba(255, 255, 255, 0.1);
            align-items: center;
        ",
        DockButton { label: "Balance".to_string(), is_active: current_tab == Tab::Balance, onclick: move |_| { let _ = CHANNEL.selected_tab_tx.send(Tab::Balance); } }
        DockButton { label: "XRP".to_string(), is_active: current_tab == Tab::XRP, onclick: move |_| { let _ = CHANNEL.selected_tab_tx.send(Tab::XRP); } }
        DockButton { label: "BTC".to_string(), is_active: current_tab == Tab::BTC, onclick: move |_| { let _ = CHANNEL.selected_tab_tx.send(Tab::BTC); } }
    }
}
                },
            }
        }
    }
}

#[component]
fn DockButton(label: String, is_active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let bg_color = if is_active { "white" } else { "transparent" };
    let text_color = if is_active { "black" } else { "#aaa" };
    let font_weight = if is_active { "bold" } else { "normal" };

    rsx! {
        button {
            style: "
                padding: 0.6rem 1.2rem; /* Increased vertical from 0.4 to 0.6 */
                min-width: 85px;        /* Ensures visual symmetry regardless of word length */
                border-radius: 1.5rem; 
                border: none; 
                cursor: pointer; 
                font-size: 1rem; 
                background-color: {bg_color}; 
                color: {text_color}; 
                font-weight: {font_weight};
                display: flex;
                justify-content: center;
                align-items: center;
            ",
            onclick: onclick,
            "{label}"
        }
    }
}