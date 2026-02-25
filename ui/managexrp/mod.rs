// src/ui/managexrp/mod.rs
use dioxus_native::prelude::*;
use crate::context::XrpContext;
use crate::channel::{XRPImport, ActiveView, Trade};
use bip39::{Mnemonic, Language};
use rand::{rng, Rng};
use zeroize::Zeroizing;
use crate::utils::styles::{terminal_action, nav_action}; 

pub mod xrpimport;
pub mod xrpcreate; 
pub mod xrpbalance;  
pub mod xrpsend;
pub mod managerlusd;
pub mod manageeuro;
pub mod receive;
pub mod transactions;
pub mod trade;

pub fn render_manage_xrp() -> Element {
    let xrp = use_context::<XrpContext>();
    let mut xrp_modal = xrp.xrp_modal; 
    let mut wallet_process = xrp.wallet_process; 
    let mut trade_tx = xrp.trade; 

    let view_type = xrp_modal.read().view_type;
    let (_amount, address_opt, _) = xrp.wallet_balance.read().clone();
    let has_wallet = address_opt.is_some();

    match view_type {
        ActiveView::Import       => return rsx! { xrpimport::view {} },
        ActiveView::Create       => return rsx! { xrpcreate::view {} },
        ActiveView::Send         => return rsx! { xrpsend::view {} },
        ActiveView::Trade        => return rsx! { trade::view {} },
        ActiveView::Transactions => return rsx! { transactions::view {} },
        ActiveView::Receive      => return rsx! { receive::view {} },
        _ => {} 
    }

    // Asset Navigation uses nav_action for the green active state
    let nav_xrp = nav_action("XRP", matches!(view_type, ActiveView::XRP), move |_| xrp_modal.with_mut(|s| s.view_type = ActiveView::XRP));
    let nav_usd = nav_action("USD", matches!(view_type, ActiveView::RLUSD), move |_| xrp_modal.with_mut(|s| s.view_type = ActiveView::RLUSD));
    let nav_eur = nav_action("EUR", matches!(view_type, ActiveView::EURO), move |_| xrp_modal.with_mut(|s| s.view_type = ActiveView::EURO));

    let create_btn = terminal_action("CREATE_XRP_WALLET", true, move |_| {
        let mut entropy = [0u8; 32];
        rng().fill_bytes(&mut entropy);
        let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy).unwrap();
        let seed = Zeroizing::new(mnemonic.to_string());
        wallet_process.with_mut(|state| state.create_wallet = Some(XRPImport { step: 1, seed: Some(seed), error: None }));
        xrp_modal.with_mut(|s| s.view_type = ActiveView::Create);
    });

    let import_btn = terminal_action("IMPORT_XRP_WALLET", true, move |_| {
        wallet_process.with_mut(|state| state.import_wallet = Some(XRPImport { step: 1, seed: None, error: None }));
        xrp_modal.with_mut(|s| s.view_type = ActiveView::Import);
    });

    let trade_btn = terminal_action("TRADE", matches!(view_type, ActiveView::Trade), move |_| {
        xrp_modal.with_mut(|state| { state.last_view = Some(ActiveView::XRP); state.view_type = ActiveView::Trade; });
        trade_tx.with_mut(|state| {
            state.send_trade = Some(Trade {
                step: 1, base_asset: None, quote_asset: None, amount: None,
                limit_price: None, fee_percentage: 0.0, flags: None, error: None,
                asset: "XRP".to_string()
            });
        });
    });

    let history_btn = terminal_action("HISTORY", matches!(view_type, ActiveView::Transactions), move |_| {
        xrp_modal.with_mut(|state| { state.last_view = Some(ActiveView::XRP); state.view_type = ActiveView::Transactions; });
    });

    rsx! {
        style { {r#"
            .terminal-viewport { 
                display: flex; 
                flex-direction: row; 
                width: 100%; 
                flex: 1;
                justify-content: space-between; 
                padding: 0 2rem; 
                box-sizing: border-box;
            }
            .setup-container {
                width: 100%;
                max-width: 600px; 
                display: flex;
                flex-direction: column;
                align-items: center;
            }
            .setup-header {
                width: 100%;
                border-bottom: 1px solid var(--border);
                padding-bottom: 0.5rem;
                margin-bottom: 2rem;
                display: flex;
                justify-content: center; 
                align-items: flex-end;
            }
            .setup-label { 
                font-size: 0.7rem; 
                color: var(--text-secondary); 
                letter-spacing: 0.25rem; 
                font-weight: 600;
                padding-left: 0.25rem; 
            }
            .term-sidebar { 
                display: flex; 
                flex-direction: column; 
                gap: 1rem; 
                justify-content: center; 
                min-width: 140px; 
            }
            .term-main { 
                flex: 1; 
                display: flex; 
                flex-direction: column; 
                align-items: center; 
                justify-content: center; 
            }
        "#} }

        div { class: "terminal-viewport",
            div { class: "term-sidebar",
                if has_wallet {
                    {nav_xrp}
                    {nav_usd}
                    {nav_eur}
                }
            }

            div { class: "term-main",
                if !has_wallet {
                    div { class: "setup-container",
                        div { class: "setup-header",
                            div { class: "setup-label", "XRP_NETWORK_INITIALIZATION" }
                        }
                        div { 
                            style: "display: flex; flex-direction: column; gap: 1rem; width: 100%; align-items: center;",
                            {create_btn}
                            {import_btn}
                        }
                    }
                } else {
                    match view_type {
                        ActiveView::RLUSD => rsx! { managerlusd::render_rlusd_balance {} },
                        ActiveView::EURO => rsx! { manageeuro::render_manage_euro {} },
                        _ => rsx! { xrpbalance::render_xrp_balance {} },
                    }
                }
            }

            div { class: "term-sidebar",
                style: "align-items: flex-end;", 
                if has_wallet {
                    {trade_btn}
                    {history_btn}
                }
            }
        }
    }
}