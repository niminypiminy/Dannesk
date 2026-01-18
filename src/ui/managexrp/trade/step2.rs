// src/ui/managexrp/trade/step2.rs (Review Screen)

use dioxus::prelude::*;
use crate::context::XrpContext;

#[component]
pub fn view() -> Element {
    let mut xrp_ctx = use_context::<XrpContext>();
    
    // Read the current trade state
    let trade_read = xrp_ctx.trade.read();
    let Some(inner) = trade_read.send_trade.as_ref() else {
        return rsx! { "No trade data found" };
    };

    // Extract data safely
    let base_asset = inner.base_asset.clone().unwrap_or_default();
    let quote_asset = inner.quote_asset.clone().unwrap_or_default();
    let amount = inner.amount.clone().unwrap_or_default();
    let limit_price = inner.limit_price.clone().unwrap_or_default();
    
    // Format Flags: Remove "tf" prefix for cleaner display
    let flags = inner.flags.clone().unwrap_or_default();
    let flags_display = if flags.is_empty() {
        "None".to_string()
    } else {
        flags.iter()
            .map(|f| f.strip_prefix("tf").unwrap_or(f)) 
            .collect::<Vec<_>>()
            .join(", ")
    };

    // Format Fees
    let fee_display = if inner.fee_percentage == 0.0 {
        "Auto".to_string()
    } else {
        format!("{:.2}%", inner.fee_percentage)
    };

  rsx! {
        div {
            style: "display: flex; flex-direction: column; width: 100%; align-items: center;",

            // Grid container matched to 33rem
            div {
                style: "width: 100%; max-width: 33rem; background-color: #1a1a1a; border: 1px solid #333; border-radius: 0.25rem; display: flex; flex-direction: column;",
                
                ReviewRow { label: "Buy Asset".to_string(), value: base_asset.clone(), is_alt: false }
                ReviewRow { label: "Sell Asset".to_string(), value: quote_asset.clone(), is_alt: true }
                ReviewRow { label: "Amount".to_string(), value: format!("{amount} {base_asset}"), is_alt: false }
                ReviewRow { label: "Limit Price".to_string(), value: format!("{limit_price} {quote_asset}"), is_alt: true }
                ReviewRow { label: "Trading Fees".to_string(), value: fee_display, is_alt: false }

                if !flags.is_empty() {
                    ReviewRow { label: "Flags".to_string(), value: flags_display, is_alt: true }
                }
            }

            // Warning Text matched to 33rem width and 0.875rem size
            div { 
                style: "width: 100%; max-width: 33rem; padding: 1.5rem 0;",
                p { 
                    style: "font-size: 0.875rem; color: #777; text-align: center; font-family: monospace; line-height: 1.4; margin: 0;",
                    "Verify order. Ledger transactions cannot be undone."
                }
            }

            // Button - Exact copy of the XRP Send / Import button style
            button {
                style: "width: 8.75rem; height: 2.25rem; background-color: #333; color: white; border: none; border-radius: 1.375rem; font-size: 1rem; display: flex; cursor: pointer; justify-content: center; align-items: center; margin-top: 1rem;",
                onclick: move |_| {
                    xrp_ctx.trade.with_mut(|s| if let Some(ref mut t) = s.send_trade {
                        t.step = 3;
                        t.error = None;
                    });
                },
                "Continue"
            }
        }
    }
}

#[component]
fn ReviewRow(label: String, value: String, is_alt: bool) -> Element {
    let bg = if is_alt { "#222" } else { "#1a1a1a" };
    
    rsx! {
        div {
            // Increased padding to 1.25rem and font to 1rem to match XRP Send style
            style: "display: flex; flex-direction: row; justify-content: space-between; align-items: center; padding: 1.25rem 1rem; background-color: {bg}; border-bottom: 1px solid #2a2a2a;",
            span { style: "font-size: 1rem; color: #999; font-family: monospace;", "{label}" }
            span { 
                style: "font-size: 1rem; color: white; font-weight: bold; text-align: right; flex: 1; margin-left: 2rem; word-break: break-all; font-family: monospace;", 
                "{value}" 
            }
        }
    }
}