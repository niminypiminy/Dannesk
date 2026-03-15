// utils/xrp.rs

use dioxus_native::prelude::*;

// LIGHT THEME VERSION (Dark Glyph)
#[component]
pub fn XrpLogo(size: String) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 512 424",
            preserve_aspect_ratio: "xMidYMid meet",

            path {
                fill: "#23292f",
                d: "M437,0h74L357,152.48c-55.77,55.19-146.19,55.19-202,0L.94,0H75L192,115.83a91.11,91.11,0,0,0,127.91,0Z"
            }
            path {
                fill: "#23292f",
                d: "M74.05,424H0L155,270.58c55.77-55.19,146.19-55.19,202,0L512,424H438L320,307.23a91.11,91.11,0,0,0-127.91,0Z"
            }
        }
    }
}

// DARK THEME VERSION (White Glyph)
#[component]
pub fn XrpLogoWhite(size: String) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 512 424",
            preserve_aspect_ratio: "xMidYMid meet",

            path {
                fill: "#FFFFFF",
                d: "M437,0h74L357,152.48c-55.77,55.19-146.19,55.19-202,0L.94,0H75L192,115.83a91.11,91.11,0,0,0,127.91,0Z"
            }
            path {
                fill: "#FFFFFF",
                d: "M74.05,424H0L155,270.58c55.77-55.19,146.19-55.19,202,0L512,424H438L320,307.23a91.11,91.11,0,0,0-127.91,0Z"
            }
        }
    }
}