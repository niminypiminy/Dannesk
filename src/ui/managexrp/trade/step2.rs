use dioxus_native::prelude::*;
use crate::context::XrpContext;
use crate::utils::send_review_layout::render_send_review;

#[component]
pub fn view() -> Element {
    let mut xrp_ctx = use_context::<XrpContext>();
    
    let trade_read = xrp_ctx.trade.read();
    let Some(inner) = trade_read.send_trade.as_ref() else {
        return rsx! { "NO_TRADE_DATA_FOUND" };
    };

    // Extracting data for the review
    let base_asset = inner.base_asset.clone().unwrap_or_default();
    let quote_asset = inner.quote_asset.clone().unwrap_or_default();
    let amount = inner.amount.clone().unwrap_or_default();
    let limit_price = inner.limit_price.clone().unwrap_or_default();
    
    // Formatting Flags (e.g., tfPartialPayment -> PARTIALPAYMENT)
    let flags = inner.flags.clone().unwrap_or_default();
    let flags_display = if flags.is_empty() {
        "NONE".to_string()
    } else {
        flags.iter()
            .map(|f| f.strip_prefix("tf").unwrap_or(f).to_uppercase()) 
            .collect::<Vec<_>>()
            .join(", ")
    };

    // Formatting Fee
    let fee_display = if inner.fee_percentage == 0.0 {
        "AUTO".to_string()
    } else {
        format!("{:.2}%", inner.fee_percentage)
    };

    // Callback for the Confirm Action
    let on_confirm_click = move |_| {
        xrp_ctx.trade.with_mut(|s| if let Some(ref mut t) = s.send_trade {
            t.step = 3;
            t.error = None;
        });
    };

    // Construct the standardized summary rows
    let summary_rows = vec![
        ("TARGET_PAIR".to_string(), format!("{base_asset} / {quote_asset}")),
        ("ORDER_TYPE".to_string(), "LIMIT_ORDER".to_string()),
        ("BUY_QUANTITY".to_string(), format!("{amount} {base_asset}")),
        ("PRICE_LIMIT".to_string(), format!("{limit_price} {quote_asset}")),
        ("FEE_TOLERANCE".to_string(), fee_display),
        ("EXECUTION_FLAGS".to_string(), flags_display),
    ];

    render_send_review(
        "MARKET_ORDER_REVIEW // STEP_02 // VALIDATION".to_string(),
        summary_rows,
        ">> ATTENTION: VERIFY_ORDER_DETAILS. BROADCASTING_TO_XRPL_LEDGER_IS_IRREVERSIBLE. ENSURE_TRANSACTION_IS_CORRECT.".to_string(),
        "XRPL_MAINNET".to_string(),
        on_confirm_click,
    )
}