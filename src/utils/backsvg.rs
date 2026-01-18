

use dioxus::prelude::*;

// Using the #b0b0b0 version to match the Eye and Profile icons
const BACK_ICON_URI: &str = "data:image/svg+xml;base64,PHN2ZyBmaWxsPSIjYjBiMGIwIiB3aWR0aD0iMjRweCIgaGVpZ2h0PSIyNHB4IiB2aWV3Qm94PSIwIDAgMjQgMjQiCiAgICAgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8cGF0aCBkPSJNMjEsMTFINS40MWwxLjMtMS4yOUExLDEsMCwwLDAsNS4yOSw4LjI5bC0zLDNhMSwxLDAsMCwwLDAsMS40MmwzLDNhMSwxLDAsMCwwLDEuNDIsMCwxLDEsMCwwLDAsMC0xLjQyTDUuNDEsMTNIMjFhMSwxLDAsMCwwLDAtMloiLz4KPC9zdmc+Cg==";

#[component]
pub fn BackIcon() -> Element {
    rsx! {
        img {
            src: "{BACK_ICON_URI}",
        }
    }
}