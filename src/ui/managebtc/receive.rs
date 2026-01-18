use dioxus::prelude::*;
use crate::context::{BtcContext, GlobalContext};
use crate::utils::styles;
use qrcode::QrCode;
use arboard::Clipboard;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

#[component]
pub fn view() -> Element {
    let mut btc_ctx = use_context::<BtcContext>();
    let global = use_context::<GlobalContext>();

    let (_, address_opt, _) = btc_ctx.bitcoin_wallet.read().clone();
    let (is_dark, _, _) = global.theme_user.read().clone();
    
    let address = address_opt.unwrap_or_else(|| "No Address".to_string());

    let qr_data_uri = use_memo(use_reactive((&address, &is_dark), move |(addr, dark_mode)| {
        let code = QrCode::new(addr.as_bytes()).unwrap();
        
        let fg_color = if dark_mode { "#FFFFFF" } else { "#000000" };
        let bg_color = if dark_mode { "#000000" } else { "#FFFFFF" };

        let svg_str = code.render::<qrcode::render::svg::Color>()
    .min_dimensions(200, 200)
    .dark_color(qrcode::render::svg::Color(fg_color))
    .light_color(qrcode::render::svg::Color(bg_color))
    .quiet_zone(false) // Changed to false
    .build();

        let b64 = BASE64.encode(svg_str);
        format!("data:image/svg+xml;base64,{}", b64)
    }));

    let on_back_click = move |_| {
        btc_ctx.btc_modal.with_mut(|state| {
            if let Some(prev) = state.last_view.clone() {
                state.view_type = prev;
            }
        });
    };

    let on_copy_click = {
        let addr = address.clone();
        move |_| {
            if let Ok(mut ctx) = Clipboard::new() {
                let _ = ctx.set_text(addr.clone());
            }
        }
    };

    rsx! {
        style { {r#"
            .receive-container {
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                width: 100%;
            }
            .back-btn {
                position: absolute;
                top: 0.75rem;
                left: 0.75rem;
                cursor: pointer;
                z-index: 10;
            }
            .qr-img {
                width: 15rem;
                height: 15rem;
            }
            .wallet-address {
                font-family: monospace;
                font-size: 1.2rem;
                margin-bottom: 1.5rem;
                margin-top: 1.5rem;
                color: #888;
            }
            .copy-btn {
                width: 8.75rem; 
                height: 2.25rem; 
                background-color: var(--btn); 
                color: #fffef9; 
                border: none; 
                border-radius: 1.375rem; 
                cursor: pointer;
                display: flex;
                justify-content: center;
                align-items: center;
                font-weight: 500;
                transition: all 0.15s ease;
            }
            .copy-btn:hover {
                background: var(--btn-hover);
            }
            .copy-btn:active {
                background: var(--btn-active);
            }
        "#} }

        div { class: "receive-container",
            div { 
                class: "back-btn",
                onclick: on_back_click,
                styles::previous_icon_button { text_color: "#fff".to_string() }
            }

            img { class: "qr-img", src: "{qr_data_uri}" }
            
            div { class: "wallet-address", "{address}" }

            button { 
                class: "copy-btn",
                onclick: on_copy_click,
                "Copy Address"
            }
        }
    }
}