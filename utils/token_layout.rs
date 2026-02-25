use dioxus_native::prelude::*;

pub fn render_token_layout(
    symbol: &str,
    currency_symbol: &str,
    int_part: &str,
    frac_part: &str,
    formatted_raw_amount: &str,
    limit_display: &str,
    rate: f64,
    send_btn: Element,
    receive_btn: Element,
) -> Element {
    rsx! {
        style { {r#"
            .manage-container {
                display: flex;
                flex-direction: column;
                width: 100%;
                max-width: 800px;
                margin: 0 auto;
                font-family: 'JetBrains Mono', monospace;
            }

            .balance-hero {
                padding: 2rem 0;
                border-bottom: 1px solid var(--border);
                margin-bottom: 2rem;
            }

            .action-grid {
                display: grid;
                grid-template-columns: 1fr 1fr;
                gap: 2rem;
            }

            .action-section {
                display: flex;
                flex-direction: column;
                gap: 1rem;
            }

            .section-label {
                font-size: 0.65rem;
                color: var(--text-secondary);
                letter-spacing: 2px;
                border-left: 2px solid var(--accent);
                padding-left: 8px;
                margin-bottom: 0.5rem;
            }

            .info-box {
                background: var(--bg-grid);
                border: 1px solid var(--border);
                padding: 1rem;
                display: flex;
                flex-direction: column;
                gap: 0.5rem;
            }

            .info-row {
                display: flex;
                justify-content: space-between;
                font-size: 0.75rem;
            }
        "#} }

        div { class: "manage-container",
            
            // 1. HERO BALANCE
            div { class: "balance-hero",
                div { style: "font-size: 0.7rem; color: var(--text-secondary); opacity: 0.6; margin-bottom: 0.5rem;", 
                    "ASSET_RESERVE // {symbol}" 
                }
                div { 
                    style: "font-size: 3.5rem; font-weight: 800; display: flex; align-items: baseline;", 
                    span { style: "font-size: 0.4em; color: var(--text-secondary); margin-right: 0.5rem;", "{currency_symbol}" }
                    span { "{int_part}" }
                    span { style: "font-size: 0.4em; color: var(--text-secondary);", "{frac_part}" }
                }
                div { style: "color: var(--accent); font-size: 0.9rem;", "{formatted_raw_amount} {symbol}" }
            }

            // 2. ORGANIZED ACTION GRID
            div { class: "action-grid",
                
                // Left Side: Financials
                div { class: "action-section",
                    div { class: "section-label", "TRANSFER_PROTOCOLS" }
                    div { style: "display: flex; gap: 10px;",
                        {send_btn},
                        {receive_btn}
                    }
                }

                // Right Side: Trustline / Metadata
                div { class: "action-section",
                    div { class: "section-label", "LEDGER_CONSTRAINTS" }
                    div { class: "info-box",
                        div { class: "info-row",
                            span { style: "color: var(--text-secondary);", "LIMIT:" }
                            span { style: "color: var(--text); font-weight: bold;", "{limit_display}" }
                        }
                        div { class: "info-row",
                            span { style: "color: var(--text-secondary);", "STATUS:" }
                            span { style: "color: var(--status-ok);", "ACTIVE" }
                        }
                    }
                }
            }

            // 3. FOOTER
            div { 
                style: "margin-top: 4rem; color: var(--text-secondary); font-size: 0.65rem; opacity: 0.3; letter-spacing: 1px;", 
                "RATE_INDEX // {currency_symbol}{rate:.4} // STABLE_ASSET" 
            }
        }
    }
}