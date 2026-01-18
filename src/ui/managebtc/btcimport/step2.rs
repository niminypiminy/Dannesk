use dioxus::prelude::*;
use crate::context::{GlobalContext, BtcContext};
use crate::ui::managebtc::btcimport::btcimportlogic::BTCImportLogic;
use zeroize::{Zeroizing};

#[component]
pub fn view() -> Element {
    let global = use_context::<GlobalContext>();
    let mut btc_ctx = use_context::<BtcContext>();
    
    let mut bip39_buffer = use_signal(|| String::new());
    let mut encryption_buffer = use_signal(|| String::new());

   let on_import_click = move |_| {
        // 1. Wrap inputs IMMEDIATELY
        let b_pass = Zeroizing::new(bip39_buffer().trim().to_string());
        let e_pass = Zeroizing::new(encryption_buffer().trim().to_string());
        
        // 2. Extract existing Zeroizing guard safely
        let seed_opt = btc_ctx.btc_wallet_process.read()
            .import_wallet.as_ref()
            .and_then(|w| w.seed.clone());

        // 3. SECURE CHECK & SPAWN (No unwrap)
        if let Some(seed_guard) = seed_opt {
            // Peek strictly for validation
            if seed_guard.is_empty() || e_pass.len() < 10 {
                 btc_ctx.btc_wallet_process.with_mut(|state| {
                    if let Some(ref mut import) = state.import_wallet {
                        import.error = Some("Check mnemonic and encryption (min 10).".to_string());
                    }
                });
                return;
            }

            // Move GUARDS into logic
            tokio::spawn(BTCImportLogic::process(
                seed_guard, // Zeroizing<String>
                b_pass,     // Zeroizing<String>
                e_pass,     // Zeroizing<String>
                global.ws_tx.clone()
            ));
        } else {
            return; // Handle missing seed edge case
        }

        // 4. WIPE UI
        bip39_buffer.set(String::new());
        encryption_buffer.set(String::new());
        
        btc_ctx.btc_wallet_process.with_mut(|state| {
            if let Some(ref mut import) = state.import_wallet {
                import.seed = None; 
            }
        });
    };

    // Pull error state for the UI
    let import_state = btc_ctx.btc_wallet_process.read();
    let current_error = import_state.import_wallet.as_ref().and_then(|i| i.error.clone());
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; width: 100%;",

            div { style: "font-size: 1.5rem; margin: 0; margin-bottom: 0.5rem;", "Enter BIP39 Passphrase (Optional)" }
            div { style: "font-size: 1rem; color: #888; margin-bottom: 1.5rem;", "If you have a 25th word, enter it here." }

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
                onclick: on_import_click,
                "Import Wallet"
            }
        }
    }
}