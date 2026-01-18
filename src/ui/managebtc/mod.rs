// src/ui/managebtc/mod.rs
use dioxus::prelude::*;
use crate::context::BtcContext;
use crate::channel::{BTCImport, BTCActiveView};
use bip39::{Mnemonic, Language};
use rand::{thread_rng, RngCore};
use zeroize::Zeroizing; 
use crate::utils::transactionssvg::TransactionsIcon;

pub mod btcimport;
pub mod btcbalance; 
pub mod receive; 
pub mod btcsend; 
pub mod btctransactions; 
pub mod btccreate;

#[component]
pub fn render_manage_btc() -> Element {
    let btc_ctx = use_context::<BtcContext>();
    
    let mut btc_modal = btc_ctx.btc_modal; 
    let mut btc_wallet_process = btc_ctx.btc_wallet_process; 

    let view_type = btc_modal.read().view_type; 
    let (_balance, address_opt, _) = btc_ctx.bitcoin_wallet.read().clone();
    let has_wallet = address_opt.is_some();
    
    // --- THE GATE ---
    match view_type {
        BTCActiveView::Import       => return rsx! { btcimport::view {} },
        BTCActiveView::Create       => return rsx! { btccreate::view {} },
        BTCActiveView::Send         => return rsx! { btcsend::view {} },
        BTCActiveView::Transactions => return rsx! { btctransactions::view {} },
        BTCActiveView::Receive      => return rsx! { receive::view {} },
        BTCActiveView::BTC          => {} 
    }

    // --- EVENT HANDLERS ---
    let on_import_click = move |_| {
        btc_wallet_process.with_mut(|state| {
            state.import_wallet = Some(BTCImport { step: 1, seed: None, error: None });
        });
        btc_modal.with_mut(|s| s.view_type = BTCActiveView::Import);
    };

    let on_create_click = move |_| {
        let mut entropy = [0u8; 32];
        thread_rng().fill_bytes(&mut entropy);
        let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy).unwrap();
        let seed = Zeroizing::new(mnemonic.to_string());

        btc_wallet_process.with_mut(|state| {
            state.create_wallet = Some(BTCImport { step: 1, seed: Some(seed), error: None });
        });
        btc_modal.with_mut(|s| s.view_type = BTCActiveView::Create);
    };

    // --- RENDER ---
    rsx! {
        style { {r#"
            .wallet-main-container { 
                display: flex; 
                flex-direction: row; 
                align-items: center; 
                justify-content: center; 
                width: 100%; 
                position: relative; 
                height: 100%;
            }
            .side-dock-container { 
                position: absolute; 
                left: 2rem; 
                display: flex; 
                flex-direction: column; 
                gap: 0.5rem; 
                padding: 1rem 0.5rem; 
                background-color: rgba(30, 30, 30, 0.8); 
                border-radius: 2rem; 
                border: 1px solid rgba(255, 255, 255, 0.1); 
                align-items: center; 
                visibility: hidden; 
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
                div { class: "side-dock-container" } 
            }

            if !has_wallet {
                div { class: "manage-grid",
                    button { class: "manage-btn", onclick: on_create_click, "Create Wallet" }
                    button { class: "manage-btn", onclick: on_import_click, "Import Wallet" }
                }
            } else {
                btcbalance::view {}
            }

            if has_wallet {
                div { class: "right-dock-container",
                    render_btc_transactions_toggle { 
                        is_active: matches!(view_type, BTCActiveView::Transactions), 
                        onclick: move |_| { 
                            btc_modal.with_mut(|s| s.view_type = BTCActiveView::Transactions);
                        }
                    } 
                }
            }
        }
    }
}

// --- HELPER COMPONENTS (Moved outside main function to avoid delimiter errors) ---

fn dock_base_style(bg: &str, color: &str) -> String {
    format!(
        "padding: 0.6rem; border-radius: 1.2rem; border: none; cursor: pointer; \
         display: flex; align-items: center; justify-content: center; \
         background-color: {}; color: {}; transition: all 0.2s ease;",
        bg, color
    )
}

#[component]
fn render_btc_transactions_toggle(is_active: bool, onclick: EventHandler<MouseEvent>) -> Element {
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
            style: "{dock_base_style(bg, text)} padding: 0.6rem 1rem; font-weight: {font_weight};",
            onclick: onclick,
            "{label}"
        }
    }
}