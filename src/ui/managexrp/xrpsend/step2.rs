//src/ui/managexrp/xrpsend/step2.rs 
//dependent upon src/utils/send_amount_layout.rs
//dependent upon src/utils/formatting.rs
//dependent upon src/utils/send_xrp_asset.rs 

use dioxus_native::prelude::*;
use crate::context::{XrpContext, GlobalContext, RlusdContext, SgdContext, EuroContext};
use crate::utils::{SendAsset, format_token_amount, format_usd};
use crate::utils::send_amount_layout::SendAmountForm;
use crate::utils::reserves::XrpBalanceInfo;

#[component]
pub fn view() -> Element {
    let xrp_ctx = use_context::<XrpContext>();
    let global = use_context::<GlobalContext>();
    let rlusd_ctx = use_context::<RlusdContext>();
    let euro_ctx = use_context::<EuroContext>();
    let sgd_ctx = use_context::<SgdContext>();

    
    // Pull the reserve calculation from the Parent context
    let reserve_memo = use_context::<Memo<XrpBalanceInfo>>();
    
    let mut sign_transaction = xrp_ctx.sign_transaction;

    let asset_str = sign_transaction.read().send_transaction.as_ref()
        .map(|s| s.asset.clone())
        .unwrap_or_else(|| "XRP".to_string());

    let asset = SendAsset::from_str(&asset_str);
    let asset_label = asset.label();
    let show_fiat = asset.has_usd_equivalent();

    // === THE CRITICAL LOGIC ===
    // If it's XRP, our "Total" for this screen is the Spendable amount from the parent.
    // If it's a Token, we can send the whole thing (asset.balance).
    let spendable_balance = if asset_str == "XRP" {
        reserve_memo.read().available
    } else {
        asset.balance(&xrp_ctx, &rlusd_ctx, &euro_ctx, &sgd_ctx)
    };

    let rates = global.rates.read();
    let exchange_rate = asset.fiat_rate_key()
        .and_then(|k| rates.get(k).copied())
        .unwrap_or(0.0) as f64;

    let mut amount_in = use_signal(|| {
        sign_transaction.read().send_transaction.as_ref()
            .and_then(|s| s.amount.clone())
            .unwrap_or_default()
    });

    let mut fiat_in = use_signal(|| {
        if !show_fiat { return String::new(); }
        let saved = amount_in();
        if let Ok(v) = saved.parse::<f64>() {
            format_usd(v * exchange_rate)
        } else {
            String::new()
        }
    });

    let mut clear_error = move || {
        sign_transaction.with_mut(|state| {
            if let Some(ref mut send) = state.send_transaction {
                send.error = None;
            }
        });
    };
    
    let on_amount_input = move |evt: FormEvent| {
        let val = evt.value().replace(['\n', '\r'], "");
        amount_in.set(val.clone());
        clear_error();

        if show_fiat {
            if let Ok(amount) = val.parse::<f64>() {
                fiat_in.set(format_usd(amount * exchange_rate));
            } else {
                fiat_in.set(String::new());
            }
        }
    };

    let on_fiat_input = move |evt: FormEvent| {
        let val = evt.value().replace(['\n', '\r'], "");
        fiat_in.set(val.clone());
        clear_error();

        if let Ok(fiat) = val.parse::<f64>() {
            if exchange_rate > 0.0 {
                amount_in.set(format_token_amount(fiat / exchange_rate, 6));
            }
        } else {
            amount_in.set(String::new());
        }
    };

    let on_next_click = move |_| {
        let amount_str = amount_in().trim().to_string();
        
        if let Ok(amount) = amount_str.parse::<f64>() {
            if amount <= 0.0 {
                sign_transaction.with_mut(|s| {
                    if let Some(ref mut tx) = s.send_transaction {
                        tx.error = Some("ERR: MIN_VALUE_REQUIRED".to_string());
                    }
                });
            } 
            // GATEKEEPER: Check against our calculated spendable_balance
            else if amount > spendable_balance {
                let err = format!("ERR: INSUFFICIENT_FUNDS // MAX: {:.6}", spendable_balance);
                sign_transaction.with_mut(|s| {
                    if let Some(ref mut tx) = s.send_transaction {
                        tx.error = Some(err);
                    }
                });
            } else {
                sign_transaction.with_mut(|s| {
                    if let Some(ref mut tx) = s.send_transaction {
                        tx.amount = Some(format_token_amount(amount, 6));
                        tx.step = 3;
                        tx.error = None;
                    }
                });
            }
        } else {
            sign_transaction.with_mut(|s| {
                if let Some(ref mut tx) = s.send_transaction {
                    tx.error = Some("ERR: INVALID_NUMBER_FORMAT".to_string());
                }
            });
        }
    };

    let current_error = sign_transaction.read().send_transaction.as_ref()
        .and_then(|s| s.error.clone());

    rsx! {
        SendAmountForm {
            asset_label: asset_label.to_string(),
            network_label: "XRP_MAINNET".to_string(),
            show_fiat,
            amount_in,
            fiat_in,
            // We pass the spendable balance here so the UI shows exactly what is sendable.
            formatted_balance: format_token_amount(spendable_balance, 6),
            exchange_rate,
            current_error,
            on_amount_input,
            on_fiat_input,
            on_next_click,
        }
    }
}