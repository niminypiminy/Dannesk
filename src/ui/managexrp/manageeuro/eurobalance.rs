//src/ui/managexrp/manageeuro/eurobalance.rs
//dependent upon utils/layout.rs 

use dioxus_native::prelude::*;
use crate::context::{GlobalContext, XrpContext, EuroContext};
use crate::utils::add_commas;
use crate::utils::styles::terminal_action;
use crate::utils::token_layout::render_token_layout; // Centralized layout

pub fn render_euro_balance() -> Element {
    let global = use_context::<GlobalContext>();
    let xrp_ctx = use_context::<XrpContext>();
    let euro_ctx = use_context::<EuroContext>();
    
    let mut sign_tx = xrp_ctx.sign_transaction;
    let mut xrp_modal = xrp_ctx.xrp_modal;

    let (euro_amount, _, euro_limit) = euro_ctx.euro.read().clone();
    let (_, hide_balance) = global.theme_user.read().clone();

    let euro_rate = 1.0000;

    let (int_part, frac_part) = if hide_balance {
        ("****".to_string(), "".to_string())
    } else {
        (
            add_commas(euro_amount.floor() as i64), 
            format!(".{:02}", (euro_amount.fract() * 100.0).round() as i64)
        )
    };

    let formatted_raw_amount = if hide_balance { "****".to_string() } else { format!("{:.6}", euro_amount) };

    let limit_display = match euro_limit {
        Some(limit) => add_commas(limit as i64),
        None => "UNLIMITED".to_string(),
    };

    // --- ACTIONS ---
    let send_btn = terminal_action("SEND_FUNDS", true, move |_| {
        xrp_modal.with_mut(|state| {
            state.last_view = Some(crate::channel::ActiveView::EURO);
            state.view_type = crate::channel::ActiveView::Send; 
        });
        sign_tx.with_mut(|state| {
            state.send_transaction = Some(crate::channel::SignTransaction {
                step: 1, error: None, recipient: None, amount: None, asset: "EURO".to_string()
            });
        });
    });

    let receive_btn = terminal_action("RECEIVE_ASSETS", true, move |_| {
        xrp_modal.with_mut(|state| {
            state.last_view = Some(crate::channel::ActiveView::EURO);
            state.view_type = crate::channel::ActiveView::Receive;
        });
    });

    rsx! {
        {render_token_layout(
            "EUR",
            "€",
            &int_part,
            &frac_part,
            &formatted_raw_amount,
            &limit_display,
            euro_rate,
            send_btn,
            receive_btn
        )}
    }
}