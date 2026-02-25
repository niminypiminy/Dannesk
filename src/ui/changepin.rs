use dioxus_native::prelude::*;
use crate::ui::pinlogic::PinLogic;
use crate::utils::styles::{terminal_action, previous_icon_button};

#[component]
pub fn view(on_back: EventHandler<()>) -> Element {
    let mut old_pin = use_signal(String::new);
    let mut new_pin = use_signal(String::new);
    let mut confirm_pin = use_signal(String::new);

    // Fixed logic: Call it with (()) to satisfy internal flow
    let mut on_submit = move |_| {
        if new_pin().is_empty() || old_pin().is_empty() { return; }
        if new_pin() != confirm_pin() { return; }
        
        tokio::spawn(PinLogic::change_pin(old_pin(), new_pin()));

        old_pin.set(String::new());
        new_pin.set(String::new());
        confirm_pin.set(String::new());
        on_back.call(()); 
    };

    rsx! {
        style { {r#"
            .pin-page-container {
                display: flex;
                flex-direction: column;
                width: 100%;
                max-width: 800px;
                margin: 0 auto;
                justify-content:center;
                padding-top: 8vh;
                padding-left: 2rem;
                padding-right: 2rem;
                font-family: 'JetBrains Mono', monospace;
            }
            .header-row {
                display: flex;
                justify-content: space-between;
                align-items: center;
                border-bottom: 1px solid var(--border);
                padding-bottom: 1rem;
                margin-bottom: 3rem;
            }
            .input-section { margin-bottom: 2rem; }
            .input-label {
                font-size: 0.65rem;
                color: var(--accent);
                border-left: 2px solid var(--accent);
                padding-left: 8px;
                margin-bottom: 0.75rem;
            }
            .terminal-input-wrapper {
                display: flex;
                align-items: center;
                background: var(--bg-grid);
                border: 1px solid var(--border);
                padding: 0.8rem 1rem;
                max-width: 300px;
            }
            .bracket { color: var(--text-secondary); opacity: 0.4; font-weight: bold; }
            .inner-input {
                flex: 1; background: transparent; border: none; outline: none;
                color: var(--text); font-family: inherit; font-size: 1rem; padding: 0 1rem;
            }
            .footer-actions {
                margin-top: 2rem;
                display: flex;
                align-items: center;
                gap: 2rem;
            }
        "#} }

        div { class: "pin-page-container",
            
            div { class: "header-row",
                div { 
                    style: "font-size: 0.7rem; color: var(--text-secondary); letter-spacing: 2px;", 
                    "SYSTEM_ADMIN // MANAGE_PIN" 
                }
                // The requested Abort/Back button from styles
                div { 
                    onclick: move |_| on_back.call(()),
                    previous_icon_button { text_color: "var(--text)".to_string() }
                }
            }

            // Inputs
            div { class: "input-section",
                div { class: "input-label", "CURRENT_AUTHORIZATION_PIN" }
                div { class: "terminal-input-wrapper",
                    span { class: "bracket", "[" }
                    input {
                        class: "inner-input",
                        value: "{old_pin}",
                        oninput: move |e| old_pin.set(e.value()),
                    }
                    span { class: "bracket", "]" }
                }
            }

            div { class: "input-section",
                div { class: "input-label", "NEW_PIN_SEQUENCE" }
                div { class: "terminal-input-wrapper",
                    span { class: "bracket", "[" }
                    input {
                        class: "inner-input",
                        value: "{new_pin}",
                        oninput: move |e| new_pin.set(e.value()),
                    }
                    span { class: "bracket", "]" }
                }
            }

            div { class: "input-section",
                div { class: "input-label", "CONFIRM_SEQUENCE" }
                div { class: "terminal-input-wrapper",
                    span { class: "bracket", "[" }
                    input {
                        class: "inner-input",
                        value: "{confirm_pin}",
                        oninput: move |e| confirm_pin.set(e.value()),
                        onkeydown: move |e| if e.key() == Key::Enter { on_submit(()); }
                    }
                    span { class: "bracket", "]" }
                }
            }

            div { class: "footer-actions",
                {terminal_action("EXECUTE_NEW_PIN", true, move |_| on_submit(()))}
                div { 
                    style: "font-size: 0.6rem; color: var(--text-secondary); opacity: 0.4;",
                    "ENCRYPTION_ACTIVE"
                    br {}
                    "LOCAL_STORAGE_ONLY"
                }
            }
        }
    }
}