use dioxus_native::prelude::*;

// 1. The exact helper from your sidebar
fn base_button_style() -> String {
    "background: transparent; border: none; cursor: pointer; padding: 6px; display: flex; align-items: center;".to_string()
}

// 2. The exact indicator from your sidebar
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

// 3. The merged component using the exact same style function
#[component]
pub fn previous_icon_button(text_color: String) -> Element {
    rsx! {
        button {
            // Uses the sidebar style string: no border, 6px padding
            style: base_button_style(),
            CliIndicator { 
                label: "ABORT".to_string(), 
                is_active: true 
            }
        }
    }
}


pub fn terminal_action(
    label: &str, 
    active: bool, 
    mut on_click: impl FnMut(MouseEvent) + 'static 
) -> Element {
    let symbol = if active { ":" } else { "." };
    let symbol_color = if active { "var(--accent)" } else { "var(--text-secondary)" };
    
    rsx! {
        button {
            style: "background: transparent; border: none; cursor: pointer; padding: 8px; display: flex; align-items: center;",
            onclick: move |e| on_click(e),
            
            span {
                style: "font-family: 'JetBrains Mono', monospace; font-size: 0.8rem; font-weight: 700; letter-spacing: 1.5px;",
                span { style: "color: var(--text-secondary); opacity: 0.4;", "[" }
                span { style: "color: {symbol_color};", "{symbol}" }
                span { style: "color: var(--text); padding: 0 8px;", "{label}" }
                span { style: "color: {symbol_color};", "{symbol}" }
                span { style: "color: var(--text-secondary); opacity: 0.4;", "]" }
            }
        }
    }
}

/// Identical to terminal_action but with green text (var(--status-ok)) when active
pub fn nav_action(
    label: &str, 
    active: bool, 
    mut on_click: impl FnMut(MouseEvent) + 'static 
) -> Element {
    let symbol = if active { ":" } else { "." };
    let symbol_color = if active { "var(--accent)" } else { "var(--text-secondary)" };
    let label_color = if active { "var(--status-ok)" } else { "var(--text)" };
    
    rsx! {
        button {
            style: "background: transparent; border: none; cursor: pointer; padding: 8px; display: flex; align-items: center;",
            onclick: move |e| on_click(e),
            
            span {
                style: "font-family: 'JetBrains Mono', monospace; font-size: 0.8rem; font-weight: 700; letter-spacing: 1.5px;",
                span { style: "color: var(--text-secondary); opacity: 0.4;", "[" }
                span { style: "color: {symbol_color};", "{symbol}" }
                span { style: "color: {label_color}; padding: 0 8px;", "{label}" }
                span { style: "color: {symbol_color};", "{symbol}" }
                span { style: "color: var(--text-secondary); opacity: 0.4;", "]" }
            }
        }
    }
}