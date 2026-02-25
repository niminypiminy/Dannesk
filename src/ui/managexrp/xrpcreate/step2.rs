//src/ui/managexrp/xrpcreate/step2.rs
//dependent upon src/utils/wallet_security_layout.rs

use dioxus_native::prelude::*;
use crate::context::{GlobalContext, XrpContext};
use crate::ui::managexrp::xrpcreate::xrpcreatelogic::XRPCreateLogic;
use crate::utils::wallet_security_layout::WalletSecurityForm;
use zeroize::Zeroizing;

#[component]
pub fn view() -> Element {
    let global = use_context::<GlobalContext>();
    let mut xrp_ctx = use_context::<XrpContext>();
    let _ = global.theme_user.read().0;
    
    let mut bip39_buffer = use_signal(|| String::new());
    let mut encryption_buffer = use_signal(|| String::new());

    let on_create_click = move |_| {
        let b_pass = Zeroizing::new(bip39_buffer().trim().to_string());
        let e_pass = Zeroizing::new(encryption_buffer().trim().to_string());
        
        let seed_opt = xrp_ctx.wallet_process.read()
            .create_wallet.as_ref()
            .and_then(|w| w.seed.clone());

        if let Some(seed_guard) = seed_opt {
            if e_pass.len() < 10 {
                 xrp_ctx.wallet_process.with_mut(|state| {
                    if let Some(ref mut create) = state.create_wallet {
                        create.error = Some("ERR: MIN_10_CHARS_REQUIRED".to_string());
                    }
                });
                return;
            }
            tokio::spawn(XRPCreateLogic::process(seed_guard, b_pass, e_pass, global.ws_tx.clone()));
        }

        bip39_buffer.set(String::new());
        encryption_buffer.set(String::new());
        
        xrp_ctx.wallet_process.with_mut(|state| {
            if let Some(ref mut create) = state.create_wallet {
                create.seed = None; 
                create.error = None;
            }
        });
    };

    let create_state = xrp_ctx.wallet_process.read();
    let current_error = create_state.create_wallet.as_ref().and_then(|i| i.error.clone());

    rsx! {
        WalletSecurityForm {
            flow_label: "CREATION".to_string(),
            network_label: "XRP_MAINNET".to_string(),
            action_label: "INIT_XRP_CREATION".to_string(),
            bip39_buffer,
            encryption_buffer,
            current_error,
            on_action_click: on_create_click,
        }
    }
}