// src/utils/receive_layout.rs
use dioxus_native::prelude::*;
use crate::utils::styles::terminal_action;
use qrcode::{QrCode, types::Color};
use arboard::Clipboard;

#[component]
pub fn ReceiveAddressLayout(
    network_name: String,
    protocol_label: String,
    address: String,
    is_dark: bool,
    on_back: EventHandler<MouseEvent>,
) -> Element {
    // Fast vector QR code (Vello/Blitz renders instantly)
    let qr_modules = use_memo(use_reactive(&address, move |addr| {
        if addr == "No Address" {
            return (0u32, vec![]);
        }
        let code = QrCode::new(addr.as_bytes()).unwrap();
        let width = code.width() as u32;
        let mut dark = Vec::with_capacity((width * width) as usize);
        for y in 0..width {
            for x in 0..width {
                if code[(x as usize, y as usize)] == Color::Dark {
                    dark.push((x, y));
                }
            }
        }
        (width, dark)
    }));

    let (grid_size, dark_modules) = qr_modules.read().clone();
    let module_color = if is_dark { "#ffffff" } else { "#000000" };
    let bg_color = if is_dark { "#111111" } else { "#ffffff" };

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
                width: 280px !important;      /* big & reliable */
                height: 280px !important;
                margin-bottom: 2rem;
                image-rendering: crisp-edges;
                display: block;
                flex-shrink: 0;
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

            // ← Big, instant, perfect QR
            svg {
                class: "qr-img",
                view_box: "-4 -4 {grid_size + 8} {grid_size + 8}",
                preserve_aspect_ratio: "xMidYMid meet",

                // Quiet zone (4 modules padding, looks professional)
                rect {
                    x: "-4",
                    y: "-4",
                    width: "{grid_size + 8}",
                    height: "{grid_size + 8}",
                    fill: "{bg_color}",
                }

                for (x, y) in dark_modules {
                    rect {
                        x: "{x}",
                        y: "{y}",
                        width: "1",
                        height: "1",
                        fill: "{module_color}",
                    }
                }
            }
            
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