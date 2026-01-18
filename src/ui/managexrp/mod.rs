// src/ui/managexrp/mod.rs
use dioxus::prelude::*;
use crate::context::XrpContext;
use crate::channel::{XRPImport, ActiveView, Trade};
use bip39::{Mnemonic, Language};
use rand::{thread_rng, RngCore};
use zeroize::Zeroizing;
use crate::utils::tradesvg::TradeIcon;
use crate::utils::transactionssvg::TransactionsIcon;

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

    let on_trade_init = move |_| {
        let current_view = xrp_modal.read().view_type.clone();
        xrp_modal.with_mut(|state| {
            state.last_view = Some(current_view);
            state.view_type = ActiveView::Trade;
        });
        trade_tx.with_mut(|state| {
            state.send_trade = Some(Trade {
                step: 1, base_asset: None, quote_asset: None, amount: None,
                limit_price: None, fee_percentage: 0.0, flags: None, error: None,
                asset: "XRP".to_string()
            });
        });
    };

    let on_tx_click = move |_: MouseEvent| {
        let current_view = xrp_modal.read().view_type.clone();
        xrp_modal.with_mut(|state| {
            state.last_view = Some(current_view);
            state.view_type = ActiveView::Transactions;
        });
    };

    let on_create_click = move |_| {
        let mut entropy = [0u8; 32];
        thread_rng().fill_bytes(&mut entropy);
        let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy).unwrap();
        let seed = Zeroizing::new(mnemonic.to_string());
        
        wallet_process.with_mut(|state| {
            state.create_wallet = Some(XRPImport { step: 1, seed: Some(seed), error: None });
        });
        xrp_modal.with_mut(|s| s.view_type = ActiveView::Create);
    };

    let on_import_click = move |_| {
        wallet_process.with_mut(|state| {
            state.import_wallet = Some(XRPImport { step: 1, seed: None, error: None });
        });
        xrp_modal.with_mut(|s| s.view_type = ActiveView::Import);
    };

rsx! {
        style { {r#"
            .wallet-main-container { 
                display: flex; 
                flex-direction: row; 
                align-items: center; 
                justify-content: center; 
                width: 100%; 
                position: relative; 
            }
            .side-dock-container { 
                position: absolute; 
                left: 2rem; 
                display: flex; 
                flex-direction: column; 
                gap: 0.5rem; 
                padding: 1.5rem 0.5rem; 
                background-color: rgba(30, 30, 30, 0.8); 
                border-radius: 2rem; 
                border: 1px solid rgba(255, 255, 255, 0.1); 
                align-items: center; 
            }
            .right-dock-container { 
                position: absolute; 
                right: 2rem; 
                display: flex; 
                flex-direction: column; 
                gap: 0.5rem; 
                padding: 1rem 0.5rem; 
                background-color: rgba(30, 30, 30, 0.8); 
                border-radius: 2rem; 
                border: 1px solid rgba(255, 255, 255, 0.1); 
                align-items: center; 
            }
            .manage-grid { display: grid; grid-template-columns: 1fr; gap: 1rem; max-width: 22rem; }
            .manage-btn { padding: 1rem 3rem; font-size: 1rem; border-radius: 1rem; border: 1px solid #444; background: var(--btn); color: #fff; cursor: pointer; }
        "#} }

        div { class: "wallet-main-container",
            if has_wallet {
                div { class: "side-dock-container",
                    DockButton { 
                        label: "XRP".to_string(), 
                        is_active: matches!(view_type, ActiveView::XRP), 
                        onclick: move |_| xrp_modal.with_mut(|s| s.view_type = ActiveView::XRP) 
                    }
                    DockButton { 
                        label: "USD".to_string(), 
                        is_active: matches!(view_type, ActiveView::RLUSD), 
                        onclick: move |_| xrp_modal.with_mut(|s| s.view_type = ActiveView::RLUSD) 
                    }
                    DockButton { 
                        label: "EUR".to_string(), 
                        is_active: matches!(view_type, ActiveView::EURO), 
                        onclick: move |_| xrp_modal.with_mut(|s| s.view_type = ActiveView::EURO) 
                    }
                }
            }

            if !has_wallet {
                div { class: "manage-grid",
                    button { class: "manage-btn", onclick: on_create_click, "Create Wallet" }
                    button { class: "manage-btn", onclick: on_import_click, "Import Wallet" }
                }
            } else {
                match view_type {
                    ActiveView::RLUSD => rsx! { managerlusd::render_rlusd_balance {} },
                    ActiveView::EURO => rsx! { manageeuro::render_manage_euro {} },
                    _ => rsx! { xrpbalance::render_xrp_balance {} },
                }
            }

            if has_wallet {
                div { class: "right-dock-container",
                    render_trade_toggle { 
                        is_active: matches!(view_type, ActiveView::Trade), 
                        onclick: on_trade_init 
                    }
                    render_transactions_toggle { 
                        is_active: matches!(view_type, ActiveView::Transactions),
                        onclick: on_tx_click 
                    }
                }
            }
        }
    }
}

fn dock_base_style(bg: &str, color: &str) -> String {
    format!(
        "padding: 0.6rem; border-radius: 1.2rem; border: none; cursor: pointer; \
         display: flex; align-items: center; justify-content: center; \
         background-color: {}; color: {}; transition: all 0.2s ease;",
        bg, color
    )
}

#[component]
fn render_trade_toggle(is_active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let (bg, icon) = if is_active { ("white", "black") } else { ("transparent", "#aaa") };
    rsx! {
        button {
            style: dock_base_style(bg, icon),
            onclick: move |e| onclick.call(e),
            TradeIcon {}
        }
    }
}

#[component]
fn render_transactions_toggle(is_active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let (bg, icon) = if is_active { ("white", "black") } else { ("transparent", "#aaa") };
    rsx! {
        button {
            style: dock_base_style(bg, icon),
            onclick: move |e| onclick.call(e),
            TransactionsIcon {}
        }
    }
}

#[component]
fn DockButton(label: String, is_active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let (bg, text) = if is_active { ("white", "black") } else { ("transparent", "#aaa") };
    let font_weight = if is_active { "bold" } else { "normal" };
    rsx! {
        button {
            // Horizontal padding 1rem for the 3-letter words
            style: "{dock_base_style(bg, text)} padding: 0.6rem 1rem; font-weight: {font_weight};",
            onclick: onclick,
            "{label}"
        }
    }
}
