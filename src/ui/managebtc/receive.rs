// src/ui/managebtc/receive.rs
use dioxus_native::prelude::*;
use crate::context::{BtcContext, GlobalContext};
use crate::utils::receive_layout::ReceiveAddressLayout;

#[component]
pub fn view() -> Element {
    let mut btc_ctx = use_context::<BtcContext>();
    let global = use_context::<GlobalContext>();

    let (_, address_opt, _) = btc_ctx.bitcoin_wallet.read().clone();
    let address = address_opt.unwrap_or_else(|| "No Address".to_string());
    let is_dark = global.theme_user.read().0;

    rsx! {
        ReceiveAddressLayout {
            network_name: "BITCOIN_NETWORK".to_string(),
            protocol_label: "BITCOIN".to_string(),
            address: address,
            is_dark: is_dark,
            on_back: move |_| {
                btc_ctx.btc_modal.with_mut(|state| {
                    if let Some(prev) = state.last_view.clone() {
                        state.view_type = prev;
                    }
                });
            }
        }
    }
}