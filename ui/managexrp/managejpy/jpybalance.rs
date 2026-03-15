//src/ui/managexrp/managerlusd/rlusdbalance.rs 
//dependent upon utils/layout.rs 

use dioxus_native::prelude::*;
use crate::context::{GlobalContext, XrpContext, JpyContext};
use crate::utils::add_commas;
use crate::utils::styles::terminal_action;
use crate::utils::token_layout::render_token_layout; // MATCHED NAME

pub fn render_jpy_balance() -> Element {
    let global = use_context::<GlobalContext>();
    let xrp_ctx = use_context::<XrpContext>();
    let jpy_ctx = use_context::<JpyContext>();
    
    let mut sign_tx = xrp_ctx.sign_transaction;
    let mut xrp_modal = xrp_ctx.xrp_modal;

    let (jpy_amount, _, jpy_limit) = jpy_ctx.jpy.read().clone();
    let (_, hide_balance) = global.theme_user.read().clone();

    let jpy_rate = 1.0000;
    let total_jpy = jpy_amount * jpy_rate;

    let (int_part, frac_part) = if hide_balance {
    ("****".to_string(), "".to_string())
} else {
    // JPY: Round to nearest whole number and use empty string for fractions
    (add_commas(total_jpy.round() as i64), "".to_string())
};
    let formatted_raw_amount = if hide_balance { "****".to_string() } else { format!("{:.6}", jpy_amount) };

    let limit_display = match jpy_limit {
        Some(limit) => add_commas(limit as i64),
        None => "UNLIMITED".to_string(),
    };

    // --- ACTIONS ---
    let send_btn = terminal_action("SEND_FUNDS", true, move |_| {
        xrp_modal.with_mut(|state| {
            state.last_view = Some(crate::channel::ActiveView::JPY);
            state.view_type = crate::channel::ActiveView::Send; 
        });
        sign_tx.with_mut(|state| {
            state.send_transaction = Some(crate::channel::SignTransaction {
                step: 1, error: None, recipient: None, amount: None, asset: "JPY".to_string()
            });
        });
    });

    let receive_btn = terminal_action("RECEIVE_ASSETS", true, move |_| {
        xrp_modal.with_mut(|state| {
            state.last_view = Some(crate::channel::ActiveView::JPY); 
            state.view_type = crate::channel::ActiveView::Receive;
        });
    });

    rsx! {
        {render_token_layout(
            "JPY",
            "¥",
            &int_part,
            &frac_part, // This will now be empty, so the span won't show anything
            &formatted_raw_amount,
            &limit_display,
            jpy_rate,
            send_btn,
            receive_btn
        )}
    }
}