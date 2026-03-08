//src/utils/wallet_security_layout.rs
//this is used in both import and create step2.rs

use dioxus_native::prelude::*;
use crate::utils::styles::terminal_action;

#[component]
pub fn WalletSecurityForm(
    flow_label: String,
    network_label: String,
    action_label: String,
    mut bip39_buffer: Signal<String>,
    mut encryption_buffer: Signal<String>,
    current_error: Option<String>,
    on_action_click: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        style { {r#"
            /* Changed selector name to match the HTML below */
            .terminal-step-container {
                display: flex;
                flex-direction: column;
                width: 100%;
                max-width: 800px;
                margin: 0 auto;
                font-family: 'JetBrains Mono', monospace;
                padding: 2rem;
            }
            .step-header { border-bottom: 1px solid var(--border); padding-bottom: 1rem; margin-bottom: 2.5rem; }
            .step-title { font-size: 0.7rem; color: var(--text-secondary); letter-spacing: 2px; }
            .input-section { margin-bottom: 2rem; }
            .input-label-row { display: flex; justify-content: space-between; align-items: baseline; margin-bottom: 0.75rem; }
            .input-label { font-size: 0.65rem; color: var(--accent); border-left: 2px solid var(--accent); padding-left: 8px; }
            .input-hint { font-size: 0.6rem; color: var(--text-secondary); opacity: 0.6; }
            .terminal-input-wrapper { display: flex; align-items: center; background: var(--bg-grid); border: 1px solid var(--border); padding: 0.8rem 1rem; }
            .terminal-input-wrapper:focus-within { border-color: var(--accent); }
            .bracket { color: var(--text-secondary); opacity: 0.4; font-weight: bold; user-select: none; }
            .inner-input { flex: 1; background: transparent; border: none; outline: none; color: var(--text); font-family: inherit; font-size: 1rem; padding: 0 1rem; }
            .error-box { background: rgba(var(--status-warn-rgb), 0.1); border-left: 3px solid var(--status-warn); padding: 0.75rem 1rem; margin-top: 1rem; font-size: 0.75rem; color: var(--status-warn); }
            .footer-nav { margin-top: 2rem; display: flex; justify-content: flex-end; align-items: center; gap: 2rem; }
        "#} }

        // This class now correctly maps to the CSS above
        div { class: "terminal-step-container",
            div { class: "step-header",
                div { class: "step-title", "WALLET_{flow_label} // STEP_02 // {network_label} // ENCRYPTION_PROTOCOL // AES-256-GCM" }
            }

            // 1. BIP39 Section
            div { class: "input-section",
                div { class: "input-label-row",
                    div { class: "input-label", "BIP39_PASSPHRASE" }
                    div { class: "input-hint", "[OPTIONAL_25TH_WORD]" }
                }
                div { class: "terminal-input-wrapper",
                    span { class: "bracket", "[" }
                    input {
                        class: "inner-input",
                        value: "{bip39_buffer()}",
                        oninput: move |e| bip39_buffer.set(e.value()),
                        placeholder: "NULL",
                    }
                    span { class: "bracket", "]" }
                }
            }

            // 2. Encryption Section
            div { class: "input-section",
                div { class: "input-label-row",
                    div { class: "input-label", "LOCAL_ENCRYPTION_KEY" }
                    div { class: "input-hint", "[MIN_10_CHARS_REQUIRED]" }
                }
                div { class: "terminal-input-wrapper",
                    span { class: "bracket", "[" }
                    input {
                        class: "inner-input",
                        r#type: "password",
                        value: "{encryption_buffer()}",
                        oninput: move |e| encryption_buffer.set(e.value()),
                        placeholder: "ENTER_PASSPHRASE",
                    }
                    span { class: "bracket", "]" }
                }

                if let Some(err) = current_error {
                    div { class: "error-box", "SIGNAL_INTERRUPT: {err}" }
                }
            }

            div { class: "footer-nav",
                {terminal_action(&action_label, true, move |e| on_action_click.call(e))}
            }
        }
    }
}