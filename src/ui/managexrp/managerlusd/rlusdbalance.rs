use dioxus::prelude::*;
use crate::context::{GlobalContext, XrpContext, RlusdContext};
use crate::utils::add_commas;
use crate::utils::rlusdsvg::RlusdIcon;
use crate::utils::upsvg::UpIcon;
use crate::utils::downsvg::DownIcon;

pub fn render_rlusd_balance() -> Element {
    let global = use_context::<GlobalContext>();
    let xrp_ctx = use_context::<XrpContext>();
    let rlusd_ctx = use_context::<RlusdContext>();
    
    let mut sign_tx = xrp_ctx.sign_transaction;
    let mut xrp_modal = xrp_ctx.xrp_modal;

    // Extracting (amount, exists, limit) from the watch receiver
    let (rlusd_amount, _, rlusd_limit) = rlusd_ctx.rlusd.read().clone();
    let (_, _, hide_balance) = global.theme_user.read().clone();

    let rlusd_rate = 1.0000;
    let total_usd = rlusd_amount * rlusd_rate;

    let (int_part, frac_part) = if hide_balance {
        ("****".to_string(), "".to_string())
    } else {
        let formatted_int = add_commas(total_usd.floor() as i64);
        let formatted_frac = format!(".{:02}", (total_usd.fract() * 100.0).round() as i64);
        (formatted_int, formatted_frac)
    };

    let formatted_raw_amount = format!("{:.6}", rlusd_amount);

    // Format the limit string for display
    let limit_display = match rlusd_limit {
        Some(limit) => add_commas(limit as i64),
        None => "None".to_string(),
    };

    let on_receive_click = move |_| {
        xrp_modal.with_mut(|state| {
            state.last_view = Some(crate::channel::ActiveView::RLUSD); 
            state.view_type = crate::channel::ActiveView::Receive;
        });
    };
    
    let on_send_click = move |_| {
        xrp_modal.with_mut(|state| {
            state.last_view = Some(crate::channel::ActiveView::RLUSD);
            state.view_type = crate::channel::ActiveView::Send; 

        });
        sign_tx.with_mut(|state| {
            state.send_transaction = Some(crate::channel::SignTransaction {
                step: 1,
                error: None,
                recipient: None, 
                amount: None,
                asset: "RLUSD".to_string()
            });
        });
    };

  rsx! {
    div {
        // Change 1: Ensure line-height is EXACTLY 1 to match XRP
        // Change 2: Use flex: 1 0 auto to prevent any vertical squishing
        style: "display: flex; flex-direction: column; align-items: center; flex: 1 0 auto; font-family: monospace; padding-top: 2rem; box-sizing: border-box;",

        div {
            style: "width: 24rem; flex-shrink: 0; display: flex; flex-direction: column; align-items: center; box-sizing: border-box;",

            // BLOCK 1: MAIN BALANCE
            div {
                style: "display: flex; justify-content: center; margin-bottom: 2.5rem; box-sizing: border-box;",
                h1 {
                    // CRITICAL: Changed line-height from 1.1 to 1 to match XRP
                    style: "display: flex; align-items: baseline; font-size: 6rem; line-height: 1; margin: 0; box-sizing: border-box;",
                    if !hide_balance {
                        span { style: "font-weight: bold; margin-right: 0.5rem;", "$" }
                        span { style: "font-weight: bold;", "{int_part}" }
                        span { style: "opacity: 0.8;", "{frac_part}" }
                    } else {
                        span { style: "font-weight: normal; letter-spacing: 0.5rem;", "****" }
                    }
                }
            }

            // BLOCK 2: DETAILS
            div {
                style: "display: flex; flex-direction: column; align-items: center; margin-bottom: 2.5rem; box-sizing: border-box;",
                h5 {
                    style: "display: flex; flex-direction: column; align-items: center; opacity: 0.7; font-weight: normal; margin: 0; box-sizing: border-box;",
                    span { 
                        style: "display: flex; align-items: center; gap: 0.5rem; font-size: 1.25rem; margin-bottom: 0.25rem; box-sizing: border-box;", 
                        if hide_balance { "****" } else { "{formatted_raw_amount}" }
                        RlusdIcon {}
                    }
                    span { style: "font-size: 0.9rem; opacity: 0.5; box-sizing: border-box;", "Rate: ${rlusd_rate:.4}" }
                }
            }

            // BLOCK 3: ACTION BUTTONS
            div {
                style: "display: flex; flex-direction: row; justify-content: center; gap: 1rem; margin-bottom: 2.5rem; box-sizing: border-box;",
                button {
                    style: "display: flex; justify-content: center; align-items: center; width: 6rem; height: 3.5rem; border: 1px solid #444; border-radius: 2rem; background: none; color: #888; cursor: pointer; box-sizing: border-box;",
                    onclick: on_send_click,
                    UpIcon {}
                }
                button {
                    style: "display: flex; justify-content: center; align-items: center; width: 6rem; height: 3.5rem; border: 1px solid #444; border-radius: 2rem; background: none; color: #888; cursor: pointer; box-sizing: border-box;",
                    onclick: on_receive_click,
                    DownIcon {}
                }
            }

            // BLOCK 4: TRUSTLINE FOOTER
            div {
                style: "display: flex; flex-direction: row; justify-content: center; gap: 2.5rem; font-size: 0.9rem; box-sizing: border-box; padding: 1rem 0;",
                span { 
                    style: "opacity: 0.6; box-sizing: border-box;", 
                    "Trustline Limit" 
                }
                span { 
                    style: "opacity: 0.6; box-sizing: border-box;",
                    "{limit_display}"
                }
            }
        }
    }
}
}