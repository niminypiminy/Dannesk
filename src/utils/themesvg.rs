use dioxus::prelude::*;

const MOON_URI: &str = "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0iI2IwYjBiMCI+PHBhdGggZD0iTTIxIDEyLjhBOSA5IDAgMSAxIDExLjIgM2E3IDcgMCAwIDAgOS44IDkuOHoiLz48L3N2Zz4=";

const SUN_URI: &str = "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgc3Ryb2tlPSIjYjBiMGIwIiBzdHJva2Utd2lkdGg9IjIiIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCI+PGNpcmNsZSBjeD0iMTIiIGN5PSIxMiIgcj0iNSIvPjxsaW5lIHgxPSIxMiIgeTE9IjEiIHgyPSIxMiIgeTI9IjMiLz48bGluZSB4MT0iMTIiIHkxPSIyMSIgeDI9IjEyIiB5Mj0iMjMiLz48bGluZSB4MT0iNC4yMiIgeTE9IjQuMjIiIHgyPSI1LjY0IiB5Mj0iNS42NCIvPjxsaW5lIHgxPSIxOC4zNiIgeTE9IjE4LjM2IiB4Mj0iMTkuNzgiIHkyPSIxOS43OCIvPjxsaW5lIHgxPSIxIiB5MT0iMTIiIHgyPSIzIiB5Mj0iMTIiLz48bGluZSB4MT0iMjEiIHkxPSIxMiIgeDI9IjIzIiB5Mj0iMTIiLz48bGluZSB4MT0iNC4yMiIgeTE9IjE5Ljc4IiB4Mj0iNS42NCIgeTI9IjE4LjM2Ii8+PGxpbmUgeDE9IjE4LjM2IiB5MT0iNS42NCIgeDI9IjE5Ljc4IiB5Mj0iNC4yMiIvPjwvc3ZnPg==";

#[component]
pub fn ThemeIcon(dark: bool) -> Element {
    let src = if dark { SUN_URI } else { MOON_URI };
    rsx! {
        img { src: "{src}" }
    }
}