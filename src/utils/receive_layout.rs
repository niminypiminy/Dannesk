// src/utils/receive_layout.rs
use dioxus_native::prelude::*;
use crate::utils::styles::terminal_action;
use qrcode::QrCode;
use arboard::Clipboard;

#[component]
pub fn ReceiveAddressLayout(
    network_name: String,
    protocol_label: String,
    address: String,
    is_dark: bool,
    on_back: EventHandler<MouseEvent>,
) -> Element {
    // Memoize QR generation to avoid expensive re-renders
    let qr_uri = use_memo(use_reactive((&address, &is_dark), move |(addr, dark)| {
        let code = QrCode::new(addr.as_bytes()).unwrap();
        let color = if dark { "#FFFFFF" } else { "#000000" };
        
        let svg_str = code.render::<qrcode::render::svg::Color>()
            .min_dimensions(200, 200)
            .dark_color(qrcode::render::svg::Color(color)) 
            .light_color(qrcode::render::svg::Color("transparent"))
            .quiet_zone(false) 
            .build();

        format!("data:image/svg+xml;utf8,{}", svg_str.replace("#", "%23"))
    }));

    let copy_action = {
        let addr = address.clone();
        move |_| {
            if let Ok(mut ctx) = Clipboard::new() {
                let _ = ctx.set_text(addr.clone());
            }
        }
    };

    rsx! {
        style { {r#"
            .terminal-receive {
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                width: 100%;
                font-family: 'JetBrains Mono', monospace;
            }
            .qr-img {
                width: 16rem;
                height: 16rem;
                image-rendering: pixelated;
                margin-bottom: 2rem;
            }
            .address-display {
                color: var(--text-secondary);
                font-size: 0.8rem;
                background: transparent;
                padding: 1rem;
                border: 1px dashed var(--border);
                margin-bottom: 2rem;
                text-align: center;
                max-width: 85%;
                word-break: break-all;
            }
        "#} }

        div { class: "terminal-receive",
            div { 
                style: "margin-bottom: 2rem; text-align: center;",
                div { style: "font-size: 0.65rem; color: var(--accent); letter-spacing: 2px;", "RECEIVE_ADDRESS" }
                div { style: "font-size: 1.2rem; font-weight: bold; color: var(--text);", "{network_name}" }
            }

            img { class: "qr-img", src: "{qr_uri}" }
            
            div { class: "address-display", "{address}" }

            div { 
                style: "display: flex; gap: 1rem;",
                {terminal_action("<<_BACK", true, move |e| on_back.call(e))}
                {terminal_action("COPY_TO_CLIPBOARD", true, copy_action)}
            }

            div { 
                style: "margin-top: 3rem; color: var(--text-secondary); font-size: 0.6rem; opacity: 0.3;", 
                "PROTOCOL // {protocol_label}" 
            }
        }
    }
}