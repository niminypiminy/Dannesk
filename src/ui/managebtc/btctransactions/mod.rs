// src/ui/managebtc/transactions.rs
use dioxus::prelude::*;
use crate::context::{BtcContext}; // Added GlobalContext
use crate::channel::{BTCActiveView, BitcoinTransactionStatus};
use crate::utils::styles;
use crate::utils::btcsvg::BtcIcon;
use chrono::{DateTime, Utc, TimeZone};

fn parse_timestamp(ts: &str) -> DateTime<Utc> {
    ts.parse::<i64>()
        .ok()
        .and_then(|s| Utc.timestamp_opt(s, 0).single())
        .unwrap_or(DateTime::<Utc>::MIN_UTC)
}

fn format_timestamp(ts: &str) -> String {
    if ts.is_empty() || ts == "0" {
        "—".to_string()
    } else {
        parse_timestamp(ts)
            .format("%Y-%m-%d %H:%M")
            .to_string()
    }
}

#[component]
pub fn view() -> Element {
    let mut btc_ctx = use_context::<BtcContext>();
    let tx_state = btc_ctx.btc_transactions.read();

    let mut sorted_txs: Vec<_> = tx_state.transactions.values().collect();
    sorted_txs.sort_by_key(|tx| std::cmp::Reverse(parse_timestamp(&tx.timestamp)));

    let display_txs = sorted_txs.into_iter().take(100).collect::<Vec<_>>();
    let is_empty = display_txs.is_empty();

    let on_back_click = move |_| {
        btc_ctx.btc_modal.with_mut(|state| {
            state.view_type = BTCActiveView::BTC;
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
            
            .c-tx-id { width: 15%; }
            .c-asset { width: 8%; display: flex; align-items: center; gap: 4px; }
            .c-status { width: 10%; }
            .c-amount { width: 12%; }
            .c-fee { width: 8%; }
            .c-senders { width: 16%; }
            .c-receivers { width: 16%; }
            .c-date { width: 15%; text-align: right; }
        "#} }

        div { class: "tx-container",
            div { 
                class: "back-button-container",
                onclick: on_back_click,
                styles::previous_icon_button { text_color: "#fff".to_string() }
            }

            if is_empty {
                div { class: "empty-state",
                    h2 { "No Bitcoin transactions yet" }
                }
            } else {
                div { class: "table-header",
                    div { class: "col c-tx-id", "Tx ID" }
                    div { class: "col c-asset", "Asset" }
                    div { class: "col c-status", "Status" }
                    div { class: "col c-amount", "Amount" }
                    div { class: "col c-fee", "Fee" }
                    div { class: "col c-senders", "Sender" }
                    div { class: "col c-receivers", "Receiver" }
                    div { class: "col c-date", "Date" }
                }

                div { class: "table-body",
                    for (i, tx) in display_txs.iter().enumerate() {
                        TransactionRow { 
                            key: "{tx.txid}",
                            index: i,
                            tx_id: tx.txid.clone(),
                            status: tx.status.clone(),
                            amount: tx.amount.clone(),
                            fee: tx.fees.clone(),
                            receivers: tx.receiver_addresses.clone(),
                            senders: tx.sender_addresses.clone(),
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
    status: BitcoinTransactionStatus,
    amount: String,
    fee: String,
    receivers: Vec<String>,
    senders: Vec<String>,
    timestamp: String
) -> Element {
    // Zebra striping using theme variables
    let bg_color = if index % 2 == 0 { "transparent" } else { "var(--bg-faint)" };
    
    let short_id = if tx_id.len() > 15 { format!("{}...", &tx_id[..15]) } else { tx_id.clone() };
    
    let format_addresses = |addrs: &[String]| {
        if addrs.is_empty() { return "—".to_string(); }
        let joined = addrs.join(", ");
        if joined.len() > 18 { format!("{}...", &joined[..15]) } else { joined }
    };
    
    let full_senders = senders.join(", ");
    let full_receivers = receivers.join(", ");
    let display_senders = format_addresses(&senders);
    let display_receivers = format_addresses(&receivers);

    // Semantic status colors adjusted for light mode contrast
    let (status_text, status_color) = match status {
        BitcoinTransactionStatus::Success => ("Success", "rgb(0, 180, 0)"),
        BitcoinTransactionStatus::Failed => ("Failed", "rgb(220, 40, 40)"),
        BitcoinTransactionStatus::Pending => ("Pending", "rgb(220, 160, 0)"),
        BitcoinTransactionStatus::Cancelled => ("Cancelled", "var(--text-secondary)"),
    };

    let formatted_date = format_timestamp(&timestamp);

    rsx! {
        div { 
            class: "table-row",
            style: "background-color: {bg_color};",

            div { class: "col c-tx-id", title: "{tx_id}", "{short_id}" }
            
            div { class: "col c-asset",
                BtcIcon {}
            }

            div { class: "col c-status", style: "color: {status_color}", "{status_text}" }
            div { class: "col c-amount", "{amount}" }
            div { class: "col c-fee", "{fee}" }
            div { class: "col c-senders", title: "{full_senders}", "{display_senders}" }
            div { class: "col c-receivers", title: "{full_receivers}", "{display_receivers}" }
            div { class: "col c-date", title: "{timestamp}", "{formatted_date}" }
        }
    }
}