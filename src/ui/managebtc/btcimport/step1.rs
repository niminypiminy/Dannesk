//src/ui/managebtc/btcimport/step1.rs
//dependent upon src/utils/import_seed_layout.rs

use dioxus_native::prelude::*;
use crate::context::{BtcContext, GlobalContext};
use crate::channel::BTCWalletProcessState;
use crate::utils::import_seed_layout::ImportSeedForm;
use zeroize::Zeroizing;

#[component]
pub fn view() -> Element {
    let btc_ctx = use_context::<BtcContext>();
    let global = use_context::<GlobalContext>();
    
    // Theme reactivity
    let _ = global.theme_user.read().0;
    
    let mut btc_wallet_process = btc_ctx.btc_wallet_process;
    let seed_words = use_signal(|| vec![String::new(); 24]);
    let mut error_msg = use_signal(|| None::<String>);

    let on_continue = move |_| {
        let current_words = seed_words.read().clone();
        let word_count = current_words.iter().filter(|w| !w.is_empty()).count();

        if word_count != 24 {
            error_msg.set(Some("MNEMONIC_INTEGRITY_CHECK_FAILED: 24_WORDS_REQUIRED".to_string()));
            return;
        }

        let seed_phrase = Zeroizing::new(current_words.join(" "));

        btc_wallet_process.with_mut(|state: &mut BTCWalletProcessState| {
            if let Some(ref mut import) = state.import_wallet {
                import.seed = Some(seed_phrase);
                import.error = None;
                import.step = 2; 
            }
        });
    };

    rsx! {
        ImportSeedForm {
            // Pass in the BTC-specific label to dynamically render the header
            network_label: "BITCOIN_MAINNET".to_string(),
            seed_words: seed_words,
            error_msg: error_msg,
            on_continue: on_continue,
        }
    }
}