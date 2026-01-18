use dioxus::prelude::*;

// Using the #b0b0b0 version to match the Eye and Profile icons
const DOWN_ICON_URI: &str = "data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjQiIGhlaWdodD0iMjQiIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8cGF0aCBkPSJNMTIgNlYxOE0xMiAxOEw5IDE1TTEyIDE4TDE1IDE1IgogICAgICAgIHN0cm9rZT0iI2IwYjBiMCIKICAgICAgICBzdHJva2Utd2lkdGg9IjEiCiAgICAgICAgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIgogICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiLz4KPC9zdmc+Cg==";

#[component]
pub fn DownIcon() -> Element {
    rsx! {
        img {
            src: "{DOWN_ICON_URI}",
        }
    }
}