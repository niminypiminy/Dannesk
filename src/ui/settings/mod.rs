// src/ui/settings.rs

use dioxus::prelude::*;
use crate::channel::{Tab, SettingsView};
use crate::context::GlobalContext;
use crate::utils::styles;
use crate::utils::profilesvg::ProfileIcon;
use crate::utils::pinsvg::PinIcon;
use crate::utils::connectionsvg::ConnectionIcon;

pub mod name;
pub mod changepin;
pub mod ws_status;
pub mod pinlogic;
pub mod namelogic;

// Helper to keep icon styles consistent with sidebar.rs
fn settings_icon_style(is_active: bool) -> String {
    format!(
        "
            font-size: 1.5rem;
            border-radius: 0.5rem;
            padding: 0.5rem;
            border: none;
            background-color: transparent;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
            transition: opacity 0.2s;
            color: {};
            opacity: {};
        ",
        if is_active { "var(--text)" } else { "var(--text-secondary)" },
        if is_active { "1.0" } else { "0.5" }
    )
}

#[component]
pub fn render_settings() -> Element {
    let global = use_context::<GlobalContext>();
    let mut settings_modal = global.settings_modal;
    let mut selected_tab = global.selected_tab;
    
    let current_view = settings_modal.read().view_type;

    let on_back_click = move |_| {
        settings_modal.with_mut(|state| {
            state.last_view = None;
        });
        selected_tab.set(Tab::Balance);
    };

    rsx! {
        style { {r#"
            .settings-main-container {
                display: flex; flex-direction: column; align-items: center; justify-content: center;
                width: 100%; height: 100%; position: relative; font-family: monospace;
            }
            /* Mirrored from balance.rs right-dock-container */
            .left-dock-container {
                position: absolute; 
    left: 2rem; 
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
            }
            .back-button-top-left {
                position: absolute;
                top: 2rem;
                left: 2rem;
                cursor: pointer;
                z-index: 10;
                opacity: 0.7;
            }
            .back-button-top-left:hover { opacity: 1; }
            
            .settings-content-wrapper {
                width: 100%;
                display: flex;
                flex-direction: column;
                align-items: center;
            }
        "#} }

       div { class: "settings-main-container",
            
            // 1. BACK BUTTON (Top Left, separate from the dock)
            div {
                class: "back-button-top-left",
                onclick: on_back_click,
                styles::previous_icon_button { text_color: "#fff".to_string() }
            }

            // 2. LEFT DOCK (Floating style matching balance.rs)
            div { class: "left-dock-container",
                button { 
                    style: settings_icon_style(current_view == SettingsView::Name),
                    onclick: move |_| settings_modal.with_mut(|s| s.view_type = SettingsView::Name),
                    ProfileIcon {} 
                }
                button { 
                    style: settings_icon_style(current_view == SettingsView::Security),
                    onclick: move |_| settings_modal.with_mut(|s| s.view_type = SettingsView::Security),
                    PinIcon {} 
                }
                button { 
                    style: settings_icon_style(current_view == SettingsView::Network),
                    onclick: move |_| settings_modal.with_mut(|s| s.view_type = SettingsView::Network),
                    ConnectionIcon {} 
                }
            }

            // 3. MAIN CONTENT
            div { class: "settings-content-wrapper",
                match current_view {
                    SettingsView::Name => rsx! { name::view {} }, 
                    SettingsView::Security => rsx! { changepin::view {} },
                    SettingsView::Network => rsx! { ws_status::view {} },
                }
            }
        }
    }
}