//src/ui/managebtc/btcsend/step1.rs 
//dependent upon utils/send_recipient_layout.rs

use dioxus_native::prelude::*;
use crate::context::BtcContext;
use crate::utils::send_recipient_layout::SendAddressForm;

#[component]
pub fn view() -> Element {
    let btc_ctx = use_context::<BtcContext>();
    let mut btc_sign_transaction = btc_ctx.btc_sign_transaction;

    // Initialize local buffer with signal for localized re-renders
    let mut addr_buffer = use_signal(|| {
        btc_sign_transaction.read()
            .send_transaction.as_ref()
            .and_then(|s| s.recipient.clone())
            .unwrap_or_default()
    });

    let on_input = move |e: FormEvent| {
        let clean_val = e.value().replace(['\n', '\r'], "");
        addr_buffer.set(clean_val);
        btc_sign_transaction.with_mut(|state| {
            if let Some(ref mut send) = state.send_transaction {
                send.error = None;
            }
        });
    };

    let on_next_click = move |_| {
        let addr = addr_buffer().trim().to_string();
        
        // BTC Validation Logic (SegWit check)
        if addr.is_empty() {
            btc_sign_transaction.with_mut(|s| s.send_transaction.as_mut().map(|t| t.error = Some("ERR: RECIPIENT_REQUIRED".to_string())));
            return;
        } 
        
        if !addr.starts_with("bc1") || addr.len() < 25 || addr.len() > 60 {
            btc_sign_transaction.with_mut(|s| s.send_transaction.as_mut().map(|t| t.error = Some("ERR: INVALID_BTC_ADDR_FORMAT".to_string())));
            return;
        }

        btc_sign_transaction.with_mut(|state| {
            if let Some(ref mut send) = state.send_transaction {
                send.recipient = Some(addr); 
                send.error = None;
                send.step = 2;
            }
        });
    };

    let current_error = btc_sign_transaction.read().send_transaction.as_ref()
        .and_then(|s| s.error.clone());

    rsx! {
        SendAddressForm {
            network_label: "BITCOIN_MAINNET".to_string(),
            address_buffer: addr_buffer,
            placeholder: "bc1...".to_string(),
            current_error: current_error,
            on_input: on_input,
            on_next_click: on_next_click,
        }
    }
}