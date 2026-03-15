// src/ui/managebtc/transactions.rs
use dioxus_native::prelude::*;
use crate::context::BtcContext;
use crate::channel::{BTCActiveView, BitcoinTransactionStatus};
use crate::utils::styles;
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

    let on_back_click = move |_: MouseEvent| {
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
                max-width: 1000px;
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
                width: 1000px; 
                min-width: 1000px; 
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
                flex: 1; 
                padding: 10px 8px;
                overflow: hidden; 
                white-space: nowrap; 
                text-overflow: ellipsis;
                border-right: 1px solid var(--bg-faint);
                font-size: 0.7rem;
            }
            .col:last-child { border-right: none; }
            .c-currency { color: var(--accent); font-weight: bold; }
            .c-date { color: var(--text-secondary); }
        "#} }

        div { class: "tx-container",
            div { 
                class: "back-button-container",
                onclick: on_back_click,
                styles::previous_icon_button { text_color: "var(--text)".to_string() }
            }

            div { class: "section-label", "NETWORK_LOG // BITCOIN_TRANSACTIONS" }

            div { class: "tx-table",
                div { class: "table-header",
                    div { class: "col", "TX_ID" }
                    div { class: "col", "ASSET" }
                    div { class: "col", "STATUS" }
                    div { class: "col", "AMOUNT" }
                    div { class: "col", "FEE" }
                    div { class: "col", "SENDER" }
                    div { class: "col", "RECV" }
                    div { class: "col c-date", "DATE" }
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
    let bg_color = if index % 2 == 0 { "transparent" } else { "var(--bg-faint)" };
    
    let (status_text, status_color) = match status {
        BitcoinTransactionStatus::Success => ("OK", "var(--status-ok)"),
        BitcoinTransactionStatus::Failed => ("FAIL", "var(--status-warn)"),
        BitcoinTransactionStatus::Pending => ("WAIT", "var(--accent)"),
        BitcoinTransactionStatus::Cancelled => ("VOID", "var(--text-secondary)"),
    };

    let short_id = if tx_id.len() > 8 { format!("{}..", &tx_id[..8]) } else { tx_id.clone() };
    
    let format_addresses = |addrs: &[String]| {
        if addrs.is_empty() { return "—".to_string(); }
        let joined = addrs.join(", ");
        if joined.len() > 10 { format!("{}..", &joined[..8]) } else { joined }
    };

    let full_senders = senders.join(", ");
    let full_receivers = receivers.join(", ");

    rsx! {
        div { 
            class: "table-row",
            style: "background-color: {bg_color};",

            div { class: "col", title: "{tx_id}", "{short_id}" }
            div { class: "col c-currency", "BTC" }
            div { class: "col", style: "color: {status_color}", "{status_text}" }
            div { class: "col", "{amount}" }
            div { class: "col", "{fee}" }
            div { class: "col", title: "{full_senders}", "{format_addresses(&senders)}" }
            div { class: "col", title: "{full_receivers}", "{format_addresses(&receivers)}" }
            div { class: "col c-date", "{format_timestamp(&timestamp)}" }
        }
    }
}