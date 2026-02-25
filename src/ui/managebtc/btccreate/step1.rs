//src/ui/managebtc/btccreate/step1.rs
//dependent upon src/utils/create_seed_layout.rs

use dioxus_native::prelude::*;
use crate::context::{BtcContext, GlobalContext};
use crate::channel::BTCWalletProcessState;
use crate::utils::create_seed_layout::CreateSeedForm; // The new centralized layout
use arboard::Clipboard;

#[component]
pub fn view() -> Element {
    let btc_ctx = use_context::<BtcContext>();
    let global = use_context::<GlobalContext>();
    
    // Theme reactivity
    let _ = global.theme_user.read().0;
    
    let mut btc_wallet_process = btc_ctx.btc_wallet_process;
    
    // Extract words from Bitcoin-specific context
    let words: Vec<String> = btc_wallet_process.read().create_wallet
        .as_ref()
        .and_then(|s| s.seed.as_ref())
        .map(|s| s.split_whitespace().map(|w| w.to_string()).collect())
        .unwrap_or_default();

    // Local Logic: Bitcoin State Transition
    let on_next_click = move |_| {
        btc_wallet_process.with_mut(|state: &mut BTCWalletProcessState| {
            if let Some(ref mut create) = state.create_wallet {
                create.step = 2;
            }
        });
    };

    // Local Logic: Clipboard Handling
    let on_copy_click = move |_| {
        if let Ok(mut ctx) = Clipboard::new() {
            if let Some(ref create) = btc_wallet_process.read().create_wallet {
                if let Some(ref seed) = create.seed {
                    let _ = ctx.set_text(seed.to_string());
                }
            }
        }
    };

    rsx! {
        CreateSeedForm {
            network_label: "BITCOIN_MAINNET".to_string(),
            words: words,
            on_copy: on_copy_click,
            on_continue: on_next_click,
        }
    }
}