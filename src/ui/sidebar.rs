use dioxus_native::prelude::*;
use crate::context::GlobalContext;
use crate::channel::CHANNEL;

#[component]
fn CliIndicator(label: String, is_active: bool) -> Element {
    let bracket_color = "var(--text-secondary)";
    let symbol = if is_active { ":" } else { "." };
    let symbol_color = if is_active { "var(--accent)" } else { "var(--text-secondary)" };

    rsx! {
        span {
            style: "font-family: 'JetBrains Mono', monospace; font-size: 0.75rem; font-weight: 700; letter-spacing: 1px;",
            span { style: "color: {bracket_color}; opacity: 0.4;", "[" }
            span { style: "color: {symbol_color};", "{symbol}" }
            span { style: "color: var(--text); padding: 0 4px;", "{label}" }
            span { style: "color: {symbol_color};", "{symbol}" }
            span { style: "color: {bracket_color}; opacity: 0.4;", "]" }
        }
    }
}

fn base_button_style() -> String {
    "background: transparent; border: none; cursor: pointer; padding: 6px; display: flex; align-items: center;".to_string()
}

pub fn render_theme_toggle() -> Element {
    let global = use_context::<GlobalContext>();
    // Pull current state from the signal
    let (is_dark, hide_balance) = global.theme_user.read().clone();
    
    let label = if is_dark { "MODE_DRK" } else { "MODE_LGT" };

    rsx! {
        button {
            style: base_button_style(),
            // Push update to Channel Sender instead of manually setting Signal
            onclick: move |_| {
                let _ = CHANNEL.theme_user_tx.send((!is_dark, hide_balance));
            },
            CliIndicator { 
                label: label.to_string(), 
                is_active: is_dark 
            }
        }
    }
}

pub fn render_balance_toggle() -> Element {
    let global = use_context::<GlobalContext>();
    let (is_dark, hide_balance) = global.theme_user.read().clone();
    
    let is_visible = !hide_balance;
    let label = if is_visible { "HIDE" } else { "REVEAL" };

    rsx! {
        button {
            style: base_button_style(),
            // Push update to Channel Sender
            onclick: move |_| {
                let _ = CHANNEL.theme_user_tx.send((is_dark, !hide_balance));
            },
            CliIndicator { 
                label: label.to_string(), 
                is_active: is_visible 
            }
        }
    }
}