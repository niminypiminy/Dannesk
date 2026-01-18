// src/ui/utils.rs
use dioxus::prelude::*;
use crate::utils::backsvg::BackIcon;



#[component]
pub fn previous_icon_button(text_color: String) -> Element {
    return rsx! {
        button {
            style: "
                padding: 0.2rem 0.4rem;
                font-size: 0.5rem;
                border-radius: 1rem;
                border: 1px solid #444;
                background: var(--btn);
                color: {text_color};
                cursor: pointer;
            ",
            BackIcon {}
        }
    };
}

