use dioxus_native::prelude::*;
use crate::utils::styles::terminal_action;

#[component]
pub fn SendAddressForm(
    network_label: String,
    address_buffer: Signal<String>,
    placeholder: String,
    current_error: Option<String>,
    on_input: EventHandler<FormEvent>,
    on_next_click: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        style { {r#"
            .send-step-container { display: flex; flex-direction: column; width: 100%; max-width: 800px; margin: 0 auto; font-family: 'JetBrains Mono', monospace; padding: 2rem; }
            .step-header { border-bottom: 1px solid var(--border); padding-bottom: 1rem; margin-bottom: 2.5rem; }
            .step-title { font-size: 0.7rem; color: var(--text-secondary); letter-spacing: 2px; }
            .input-section { margin-bottom: 2rem; }
            .input-label-row { display: flex; align-items: baseline; margin-bottom: 0.75rem; }
            .input-label { font-size: 0.65rem; color: var(--accent); border-left: 2px solid var(--accent); padding-left: 8px; }
            .terminal-input-wrapper { display: flex; align-items: center; background: var(--bg-grid); border: 1px solid var(--border); padding: 0.8rem 1rem; }
            .bracket { color: var(--text-secondary); opacity: 0.4; font-weight: bold; }
            .inner-input { flex: 1; background: transparent; border: none; outline: none; color: var(--text); font-family: inherit; font-size: 1rem; padding: 0 1rem; }
            .error-box { background: rgba(239, 68, 68, 0.1); border-left: 3px solid var(--status-warn); padding: 0.75rem 1rem; margin-top: 1rem; font-size: 0.75rem; color: var(--status-warn); }
            .footer-nav { margin-top: 2rem; display: flex; justify-content: flex-end; align-items: center; gap: 2rem; }
        "#} }

        div { class: "send-step-container",
            div { class: "step-header",
                div { class: "step-title", "TRANSACTION // STEP_01 // RECIPIENT_ADDRESS // {network_label}" }
            }

            div { class: "input-section",
                div { class: "input-label-row",
                    div { class: "input-label", "DESTINATION_ADDR" }
                }
                div { class: "terminal-input-wrapper",
                    span { class: "bracket", "[" }
                    input {
                        class: "inner-input",
                        value: "{address_buffer()}",
                        placeholder: "{placeholder}",
                        oninput: move |e| on_input.call(e),
                    }
                    span { class: "bracket", "]" }
                }

                if let Some(err) = current_error {
                    div { class: "error-box", "SIGNAL_INTERRUPT: {err}" }
                }
            }

            div { class: "footer-nav",
                {terminal_action("CONTINUE", true, move |e| on_next_click.call(e))}
            }
        }
    }
}