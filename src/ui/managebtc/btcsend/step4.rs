//src/ui/managebtc/btcsend/step4.rs
//dependent upon utils/send_review_layout

use dioxus_native::prelude::*;
use crate::context::{BtcContext, GlobalContext};
use crate::utils::send_review_layout::render_send_review;

#[component]
pub fn view() -> Element {
    let btc_ctx = use_context::<BtcContext>();
    let global = use_context::<GlobalContext>();
    
    let mut btc_sign_transaction = btc_ctx.btc_sign_transaction;
    let rates = global.rates.read();
    let exchange_rate = rates.get("BTC/USD").copied().unwrap_or(0.0) as f64;
    
    let sign_state = btc_sign_transaction.read();
    let send_data = sign_state.send_transaction.as_ref();

    let recipient = send_data.and_then(|s| s.recipient.clone()).unwrap_or_else(|| "NULL".into());
    let amount = send_data.and_then(|s| s.amount.clone()).unwrap_or_else(|| "0.00".into());
    let fee = send_data.and_then(|s| Some(s.fee.clone())).unwrap_or_else(|| "0".into());
    
    let usd_amount = if let Ok(amt) = amount.parse::<f64>() {
        format!("{:.2}", amt * exchange_rate)
    } else {
        "0.00".into()
    };

    let on_confirm_click = move |_| {
        btc_sign_transaction.with_mut(|state| {
            if let Some(ref mut send) = state.send_transaction {
                send.step = 5;
                send.error = None;
            }
        });
    };

    let summary_rows = vec![
        ("RECIPIENT_ADDR".to_string(), recipient),
        ("SEND_QUANTITY".to_string(), format!("{} BTC", amount)),
        ("USD_VALUATION".to_string(), format!("${}", usd_amount)),
        ("MINER_FEE_TOTAL".to_string(), format!("{} SATS", fee)),
        ("NETWORK_ID".to_string(), "BITCOIN_MAINNET".to_string()),
    ];

    render_send_review(
        "TRANSACTION_INITIALIZATION // STEP_04 // REVIEW_TRANSACTION".to_string(),
        summary_rows,
        "CAUTION: Verify the recipient address carefully. Bitcoin transactions are immutable and cannot be reversed once broadcast.".to_string(),
        "BITCOIN_MAINNET".to_string(),
        on_confirm_click,
    )
}