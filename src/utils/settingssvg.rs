

use dioxus::prelude::*;

// Using the #b0b0b0 version to match the Eye and Profile icons
const SETTINGS_ICON_URI: &str = "data:image/svg+xml;base64,PHN2ZwogIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIKICB3aWR0aD0iMjQiCiAgaGVpZ2h0PSIyNCIKICB2aWV3Qm94PSIwIDAgMjQgMjQiCiAgZmlsbD0iI2IwYjBiMCIKPgogIDxwYXRoIGQ9Ik0xNCAyYTYgNiAwIDAgMC00LjkgOS41TDIgMTguNiA1LjQgMjJsNy4xLTcuMUE2IDYgMCAwIDAgMjIgMTBsLTUgMS00LTQgMS01eiIvPgo8L3N2Zz4K
";

#[component]
pub fn SettingsIcon() -> Element {
    rsx! {
        img {
            src: "{SETTINGS_ICON_URI}",
        }
    }
}