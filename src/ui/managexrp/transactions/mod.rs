// src/ui/managexrp/transactions.rs
use dioxus::prelude::*;
use crate::context::{XrpContext, GlobalContext};
use crate::channel::{ActiveView, TransactionStatus};
use crate::utils::styles;
use crate::utils::xrpsvg::XrpIcon;
use crate::utils::rlusdsvg::RlusdIcon;
use crate::utils::europsvg::EuropIcon;
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
    let is_empty = display_txs.is_empty();

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
                padding-top: 5rem;
                color: var(--text);
            }
            .back-button-container {
                position: absolute;
                top: 0.75rem;
                left: 0.75rem;
                cursor: pointer;
                z-index: 10;
            }
            .empty-state {
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                height: 100%; 
                color: var(--text-secondary);
            }
            .table-header {
                display: flex;
                flex-direction: row;
                background-color: var(--bg-secondary);
                padding: 10px 5px;
                border-bottom: 1px solid var(--border);
                font-weight: bold;
                color: var(--text);
                font-size: 0.85rem;
                font-family: monospace;
            }
            .table-body {
                display: flex;
                flex-direction: column;
                overflow-y: auto;
                flex: 1;
                width: 100%;
            }
            .table-row {
                display: flex;
                flex-direction: row;
                padding: 10px 5px;
                align-items: center;
                border-bottom: 1px solid var(--bg-faint);
                font-size: 0.8rem;
                font-family: monospace;
                color: var(--text);
            }
            .col { padding: 0 4px; overflow: hidden; white-space: nowrap; text-overflow: ellipsis; }
            .c-tx-id { width: 12%; }
            .c-type { width: 8%; }
            .c-status { width: 8%; }
            .c-price { width: 8%; }
            .c-amount { width: 8%; }
            .c-currency { width: 10%; display: flex; align-items: center; gap: 4px; }
            .c-fee { width: 6%; }
            .c-flags { width: 6%; }
            .c-receiver { width: 11%; }
            .c-sender { width: 11%; }
            .c-date { width: 12%; text-align: right; }
        "#} }

        div { class: "tx-container",
            div { 
                class: "back-button-container",
                onclick: on_back_click,
                // Use var(--text) so the arrow changes color with theme
                styles::previous_icon_button { text_color: "#fff".to_string() }
            }

            if is_empty {
                div { class: "empty-state",
                    h2 { "No XRP transactions yet" }
                }
            } else {
                div { class: "table-header",
                    div { class: "col c-tx-id", "Tx ID" }
                    div { class: "col c-type", "Type" }
                    div { class: "col c-status", "Status" }
                    div { class: "col c-price", "Price" }
                    div { class: "col c-amount", "Amount" }
                    div { class: "col c-currency", "Asset" }
                    div { class: "col c-fee", "Fee" }
                    div { class: "col c-flags", "Flags" }
                    div { class: "col c-receiver", "Receiver" }
                    div { class: "col c-sender", "Sender" }
                    div { class: "col c-date", "Date" }
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
    let global = use_context::<GlobalContext>();
    let (is_dark, _, _) = global.theme_user.read().clone();
    
    // Use var(--bg-faint) for zebra striping instead of hardcoded #111
    let bg_color = if index % 2 == 0 { "transparent" } else { "var(--bg-faint)" };
    
    let short_id = if tx_id.len() > 15 { format!("{}...", &tx_id[..15]) } else { tx_id.clone() };
    let truncate_addr = |addr: &str| {
        if addr.len() > 15 { format!("{}...", &addr[..15]) } else { addr.to_string() }
    };

    // Keep these specific semantic colors as they usually work on both themes
    let (status_text, status_color) = match status {
        TransactionStatus::Success => ("Success", "rgb(0, 180, 0)"), // Slightly darker green for light mode readability
        TransactionStatus::Failed => ("Failed", "rgb(220, 40, 40)"),
        TransactionStatus::Pending => ("Pending", "rgb(220, 160, 0)"),
        TransactionStatus::Cancelled => ("Cancelled", "var(--text-secondary)"),
    };

    let display_price = if execution_price.is_empty() || execution_price == "0" { "—" } else { &execution_price };
    let display_amount = if amount.is_empty() { "—" } else { &amount };
    let display_fee = if fee.is_empty() { "—" } else { &fee };
    let display_flags = flags.as_deref().filter(|s| !s.is_empty()).unwrap_or("—");

    rsx! {
        div { 
            class: "table-row",
            style: "background-color: {bg_color};",

            div { class: "col c-tx-id", title: "{tx_id}", "{short_id}" }
            div { class: "col c-type", "{order_type}" }
            div { class: "col c-status", style: "color: {status_color}", "{status_text}" }
            div { class: "col c-price", "{display_price}" }
            div { class: "col c-amount", "{display_amount}" }
            
            div { class: "col c-currency",
                if currency == "XRP" {
                    XrpIcon { dark: is_dark }
                    span { "{currency}" }
                } else if currency == "RLUSD" {
                    RlusdIcon {}
                    span { "RLUSD" }
                } else if currency == "EUROP" {
                    EuropIcon {}
                    span { "EUROP" }
                } else {
                    span { "{currency}" }
                }
            }

            div { class: "col c-fee", "{display_fee}" }
            div { class: "col c-flags", "{display_flags}" }
            div { class: "col c-receiver", title: "{receiver}", "{truncate_addr(&receiver)}" }
            div { class: "col c-sender", title: "{sender}", "{truncate_addr(&sender)}" }
            div { class: "col c-date", "{timestamp}" }
        }
    }
}