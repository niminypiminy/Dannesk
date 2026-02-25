use dioxus_native::prelude::*;
use crate::utils::styles::terminal_action;

#[component]
pub fn SendAuthForm(
    step_title: String, // Added parameter
    network_label: String,
    input_mode: Signal<String>,
    passphrase_val: Signal<String>,
    seed_words: Signal<Vec<String>>,
    bip39_val: Signal<String>,
    current_error: Option<String>,
    on_submit: EventHandler<MouseEvent>,
    on_clear_error: EventHandler<()>,
) -> Element {
    rsx! {
        style { {r#"
            .send-step-container { display: flex; flex-direction: column; width: 100%; max-width: 800px; margin: 0 auto; font-family: 'JetBrains Mono', monospace; padding: 2rem; }
            .step-header { border-bottom: 1px solid var(--border); padding-bottom: 1rem; margin-bottom: 2rem; }
            .step-title { font-size: 0.7rem; color: var(--text-secondary); letter-spacing: 2px; }
            .auth-tabs { display: flex; gap: 2rem; margin-bottom: 2rem; border-bottom: 1px solid var(--border); }
            .auth-tab { padding: 0.5rem 0; font-size: 0.7rem; background: transparent; border: none; cursor: pointer; color: var(--text-secondary); position: relative; }
            .auth-tab-active { color: var(--accent); border-bottom: 2px solid var(--accent); }
            .input-section { margin-bottom: 1.5rem; }
            .input-label-row { display: flex; align-items: baseline; margin-bottom: 0.75rem; gap: 1rem; }
            .input-label { font-size: 0.65rem; color: var(--accent); border-left: 2px solid var(--accent); padding-left: 8px; }
            .input-hint { font-size: 0.6rem; color: var(--text-secondary); opacity: 0.6; }
            .terminal-input-wrapper { display: flex; align-items: center; background: var(--bg-grid); border: 1px solid var(--border); padding: 0.8rem 1rem; }
            .bracket { color: var(--text-secondary); opacity: 0.4; font-weight: bold; }
            .inner-input { flex: 1; background: transparent; border: none; outline: none; color: var(--text); font-family: inherit; font-size: 1rem; padding: 0 1rem; }
            .word-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 0.75rem; }
            .input-cell { display: flex; align-items: center; background: var(--input-bg); border: 1px solid var(--border); padding: 0.2rem 0.5rem; }
            .cell-index { font-size: 0.6rem; color: var(--text-secondary); opacity: 0.5; width: 1.2rem; font-weight: bold; }
            .cell-input { width: 100%; background: transparent; border: none; outline: none; color: var(--text); font-size: 0.85rem; height: 1.8rem; font-family: inherit; }
            .error-box { background: rgba(239, 68, 68, 0.1); border-left: 3px solid var(--status-warn); padding: 0.75rem 1rem; margin-top: 1rem; font-size: 0.75rem; color: var(--status-warn); }
            .footer-nav { margin-top: 2rem; display: flex; justify-content: space-between; align-items: center; }
        "#} }

        div { class: "send-step-container",
            div { class: "step-header",
                // Use the passed step_title here
                div { class: "step-title", "{step_title} // {network_label}" }
            }

            div { class: "auth-tabs",
                button { 
                    class: if input_mode() == "passphrase" { "auth-tab auth-tab-active" } else { "auth-tab" },
                    onclick: move |_| {
                        input_mode.set("passphrase".to_string());
                        on_clear_error.call(());
                    },
                    "DECRYPTION_PASSPHRASE"
                }
                button { 
                    class: if input_mode() == "seed" { "auth-tab auth-tab-active" } else { "auth-tab" },
                    onclick: move |_| {
                        input_mode.set("seed".to_string());
                        on_clear_error.call(());
                    },
                     "MNEMONIC_SEED"
                }
            }

            if input_mode() == "passphrase" {
                div { class: "input-section",
                    div { class: "input-label-row", div { class: "input-label", "ENCRYPTION_KEY" } }
                    div { class: "terminal-input-wrapper",
                        span { class: "bracket", "[" }
                        input {
                            class: "inner-input",
                            r#type: "password",
                            value: "{passphrase_val()}",
                            placeholder: "ENTER_PASSPHRASE",
                            oninput: move |e| passphrase_val.set(e.value())
                        }
                        span { class: "bracket", "]" }
                    }
                }
            } else {
                div { class: "input-section",
                    div { class: "input-label-row", 
                        div { class: "input-label", "MNEMONIC_MATRIX" } 
                        div { class: "input-hint", "CTRL V" }
                    }
                    div { class: "word-grid",
                        for i in 0..24 {
                            div { key: "{i}", class: "input-cell",
                                span { class: "cell-index", "{i + 1:02}" }
                                input {
                                    class: "cell-input",
                                    value: "{seed_words.read()[i]}",
                                    oninput: move |evt| {
                                        let val = evt.value();
                                        if val.trim().contains(' ') {
                                            let words: Vec<String> = val.split_whitespace().map(|s| s.to_string()).collect();
                                            let mut current = seed_words.peek().clone();
                                            for (j, word) in words.iter().enumerate() {
                                                if i + j < 24 { current[i + j] = word.clone(); }
                                            }
                                            seed_words.set(current);
                                        } else {
                                            let mut current = seed_words.peek().clone();
                                            current[i] = val.trim().to_string();
                                            seed_words.set(current);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "input-section",
                div { class: "input-label-row", 
                    div { class: "input-label", "BIP39_PASSPHRASE" } 
                    div { class: "input-hint", "[OPTIONAL]" }
                }
                div { class: "terminal-input-wrapper",
                    span { class: "bracket", "[" }
                    input {
                        class: "inner-input",
                        value: "{bip39_val()}",
                        oninput: move |e| bip39_val.set(e.value())
                    }
                    span { class: "bracket", "]" }
                }
            }

            if let Some(err) = current_error {
                div { class: "error-box", "SIGNAL_INTERRUPT: {err}" }
            }

            div { class: "footer-nav",
                {terminal_action("EXECUTE_BROADCAST", true, move |ev| on_submit.call(ev))}
            }
        }
    }
}