use dioxus_native::prelude::*;
use crate::context::XrpContext;
use crate::utils::send_recipient_layout::SendAddressForm;

#[component]
pub fn view() -> Element {
    let xrp_ctx = use_context::<XrpContext>();
    let mut sign_transaction = xrp_ctx.sign_transaction;

    let mut addr_buffer = use_signal(|| {
        sign_transaction.read()
            .send_transaction.as_ref()
            .and_then(|s| s.recipient.clone())
            .unwrap_or_default()
    });

    let on_input = move |e: FormEvent| {
        let clean_val = e.value().replace(['\n', '\r'], "");
        addr_buffer.set(clean_val);
        sign_transaction.with_mut(|state| {
            if let Some(ref mut send) = state.send_transaction {
                send.error = None;
            }
        });
    };

    let on_next_click = move |_| {
        let addr = addr_buffer().trim().to_string();
        
        if addr.is_empty() {
            sign_transaction.with_mut(|s| s.send_transaction.as_mut().map(|t| t.error = Some("ERR: RECIPIENT_REQUIRED".to_string())));
            return;
        } 
        
        if !addr.starts_with('r') || addr.len() < 25 || addr.len() > 35 {
            sign_transaction.with_mut(|s| s.send_transaction.as_mut().map(|t| t.error = Some("ERR: INVALID_XRP_ADDR_FORMAT".to_string())));
            return;
        }

        sign_transaction.with_mut(|state| {
            if let Some(ref mut send) = state.send_transaction {
                send.recipient = Some(addr); 
                send.error = None;
                send.step = 2;
            }
        });
    };

    let current_error = sign_transaction.read().send_transaction.as_ref()
        .and_then(|s| s.error.clone());

    rsx! {
        SendAddressForm {
            network_label: "XRP_MAINNET".to_string(),
            address_buffer: addr_buffer,
            placeholder: "r...".to_string(),
            current_error: current_error,
            on_input: on_input,
            on_next_click: on_next_click,
        }
    }
}