//src/ui/managexrp/xrpsend/step4.rs
//dependent upon utils/auth_layout.rs

use dioxus_native::prelude::*;
use crate::context::{GlobalContext, XrpContext};
use crate::ui::managexrp::xrpsend::sendlogic::XRPSendLogic;
use crate::utils::send_auth_layout::SendAuthForm;

#[component]
pub fn view() -> Element {
    let global = use_context::<GlobalContext>();
    let xrp_ctx = use_context::<XrpContext>();
    let mut sign_transaction = xrp_ctx.sign_transaction;

    let (_, address_opt, _) = xrp_ctx.wallet_balance.read().clone();
    let wallet_address = address_opt.unwrap_or_else(|| "NULL_ADDR".to_string());

    let input_mode = use_signal(|| "passphrase".to_string()); 
    let passphrase_val = use_signal(|| String::new());
    let bip39_val = use_signal(|| String::new());
    let seed_words = use_signal(|| vec![String::new(); 24]);

    let current_error = sign_transaction.read()
        .send_transaction.as_ref()
        .and_then(|s| s.error.clone());

    let on_submit = move |_| {
        let mode = input_mode();
        let p_val = passphrase_val();
        let s_words = seed_words.read();
        let b39 = bip39_val();
        let last_v = xrp_ctx.xrp_modal.read().last_view.clone();

        let mut validation_error: Option<String> = None;

        match mode.as_str() {
            "passphrase" => {
                if p_val.trim().is_empty() {
                    validation_error = Some("ERR: PASSPHRASE_REQUIRED".to_string());
                }
            },
            "seed" => {
                let word_count = s_words.iter().filter(|w| !w.trim().is_empty()).count();
                if word_count != 24 {
                    validation_error = Some("ERR: MNEMONIC_LENGTH_MISMATCH".to_string());
                }
            },
            _ => { validation_error = Some("ERR: INVALID_MODE".to_string()); }
        }

        if let Some(err) = validation_error {
            sign_transaction.with_mut(|state| {
                if let Some(ref mut send) = state.send_transaction {
                    send.error = Some(err);
                }
            });
            return;
        }

        let (recipient, amount, asset) = {
            let state = sign_transaction.read();
            let st = state.send_transaction.as_ref().unwrap();
            (st.recipient.clone().unwrap_or_default(), st.amount.clone().unwrap_or_default(), st.asset.clone())
        };
        
        let seed_string = s_words.iter().filter(|w| !w.is_empty()).cloned().collect::<Vec<_>>().join(" ");
        
        tokio::spawn(XRPSendLogic::process(
            mode,
            p_val,
            seed_string,
            b39,
            recipient,
            amount,
            wallet_address.clone(),
            asset,
            global.ws_tx.clone(),
            last_v,
        ));
    };

    rsx! {
        SendAuthForm {
            // Pass the step-specific title here
            step_title: "TRANSACTION_AUTHORIZATION // STEP_04".to_string(),
            network_label: "XRPL_MAINNET",
            input_mode,
            passphrase_val,
            seed_words,
            bip39_val,
            current_error,
            on_submit,
            on_clear_error: move |_| {
                sign_transaction.with_mut(|s| {
                    if let Some(ref mut tx) = s.send_transaction { 
                        tx.error = None; 
                    }
                });
            }
        }
    }
}