//src/utils/send_review_layout.rs

use dioxus_native::prelude::*;
use crate::utils::styles::terminal_action;

pub fn render_send_review(
    step_title: String,
    summary_rows: Vec<(String, String)>,
    warning_text: String,
    network_label: String,
    on_confirm_click: impl FnMut(MouseEvent) + 'static,
) -> Element {
    rsx! {
        style { {r#"
            .send-step-container {
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

            
            .summary-box {
                display: grid;
                grid-template-columns: 1fr;
                gap: 1px;
                background: var(--border); 
                border: 1px solid var(--border);
                width: 100%;
            }

            .summary-row {
                display: grid;
                grid-template-columns: 140px 1fr; 
                background: var(--bg-grid);
                padding: 1.25rem 1rem;
                align-items: start;
            }

            .row-label {
                font-size: 0.65rem;
                color: var(--accent);
                margin-top: 0.2rem;
            }

            .row-value {
                font-size: 0.9rem;
                color: var(--text);
                word-break: break-all;
                line-height: 1.4;
                text-align: right;
            }

            .warning-footer {
                margin-top: 2rem;
                padding: 1rem;
                background: rgba(var(--status-warn-rgb), 0.05);
                border: 1px dashed var(--border);
                color: var(--text-secondary);
                font-size: 0.7rem;
                line-height: 1.5;
            }

            .footer-nav { margin-top: 2rem; display: flex; justify-content: flex-end; align-items: center; gap: 2rem; }
        "#} }

        div { class: "send-step-container",
            
            div { class: "step-header",
                div { class: "step-title", "{step_title} // {network_label}" }

            }

            div { class: "summary-box",
                for (label, value) in summary_rows {
                    div { class: "summary-row",
                        div { class: "row-label", "{label}" }
                        div { class: "row-value", "{value}" }
                    }
                }
            }

            div { class: "warning-footer", "{warning_text}" }

            div { class: "footer-nav",
                {terminal_action("CONTINUE", true, on_confirm_click)}
            }
        }
    }
}