use dioxus::prelude::*;
use crate::context::GlobalContext;
use crate::ui::settings::namelogic::NameLogic;

#[component]
pub fn view() -> Element {
    let global = use_context::<GlobalContext>();
    let mut name_buffer = use_signal(String::new);
    let (is_dark, _, hide_balance) = global.theme_user.read().clone();

    let mut on_submit = move |_| {
        let new_name = name_buffer().trim().to_string();
        if new_name.is_empty() {
            return;
        }

        // FIRE AND FORGET LOGIC
        tokio::spawn(NameLogic::update_name(new_name, is_dark, hide_balance));

        // Clear local buffer
        name_buffer.set(String::new());
    };

    rsx! {
        div {
            // Main Container: Centering everything horizontally
            style: "display: flex; flex-direction: column; width: 100%; align-items: center; justify-content: center;",

            // --- Title Section (Centered) ---
            div { 
                style: "text-align: center; margin-bottom: 2rem;",
                div { style: "font-size: 1.5rem; font-weight: 500; margin-bottom: 0.25rem;", 
                    "Display Name" 
                }
                div { style: "font-size: 1rem; color: #999; margin-top: 1rem;", 
                    "Update how your name appears across the application." 
                }
            }

            // Input Field (Your original centered CSS)
            input {
                style: "width: 100%; max-width: 33rem; height: 2.5rem; padding: 0.5rem; background-color: transparent; border: 1px solid #444; border-radius: 0.25rem; font-size: 1.25rem; color: white; box-sizing: border-box; text-align: center;",
                placeholder: "Enter New Name",
                value: "{name_buffer}",
                oninput: move |e| name_buffer.set(e.value()),
                onkeydown: move |e| if e.key() == Key::Enter { on_submit(()); }
            }

            // Button (Your original flex-centered CSS)
            button {
                style: "width: 10rem; height: 2.5rem; background-color: #333; color: white; border: none; 
                border-radius: 1.375rem; font-size: 1rem; display: flex; cursor: pointer; justify-content: center; align-items: center; margin-top: 1.5rem;",
                onclick: move |_| on_submit(()),
                "Update Name"
            }
        }
    }
}