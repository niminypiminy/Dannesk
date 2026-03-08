use dioxus_native::prelude::*;
use crate::VERSION;
use crate::context::GlobalContext;

#[component]
pub fn UpdatePrompt() -> Element {
    let global = use_context::<GlobalContext>();
    let remote_version = global.version.read();
    let display_version = remote_version.as_deref().unwrap_or("vX.X.X");

    rsx! {
        style { {r#"
            .update-overlay {
                height: 100vh;
                width: 100%;
                display: flex;
                align-items: center;
                justify-content: center;
                background: var(--bg-primary);
                font-family: 'Inter', 'JetBrains Mono', monospace;
                color: var(--text);
            }

            .alert-frame {
                /* Removed red border, replaced with subtle accent */
                border: 1px solid var(--border);
                background: var(--bg-secondary);
                padding: 3.5rem;
                max-width: 600px;
                border-radius: 8px;
                box-shadow: 0 20px 50px rgba(0, 0, 0, 0.3);
            }

            .version-compare {
                display: flex;
                align-items: center;
                gap: 2rem;
                margin: 2rem 0;
                padding: 1.5rem;
                background: rgba(255, 255, 255, 0.03);
                border-radius: 6px;
            }

            .v-node {
                display: flex;
                flex-direction: column;
                gap: 4px;
            }

            .v-label {
                font-size: 0.65rem;
                color: var(--text-secondary);
                letter-spacing: 1px;
            }

            .v-value {
                font-size: 1.1rem;
                font-weight: 600;
            }

            .instruction-box {
                text-align: left;
                margin-top: 2rem;
            }

            .download-link {
                display: inline-block;
                margin-top: 1rem;
                color: var(--accent);
                text-decoration: none;
                font-weight: bold;
                padding: 10px 20px;
                border: 1px solid var(--accent);
                border-radius: 4px;
            }
            
            .download-link:hover {
                background: var(--accent);
                color: var(--bg-primary);
            }
        "#} }

        div { class: "update-overlay",
            div { class: "alert-frame",
                
                div { 
                    style: "font-size: 1.8rem; font-weight: 700; margin-bottom: 0.5rem;", 
                    "New Version Available" 
                }

                div { 
                    style: "font-size: 0.9rem; color: var(--text-secondary);", 
                    "A newer build of Dannesk Core is ready for download." 
                }

                div { class: "version-compare",
                    div { class: "v-node",
                        span { class: "v-label", "Current" }
                        span { class: "v-value", "{VERSION}" }
                    }
                    div { 
                        style: "font-size: 1.2rem; color: var(--text-secondary);",
                        "→"
                    }
                    div { class: "v-node",
                        span { class: "v-label", "Latest" }
                        span { class: "v-value", style: "color: var(--status-ok)", "{display_version}" }
                    }
                }

                div { class: "instruction-box",
                    p { style: "font-size: 0.95rem; line-height: 1.6; color: var(--text-secondary);",
                        "To continue using the latest features and improvements, please update your application."
                    }
                    
                    a { 
                        class: "download-link", 
                        href: "https://dannesk.com",
                        "Download Update" 
                    }
                }

                div { 
                    style: "margin-top: 3rem; font-size: 0.7rem; color: var(--text-secondary);",
                    "Build Sync: Stable"
                }
            }
        }
    }
}