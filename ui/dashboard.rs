use dioxus_native::prelude::*;
use crate::channel::{Tab, SideBarView};     
use crate::ui::{balance, managebtc, managexrp, progressbar::ProgressBar, sidebar, ticker, changepin}; // ← added changepin
use crate::context::GlobalContext;

pub fn render_dashboard() -> Element {
    let global = use_context::<GlobalContext>();
    let current_tab = *global.selected_tab.read();
    let progress = global.progress.read().clone();
    let is_dark = global.theme_user.read().0;

    let dock_bg = if is_dark { "#000000" } else { "#f8fafc" };

    rsx! {
        div {
            class: "theme-root",
            style: "display: flex; flex-direction: column; height: 100%; width: 100%; overflow: hidden; position: relative;",

            // Topbar (always visible)
            div {
                style: "
                    position: absolute;
                    top: 0.5rem;
                    right: 1rem;
                    display: flex;
                    flex-direction: row;
                    gap: 0.5rem;
                    z-index: 1000;
                ",
                sidebar::render_balance_toggle {}
                sidebar::render_pin_button {}
                sidebar::render_rates_button {} 
                sidebar::render_theme_toggle {}
            }

            // Progress > Security > Normal Dashboard
            match progress {
                Some(_) => rsx! { ProgressBar { operation_name: "Processing...".to_string() } },
                None => match *global.sidebar_view.read() {
                    // ← Security takes over whole screen
                    SideBarView ::ChangePin => rsx! { changepin::view {} },
                    SideBarView::ExchangeRates => rsx! { ticker::view {} },


                    // ← Normal view
                    SideBarView::None => rsx! {
                        div {
                            class: "theme-bg-primary",
                            style: "flex: 1; width: 100%; display: flex; overflow-y: auto;",
                            match current_tab {
                                Tab::Balance => rsx! { balance::render_balance {} },
                                Tab::XRP => rsx! { managexrp::render_manage_xrp {} },
                                Tab::BTC => rsx! { managebtc::render_manage_btc {} },
                            }
                        }

                        // Bottom dock ONLY in normal view
                        div {
                            style: "
                                display: flex; 
                                width: 100%; 
                                height: 60px; 
                                background-color: {dock_bg};
                            ",
                            DockButton { 
                                label: "BALANCE".to_string(), 
                                is_active: current_tab == Tab::Balance, 
                                is_dark,
                                onclick: move |_| { let _ = crate::channel::CHANNEL.selected_tab_tx.send(Tab::Balance); } 
                            }
                            DockButton { 
                                label: "XRP".to_string(), 
                                is_active: current_tab == Tab::XRP, 
                                is_dark,
                                onclick: move |_| { let _ = crate::channel::CHANNEL.selected_tab_tx.send(Tab::XRP); } 
                            }
                            DockButton { 
                                label: "BTC".to_string(), 
                                is_active: current_tab == Tab::BTC, 
                                is_dark,
                                onclick: move |_| { let _ = crate::channel::CHANNEL.selected_tab_tx.send(Tab::BTC); } 
                            }
                        }
                    },
                },
            }
        }
    }
}

#[component]
fn DockButton(label: String, is_active: bool, is_dark: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let (text_color, bg_color) = if is_dark {
        if is_active { ("#ffffff", "#141414") } else { ("#737373", "transparent") }
    } else {
        if is_active { ("#0f172a", "#e2e8f0") } else { ("#64748b", "transparent") }
    };

    rsx! {
        button {
            style: "
                flex: 1;
                display: flex;
                align-items: center;
                justify-content: center;
                background-color: {bg_color};
                color: {text_color}; 
                font-family: monospace;
                font-size: 14px; 
                letter-spacing: 0.1em;
                cursor: pointer; 
                border: none;
                outline: none;
                margin: 0;
                padding: 0;
            ",
            onclick: onclick,
            "{label}"
        }
    }
}