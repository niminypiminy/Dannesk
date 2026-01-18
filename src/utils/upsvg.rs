
use dioxus::prelude::*;

// Using the #b0b0b0 version to match the Eye and Profile icons
const UP_ICON_URI: &str = "data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjQiIGhlaWdodD0iMjQiIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8cGF0aCBkPSJNMTIgMThWNk0xMiA2TDkgOU0xMiA2TDE1IDkiCiAgICAgICAgc3Ryb2tlPSIjYjBiMGIwIgogICAgICAgIHN0cm9rZS13aWR0aD0iMSIKICAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIvPgo8L3N2Zz4K";

#[component]
pub fn UpIcon() -> Element {
    rsx! {
        img {
            src: "{UP_ICON_URI}",
        }
    }
}