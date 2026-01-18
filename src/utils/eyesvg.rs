use dioxus::prelude::*;

const EYE_OPEN_URI: &str = "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgc3Ryb2tlPSIjYjBiMGIwIiBzdHJva2Utd2lkdGg9IjIiIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCI+PHBhdGggZD0iTTEgMTJzNC04IDExLTggMTEgOCAxMSA4LTQgOC0xMSA4LTExLTgtMTEtOHoiLz48Y2lyY2xlIGN4PSIxMiIgY3k9IjEyIiByPSIzIi8+PC9zdmc+";

const EYE_CLOSE_URI: &str = "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgc3Ryb2tlPSIjYjBiMGIwIiBzdHJva2Utd2lkdGg9IjIiIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCI+PGxpbmUgeDE9IjEiIHkxPSIxIiB4Mj0iMjMiIHkyPSIyMyIgc3Ryb2tlPSIjYjBiMGIwIiAvPjxwYXRoIGQ9Ik0xNy45NCAxNy45NEExMC45NCAxMC45NCAwIDAgMSAxMiAxOWMtNyAwLTExLTctMTEtN2EyMS43NyAyMS43NyAwIDAgMSA1LjA2LTUuOTQiLz48cGF0aCBkPSJNMTIgNWM3IDAgMTEgNyAxMSA3YTIxLjgyIDIxLjgyIDAgMCAxLTUuMDYgNS45NCIvPjxwYXRoIGQ9Ik05Ljg4IDkuODhBMyAzIDAgMSAwIDE0LjEyIDE0LjEyIiAvPjwvc3ZnPg==";

#[component]
pub fn EyeIcon(open: bool) -> Element {
    let src = if open { EYE_OPEN_URI } else { EYE_CLOSE_URI };
    rsx! {
        img { src: "{src}" }
    }
}