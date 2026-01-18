// src/ui/sidebar.rs

use dioxus::prelude::*;
use crate::context::GlobalContext; 
use crate::channel::SettingsView;
use crate::utils::eyesvg::EyeIcon;
use crate::utils::themesvg::ThemeIcon;
use crate::utils::settingssvg::SettingsIcon;

// Shared style helper
fn base_button_style(color_var: &str) -> String {
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
            color: var({});
        ",
        color_var
    )
}

pub fn render_theme_toggle() -> Element {
    let mut global = use_context::<GlobalContext>();
    let (is_dark, user, hide_balance) = global.theme_user.read().clone();
    
    rsx! {
        button {
            style: base_button_style("--text"),
            onclick: move |_| {
                // Directly update the signal in the context
                global.theme_user.set((!is_dark, user.clone(), hide_balance));
            },
            ThemeIcon { dark: is_dark }
        }
    }
}

pub fn render_balance_toggle() -> Element {
    let mut global = use_context::<GlobalContext>();
    let (is_dark, user, hide_balance) = global.theme_user.read().clone();
    let open = !hide_balance;

    rsx! {
        button {
            style: base_button_style("--text-secondary"),
            onclick: move |_| {
                // Directly update the signal in the context
                global.theme_user.set((is_dark, user.clone(), !hide_balance));
            },
            EyeIcon { open }
        }
    }
}

pub fn render_settings_toggle() -> Element {
    let mut global = use_context::<GlobalContext>();
    
    rsx! {
        button {
            style: base_button_style("--text-secondary"),
            onclick: move |_| {
                // Open settings by setting the view to Name (default)
                // and ensuring we trigger the "Some" state for your gate logic
                global.settings_modal.with_mut(|state| {
                    state.view_type = SettingsView::Name;
                    state.last_view = Some(SettingsView::Name);
                });
            },
            SettingsIcon {}
        }
    }
}