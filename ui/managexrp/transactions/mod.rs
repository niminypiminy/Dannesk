// src/ui/managexrp/transactions.rs
use dioxus_native::prelude::*;
use crate::context::XrpContext;
use crate::channel::{ActiveView, TransactionStatus};
use crate::utils::styles;
use chrono::{DateTime, Utc};

#[component]
pub fn view() -> Element {
    let xrp_ctx = use_context::<XrpContext>();
    let mut xrp_modal = xrp_ctx.xrp_modal;
    let tx_state = xrp_ctx.transactions.read();

    let mut sorted_txs: Vec<_> = tx_state.transactions.values().collect();
    sorted_txs.sort_by_key(|tx| std::cmp::Reverse(
        tx.timestamp.parse::<DateTime<Utc>>().unwrap_or(DateTime::<Utc>::MIN_UTC)
    ));

    let display_txs = sorted_txs.into_iter().take(100).collect::<Vec<_>>();

    let on_back_click = move |_: MouseEvent| {
        xrp_modal.with_mut(|m| {
            if let Some(previous) = m.last_view.clone() {
                m.view_type = previous;
            } else {
                m.view_type = ActiveView::XRP;
            }
        });
    };

    rsx! {
        style { {r#"
            .tx-container {
                display: flex;
                flex-direction: column;
                width: 100%;
                align-items: center; 
                padding-top: 4rem;
                color: var(--text);
                font-family: 'JetBrains Mono', monospace;
            }
            .back-button-container {
                position: absolute;
                top: 0.75rem;
                left: 0.75rem;
                cursor: pointer;
                z-index: 10;
            }
            .section-label {
                width: 100%;
                max-width: 1500px;
                font-size: 0.65rem;
                color: var(--text-secondary);
                letter-spacing: 2px;
                border-left: 2px solid var(--accent);
                padding-left: 8px;
                margin-bottom: 1rem;
            }
            .tx-table {
                display: flex;
                flex-direction: column;
                width: 100%;
                max-width: 1500px; /* STRICT MAX WIDTH */
                min-width: 1500px; /
                border: 1px solid var(--border);
                background: var(--bg-primary);
            }
            .table-header {
                display: flex;
                flex-direction: row;
                background-color: var(--bg-grid);
                border-bottom: 1px solid var(--border);
                font-weight: 600;
                color: var(--text-secondary);
                font-size: 0.6rem;
            }
            .table-body {
                display: flex;
                flex-direction: column;
            }
            .table-row {
                display: flex;
                flex-direction: row;
                align-items: center;
                border-bottom: 1px solid var(--bg-faint);
            }
            .col { 
                flex: 1; /* All columns share the 900px equally */
                padding: 10px 8px;
                overflow: hidden; 
                white-space: nowrap; 
                text-overflow: ellipsis;
                border-right: 1px solid var(--bg-faint);
                font-size: 0.7rem;
            }
            .col:last-child { border-right: none; }
            .c-type { text-transform: uppercase; }
            .c-currency { color: var(--accent); font-weight: bold; }
            .c-date { color: var(--text-secondary); }
        "#} }

        div { class: "tx-container",
            div { 
                class: "back-button-container",
                onclick: on_back_click,
                styles::previous_icon_button { text_color: "var(--text)".to_string() }
            }

            div { class: "section-label", "NETWORK_LOG // XRPL_TRANSACTIONS" }

            div { class: "tx-table",
                div { class: "table-header",
                    div { class: "col", "TX_ID" }
                    div { class: "col", "TYPE" }
                    div { class: "col", "STATUS" }
                    div { class: "col", "PRICE" }
                    div { class: "col", "AMOUNT" }
                    div { class: "col", "ASSET" }
                    div { class: "col", "FEE" }
                    div { class: "col", "FLAGS" }
                    div { class: "col", "RECV" }
                    div { class: "col", "SEND" }
                    div { class: "col c-date", "DATE" }
                }

                div { class: "table-body",
                    for (i, tx) in display_txs.iter().enumerate() {
                        TransactionRow { 
                            key: "{tx.tx_id}",
                            index: i,
                            tx_id: tx.tx_id.clone(),
                            order_type: tx.order_type.clone(),
                            status: tx.status.clone(),
                            execution_price: tx.execution_price.clone(),
                            amount: tx.amount.clone(),
                            currency: tx.currency.clone(),
                            fee: tx.fee.clone(),
                            flags: tx.flags.clone(),
                            receiver: tx.receiver.clone(),
                            sender: tx.sender.clone(),
                            timestamp: tx.timestamp.clone(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TransactionRow(
    index: usize,
    tx_id: String,
    order_type: String,
    status: TransactionStatus,
    execution_price: String,
    amount: String,
    currency: String,
    fee: String,
    flags: Option<String>,
    receiver: String,
    sender: String,
    timestamp: String
) -> Element {
    let bg_color = if index % 2 == 0 { "transparent" } else { "var(--bg-faint)" };
    
    // Using .as_deref() to borrow the Option content instead of moving it
    let display_flags = flags.as_deref().unwrap_or("---");
    
    let (status_text, status_color) = match status {
        TransactionStatus::Success => ("OK", "var(--status-ok)"),
        TransactionStatus::Failed => ("FAIL", "var(--status-warn)"),
        TransactionStatus::Pending => ("WAIT", "var(--accent)"),
        TransactionStatus::Cancelled => ("VOID", "var(--text-secondary)"),
    };

    let short_id = if tx_id.len() > 8 { format!("{}..", &tx_id[..8]) } else { tx_id.clone() };
    let truncate_addr = |addr: &str| {
        if addr.len() > 8 { format!("{}..", &addr[..8]) } else { addr.to_string() }
    };

    rsx! {
        div { 
            class: "table-row",
            style: "background-color: {bg_color};",

            div { class: "col", title: "{tx_id}", "{short_id}" }
            div { class: "col c-type", "{order_type}" }
            div { class: "col", style: "color: {status_color}", "{status_text}" }
            div { class: "col", "{execution_price}" }
            div { class: "col", "{amount}" }
            div { class: "col c-currency", "{currency}" }
            div { class: "col", "{fee}" }
            div { class: "col", "{display_flags}" }
            div { class: "col", title: "{receiver}", "{truncate_addr(&receiver)}" }
            div { class: "col", title: "{sender}", "{truncate_addr(&sender)}" }
            div { class: "col c-date", "{timestamp}" }
        }
    }
}