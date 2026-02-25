//src/ui/managexrp/xrpsend/step3.rs
// dependent on utils/send_review_layout.rs

use dioxus_native::prelude::*;
use crate::context::{XrpContext, GlobalContext};
use crate::utils::send_review_layout::render_send_review;

#[component]
pub fn view() -> Element {
    let xrp_ctx = use_context::<XrpContext>();
    let global = use_context::<GlobalContext>();
    
    let mut sign_transaction = xrp_ctx.sign_transaction;
    let rates = global.rates.read();
    let exchange_rate = rates.get("XRP/USD").copied().unwrap_or(0.0) as f64;
    
    let sign_state = sign_transaction.read();
    let send_data = sign_state.send_transaction.as_ref();

    let recipient = send_data.and_then(|s| s.recipient.clone()).unwrap_or_else(|| "NULL".into());
    let amount = send_data.and_then(|s| s.amount.clone()).unwrap_or_else(|| "0.00".into());
    let asset = send_data.map(|s| s.asset.clone()).unwrap_or_else(|| "XRP".into());

    let usd_amount = if asset == "XRP" {
        if let Ok(amt) = amount.parse::<f64>() {
            format!("{:.2}", amt * exchange_rate)
        } else {
            "0.00".into()
        }
    } else {
        amount.clone()
    };

    let on_confirm_click = move |_| {
        sign_transaction.with_mut(|state| {
            if let Some(ref mut send) = state.send_transaction {
                send.step = 4;
                send.error = None;
            }
        });
    };

    // Dynamically build rows (handling the RLUSD condition)
    let mut summary_rows = vec![
        ("RECIPIENT_ADDR".to_string(), recipient),
        ("SEND_QUANTITY".to_string(), format!("{} {}", amount, asset)),
    ];
    
    if asset != "RLUSD" {
        summary_rows.push(("USD_VALUATION".to_string(), format!("${}", usd_amount)));
    }
    
    summary_rows.push(("NETWORK_ID".to_string(), "XRP_LEDGER_MAINNET".to_string()));

    render_send_review(
        "TRANSACTION_INITIALIZATION // STEP_03 // REVIEW_TRANSACTION".to_string(),
        summary_rows,
        "CAUTION: Verify the recipient address carefully. Ledger transactions are immutable and cannot be reversed once broadcast.".to_string(),
        "XRPL_MAINNET".to_string(),
        on_confirm_click,
    )
}