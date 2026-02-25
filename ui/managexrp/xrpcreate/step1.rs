//src/ui/managexrp/xrpcreate/step1.rs
//dependent upon src/utils/create_seed_layout.rs

use dioxus_native::prelude::*;
use crate::context::{XrpContext, GlobalContext};
use crate::channel::XRPWalletProcessState;
use crate::utils::create_seed_layout::CreateSeedForm; // Import the new layout
use arboard::Clipboard;

#[component]
pub fn view() -> Element {
    let xrp_ctx = use_context::<XrpContext>();
    let global = use_context::<GlobalContext>();
    let _ = global.theme_user.read().0; // Theme reactivity
    
    let mut wallet_process = xrp_ctx.wallet_process;
    
    // Extract words from state
    let words: Vec<String> = wallet_process.read().create_wallet
        .as_ref()
        .and_then(|s| s.seed.as_ref())
        .map(|s| s.split_whitespace().map(|w| w.to_string()).collect())
        .unwrap_or_default();

    let on_continue = move |_| {
        wallet_process.with_mut(|state: &mut XRPWalletProcessState| {
            if let Some(ref mut create) = state.create_wallet {
                create.step = 2;
            }
        });
    };

    let on_copy = move |_| {
        if let Ok(mut ctx) = Clipboard::new() {
            if let Some(ref create) = wallet_process.read().create_wallet {
                if let Some(ref seed) = create.seed {
                    let _ = ctx.set_text(seed.to_string());
                }
            }
        }
    };

    rsx! {
        CreateSeedForm {
            network_label: "XRP_MAINNET".to_string(),
            words: words,
            on_copy: on_copy,
            on_continue: on_continue,
        }
    }
}