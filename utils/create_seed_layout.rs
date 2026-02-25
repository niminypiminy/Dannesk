use dioxus_native::prelude::*;
use crate::utils::styles::terminal_action;

#[component]
pub fn CreateSeedForm(
    network_label: String,
    words: Vec<String>,
    on_copy: EventHandler<MouseEvent>,
    on_continue: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        style { {r#"
            .create-step-container {
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

            .display-cell {
                display: flex;
                align-items: center;
                background: var(--bg-grid);
                border: 1px solid var(--border);
                padding: 0.5rem 0.75rem;
            }

            .display-cell:hover { border-color: var(--accent); }

            .cell-index {
                font-size: 0.6rem;
                color: var(--accent);
                opacity: 0.5;
                width: 1.2rem;
                font-weight: bold;
            }

            .cell-text {
                color: var(--text);
                font-size: 0.85rem;
                font-weight: 500;
            }

            .warning-banner {
                background: rgba(var(--status-warn-rgb), 0.1);
                color: var(--status-warn);
                border-left: 3px solid var(--status-warn);
                padding: 0.75rem 1rem;
                margin-top: 1.5rem;
                font-size: 0.7rem;
                letter-spacing: 1px;
            }

            .footer-nav { margin-top: 2rem; display: flex; justify-content: flex-start; align-items: center; gap: 2rem; }
        "#} }

        div { class: "create-step-container",
            // 1. Header
            div { class: "step-header",
                div { class: "step-title", "WALLET_CREATION // STEP_01 // SEED_PHRASE // {network_label}" }
            }

            // 2. Read-only Grid
            div { class: "word-grid",
                for (i, word) in words.iter().enumerate() {
                    div { key: "{i}", class: "display-cell",
                        span { class: "cell-index", "{i + 1:02}" }
                        span { class: "cell-text", "{word}" }
                    }
                }
            }

            // 3. Warning
            div { class: "warning-banner", 
                ">> CAUTION: Do not lose this seed phrase. If you lose the seed, you lose your coins." 
            }

            // 4. Actions
            div { class: "footer-nav",
                div { style: "display: flex; gap: 1rem;",
                    {terminal_action("COPY", false, move |e| on_copy.call(e))},
                    {terminal_action("CONTINUE", true, move |e| on_continue.call(e))}
                }
            }
        }
    }
}