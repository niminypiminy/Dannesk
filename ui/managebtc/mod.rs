// src/ui/managebtc/mod.rs
use dioxus_native::prelude::*;
use crate::context::BtcContext;
use crate::channel::{BTCImport, BTCActiveView};
use bip39::{Mnemonic, Language};
use rand::{rng, Rng};
use zeroize::Zeroizing; 
use crate::utils::styles::terminal_action; 

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

    // --- TERMINAL ACTIONS ---
    let create_btn = terminal_action("CREATE_BTC_WALLET", true, move |_| {
        let mut entropy = [0u8; 32];
        rng().fill_bytes(&mut entropy);
        let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy).unwrap();
        let seed = Zeroizing::new(mnemonic.to_string());

        btc_wallet_process.with_mut(|state| {
            state.create_wallet = Some(BTCImport { step: 1, seed: Some(seed), error: None });
        });
        btc_modal.with_mut(|s| s.view_type = BTCActiveView::Create);
    });

    let import_btn = terminal_action("IMPORT_BTC_WALLET", true, move |_| {
        btc_wallet_process.with_mut(|state| {
            state.import_wallet = Some(BTCImport { step: 1, seed: None, error: None });
        });
        btc_modal.with_mut(|s| s.view_type = BTCActiveView::Import);
    });

    let history_btn = terminal_action("HISTORY", matches!(view_type, BTCActiveView::Transactions), move |_| {
        btc_modal.with_mut(|s| s.view_type = BTCActiveView::Transactions);
    });

    // --- RENDER ---
   rsx! {
        style { {r#"
            .terminal-viewport { 
                display: flex; 
                flex-direction: row; 
                width: 100%; 
                flex: 1;
                justify-content: center; 
                padding: 0 2rem; 
                box-sizing: border-box;
                position: relative;
            }
            .setup-container {
                width: 100%;
                max-width: 600px; 
                display: flex;
                flex-direction: column;
                align-items: center;
            }
            .setup-header {
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
            .term-main { 
                flex: 1; 
                display: flex; 
                flex-direction: column; 
                align-items: center; 
                justify-content: center; 
            }
            .term-sidebar-right { 
                position: absolute;
                right: 2rem;
                display: flex; 
                flex-direction: column; 
                gap: 1rem; 
                justify-content: center; 
                height: 100%;
                align-items: flex-end;
            }
        "#} }

        div { class: "terminal-viewport",
            // No left sidebar for BTC
            
            div { class: "term-main",
                if !has_wallet {
                    // Applied matching setup container and label
                    div { class: "setup-container",
                        div { class: "setup-header",
                            div { class: "setup-label", "BTC_MAINNET" }
                        }
                        div { 
                            style: "display: flex; flex-direction: column; gap: 1rem; width: 100%; align-items: center;",
                            {create_btn}
                            {import_btn}
                        }
                    }
                } else {
                    btcbalance::view {}
                }
            }

            if has_wallet {
                div { class: "term-sidebar-right",
                    {history_btn}
                }
            }
        }
    }
}