// src/ui/managebtc/btccreate/step2.rs
use dioxus::prelude::*;
use crate::context::{GlobalContext, BtcContext};
use crate::ui::managebtc::btccreate::btccreatelogic::BTCCreateLogic;
use zeroize::Zeroizing;

#[component]
pub fn view() -> Element {
    let global = use_context::<GlobalContext>();
    let mut btc_ctx = use_context::<BtcContext>();
    
    let mut bip39_buffer = use_signal(|| String::new());
    let mut encryption_buffer = use_signal(|| String::new());

    let on_create_click = move |_| {
        // 1. Wrap inputs IMMEDIATELY
        let b_pass = Zeroizing::new(bip39_buffer().trim().to_string());
        let e_pass = Zeroizing::new(encryption_buffer().trim().to_string());
        
        // 2. Extract the Generated Seed (from Step 1) safely
        let seed_opt = btc_ctx.btc_wallet_process.read()
            .create_wallet.as_ref()
            .and_then(|w| w.seed.clone());

        // 3. SECURE CHECK & SPAWN
        if let Some(seed_guard) = seed_opt {
            // Validation
            if e_pass.len() < 10 {
                 btc_ctx.btc_wallet_process.with_mut(|state| {
                    if let Some(ref mut create) = state.create_wallet {
                        create.error = Some("Encryption passphrase must be at least 10 characters.".to_string());
                    }
                });
                return;
            }

            // Move GUARDS into logic
            tokio::spawn(BTCCreateLogic::process(
                seed_guard, // Zeroizing<String> from Step 1
                b_pass,     // Zeroizing<String>
                e_pass,     // Zeroizing<String>
                global.ws_tx.clone()
            ));
        } else {
            // Should not happen if Step 1 worked, but good to handle
            return; 
        }

        // 4. WIPE UI
        bip39_buffer.set(String::new());
        encryption_buffer.set(String::new());
        
        // Clear the seed from state immediately so it doesn't linger in memory
        btc_ctx.btc_wallet_process.with_mut(|state| {
            if let Some(ref mut create) = state.create_wallet {
                create.seed = None; 
                create.error = None;
            }
        });
    };

    // Pull error state for the UI
    let create_state = btc_ctx.btc_wallet_process.read();
    let current_error = create_state.create_wallet.as_ref().and_then(|i| i.error.clone());

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; width: 100%;",

            div { style: "font-size: 1.5rem; margin: 0; margin-bottom: 0.5rem;", "Enter BIP39 Passphrase (Optional)" }
            div { style: "font-size: 1rem; color: #888; margin-bottom: 1.5rem;", "If you want a 25th word, enter it here." }

            input {
                style: "width: 100%; max-width: 25rem; height: 2rem; padding: 0.3125rem; background-color: transparent; border: 1px solid #444; border-radius: 0.25rem; font-size: 1.25rem; margin-bottom: 2rem;",
                value: "{bip39_buffer()}",
                oninput: move |e| {
                    bip39_buffer.set(e.value());
                }
            }

            div { style: "font-size: 1.5rem; margin: 0; margin-bottom: 0.5rem;", "Encryption Passphrase" }
            div { style: "font-size: 1rem; color: #888; margin-bottom: 1.5rem;", "Password to encrypt your wallet locally." }

            input {
                style: "width: 100%; max-width: 25rem; height: 2rem; padding: 0.3125rem; background-color: transparent; border: 1px solid #444; border-radius: 0.25rem; font-size: 1.25rem; margin-bottom: 1rem;",
                value: "{encryption_buffer()}",
                type: "password", // Helps visually mask it
                oninput: move |e| {
                    encryption_buffer.set(e.value());
                }
            }

            if let Some(err) = current_error {
                div { style: "color: #ff4d4d; margin-bottom: 1rem; font-size: 0.875rem; font-weight: bold;", "{err}" }
            }

           button {
                style: "width: 8.75rem; height: 2.25rem; background-color: #333; color: white; border: none; 
        border-radius: 1.375rem; font-size: 1rem; display: flex; cursor: pointer; justify-content: center; align-items: center; margin-top: 1rem;",
                onclick: on_create_click,
                "Create Wallet"
            }
        }
    }
}