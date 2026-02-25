//src/ui/managebtc/btcimport/step2.rs
//dependent upon src/utils/wallet_security_layout.rs

use dioxus_native::prelude::*;
use crate::context::{GlobalContext, BtcContext};
use crate::ui::managebtc::btcimport::btcimportlogic::BTCImportLogic;
use crate::utils::wallet_security_layout::WalletSecurityForm;
use zeroize::Zeroizing;

#[component]
pub fn view() -> Element {
    let global = use_context::<GlobalContext>();
    let mut btc_ctx = use_context::<BtcContext>();
    
    let mut bip39_buffer = use_signal(|| String::new());
    let mut encryption_buffer = use_signal(|| String::new());

    let on_import_click = move |_| {
        let b_pass = Zeroizing::new(bip39_buffer().trim().to_string());
        let e_pass = Zeroizing::new(encryption_buffer().trim().to_string());
        
        let seed_opt = btc_ctx.btc_wallet_process.read().import_wallet.as_ref().and_then(|w| w.seed.clone());

        if let Some(seed_guard) = seed_opt {
            if seed_guard.is_empty() || e_pass.len() < 10 {
                 btc_ctx.btc_wallet_process.with_mut(|s| {
                    if let Some(ref mut i) = s.import_wallet { i.error = Some("ERR: MIN_10_CHARS_REQUIRED".to_string()); }
                });
                return;
            }
            tokio::spawn(BTCImportLogic::process(seed_guard, b_pass, e_pass, global.ws_tx.clone()));
        }

        bip39_buffer.set(String::new());
        encryption_buffer.set(String::new());
        btc_ctx.btc_wallet_process.with_mut(|s| { if let Some(ref mut i) = s.import_wallet { i.seed = None; } });
    };

    let import_state = btc_ctx.btc_wallet_process.read();
    let current_error = import_state.import_wallet.as_ref().and_then(|i| i.error.clone());

    rsx! {
        WalletSecurityForm {
            flow_label: "IMPORT".to_string(),
            network_label: "BITCOIN_MAINNET".to_string(),
            action_label: "INIT_BTC_IMPORT".to_string(),
            bip39_buffer: bip39_buffer,
            encryption_buffer: encryption_buffer,
            current_error: current_error,
            on_action_click: on_import_click,
        }
    }
}