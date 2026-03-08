//src/utils/import_seed_layout.rs
use dioxus_native::prelude::*;
use crate::utils::styles::terminal_action;

#[component]
pub fn ImportSeedForm(
    network_label: String,
    mut seed_words: Signal<Vec<String>>,
    mut error_msg: Signal<Option<String>>,
    on_continue: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        style { {r#"
            .import-step-container {
                display: flex;
                flex-direction: column;
                width: 100%;
                max-width: 800px;
                margin: 0 auto;
                font-family: 'JetBrains Mono', monospace;
                padding: 2rem;
            }

            .step-header {
                border-bottom: 1px solid var(--border);
                padding-bottom: 1rem;
                margin-bottom: 1.5rem;
            }

            .step-title {
                font-size: 0.7rem;
                color: var(--text-secondary);
                letter-spacing: 2px;
            }

            .word-grid {
                display: grid; 
                grid-template-columns: repeat(4, 1fr); 
                gap: 0.75rem;
            }

            .input-cell {
                display: flex;
                align-items: center;
                background: var(--input-bg);
                border: 1px solid var(--border);
                padding: 0.2rem 0.5rem;
            }

            .cell-index {
                font-size: 0.6rem;
                color: var(--text-secondary);
                opacity: 0.5;
                width: 1.2rem;
                font-weight: bold;
            }

            .cell-input {
                width: 100%;
                background: transparent;
                border: none;
                outline: none;
                color: var(--text);
                font-size: 0.85rem;
                height: 1.8rem;
            }

            .error-banner {
                background: rgba(var(--status-warn-rgb), 0.1);
                color: var(--status-warn);
                border-left: 3px solid var(--status-warn);
                padding: 0.75rem 1rem;
                margin-bottom: 1.5rem;
                font-size: 0.7rem;
                letter-spacing: 1px;
            }

           .footer-meta {
                margin-top: 2rem; 
                display: flex; 
                justify-content: flex-end; 
                align-items: center; 
                gap: 2rem; 
            }
        "#} }

        div { class: "import-step-container",
            
            // 1. Header Row
            div { class: "step-header",
                div { class: "step-title", "WALLET_IMPORT // STEP_01 // MNEMONIC_ENTRY // {network_label} // CTRL V" }
            }

            // 2. The Grid Container
            div { class: "word-grid",
                for i in 0..24 {
                    div { key: "{i}", class: "input-cell",
                        span { class: "cell-index", "{i + 1:02}" }
                        input {
                            class: "cell-input",
                            value: "{seed_words.read()[i]}",
                            spellcheck: false,
                            autocomplete: "off",
                            oninput: move |evt| {
                                let val = evt.value().replace(['\n', '\r'], " ");
                                error_msg.set(None);
                                
                                if val.trim().contains(' ') {
                                    // Handle paste of multiple words
                                    let words: Vec<String> = val.split_whitespace().map(|s| s.to_string()).collect();
                                    let mut current_words = seed_words.peek().clone();
                                    for (j, word) in words.iter().enumerate() {
                                        if i + j < 24 {
                                            current_words[i + j] = word.trim().to_string();
                                        }
                                    }
                                    seed_words.set(current_words);
                                } else {
                                    // Handle single word typing
                                    let mut current_words = seed_words.peek().clone();
                                    current_words[i] = val.trim().to_string();
                                    seed_words.set(current_words);
                                }
                            }
                        }
                    }
                }
            }

            // 3. Error Handling
            if let Some(err) = error_msg() {
                div { class: "error-banner", ">> {err}" }
            }

            // 4. Action Footer
            div { class: "footer-meta",
                {terminal_action("VERIFY_STRUCTURE", true, move |e| on_continue.call(e))}
            }
        }
    }
}