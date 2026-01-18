use dioxus::prelude::*;
use crate::VERSION;
use crate::context::GlobalContext;

#[component]
pub fn UpdatePrompt() -> Element {
    let global = use_context::<GlobalContext>();
    let remote_version = global.version.read();
    let display_version = remote_version.as_deref().unwrap_or("latest");

    rsx! {
        style { {r#"
            .update-screen {
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                width: 100%;
                height: 100vh;
                font-family: monospace;
                text-align: center;
            }
        "#} }

        div { class: "update-screen",
            h1 { style: "color: #e74c3c; font-size: 3.5rem; margin: 0;", "UPDATE REQUIRED" }
            
            h2 { style: "font-size: 1.5rem; margin: 1rem 0; opacity: 0.8;", "Version {display_version} available" }
            
            p { 
                style: "max-width: 600px; font-size: 1.2rem; margin: 2rem 0;",
                "Current version {VERSION} is no longer supported. For security, you must update Dannesk before continuing. Download the new version at https://dannesk.com and restart the app."
            }
        }
    }
}