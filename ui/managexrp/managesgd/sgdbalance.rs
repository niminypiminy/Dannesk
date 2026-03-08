//src/ui/managexrp/managerlusd/rlusdbalance.rs 
//dependent upon utils/layout.rs 

use dioxus_native::prelude::*;
use crate::context::{GlobalContext, XrpContext, SgdContext};
use crate::utils::add_commas;
use crate::utils::styles::terminal_action;
use crate::utils::token_layout::render_token_layout; // MATCHED NAME

pub fn render_sgd_balance() -> Element {
    let global = use_context::<GlobalContext>();
    let xrp_ctx = use_context::<XrpContext>();
    let sgd_ctx = use_context::<SgdContext>();
    
    let mut sign_tx = xrp_ctx.sign_transaction;
    let mut xrp_modal = xrp_ctx.xrp_modal;

    let (sgd_amount, _, sgd_limit) = sgd_ctx.sgd.read().clone();
    let (_, hide_balance) = global.theme_user.read().clone();

    let sgd_rate = 1.0000;
    let total_sgd = sgd_amount * sgd_rate;

    let (int_part, frac_part) = if hide_balance {
        ("****".to_string(), "".to_string())
    } else {
        (add_commas(total_sgd.floor() as i64), format!(".{:02}", (total_sgd.fract() * 100.0).round() as i64))
    };

    let formatted_raw_amount = if hide_balance { "****".to_string() } else { format!("{:.6}", sgd_amount) };

    let limit_display = match sgd_limit {
        Some(limit) => add_commas(limit as i64),
        None => "UNLIMITED".to_string(),
    };

    // --- ACTIONS ---
    let send_btn = terminal_action("SEND_FUNDS", true, move |_| {
        xrp_modal.with_mut(|state| {
            state.last_view = Some(crate::channel::ActiveView::SGD);
            state.view_type = crate::channel::ActiveView::Send; 
        });
        sign_tx.with_mut(|state| {
            state.send_transaction = Some(crate::channel::SignTransaction {
                step: 1, error: None, recipient: None, amount: None, asset: "XSGD".to_string()
            });
        });
    });

    let receive_btn = terminal_action("RECEIVE_ASSETS", true, move |_| {
        xrp_modal.with_mut(|state| {
            state.last_view = Some(crate::channel::ActiveView::SGD); 
            state.view_type = crate::channel::ActiveView::Receive;
        });
    });

    rsx! {
        {render_token_layout(
            "XSGD",
            "S$",
            &int_part,
            &frac_part,
            &formatted_raw_amount,
            &limit_display,
            sgd_rate,
            send_btn,
            receive_btn
        )}
    }
}