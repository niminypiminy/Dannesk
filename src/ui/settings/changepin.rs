use dioxus::prelude::*;
use crate::ui::settings::pinlogic::PinLogic;

#[component]
pub fn view() -> Element {
    let mut old_pin = use_signal(String::new);
    let mut new_pin = use_signal(String::new);
    let mut confirm_pin = use_signal(String::new);

    let mut on_submit = move |_| {
        if new_pin().is_empty() || old_pin().is_empty() {
            return;
        }
        if new_pin() != confirm_pin() {
            return;
        }
        
        tokio::spawn(PinLogic::change_pin(old_pin(), new_pin()));

        old_pin.set(String::new());
        new_pin.set(String::new());
        confirm_pin.set(String::new());
    };

    rsx! {
        div {
            // Main Container: Fully centered axis
            style: "display: flex; flex-direction: column; width: 100%; align-items: center;",

            // --- Title Section (Centered) ---
            div { style: "font-size: 1.5rem; font-weight: 500; margin-bottom: 0.25rem; text-align: center;", 
                "Security PIN" 
            }
            div { style: "font-size: 1rem; color: #999; margin-bottom: 2rem; margin-top:1rem; text-align: center;", 
                "Update your transaction authorization PIN." 
            }

            // Inputs (Your original CSS)
            input {
                style: "width: 100%; max-width: 18rem; height: 2.5rem; padding: 0.5rem; background-color: transparent; border: 1px solid #444; border-radius: 0.25rem; font-size: 1.1rem; color: white; margin-bottom: 0.75rem; box-sizing: border-box; text-align: center;",
                r#type: "password",
                placeholder: "Old PIN",
                value: "{old_pin}",
                oninput: move |e| old_pin.set(e.value()),
            }
            input {
                style: "width: 100%; max-width: 18rem; height: 2.5rem; padding: 0.5rem; background-color: transparent; border: 1px solid #444; border-radius: 0.25rem; font-size: 1.1rem; color: white; margin-bottom: 0.75rem; box-sizing: border-box; text-align: center;",
                r#type: "password",
                placeholder: "New PIN",
                value: "{new_pin}",
                oninput: move |e| new_pin.set(e.value()),
            }
            input {
                style: "width: 100%; max-width: 18rem; height: 2.5rem; padding: 0.5rem; background-color: transparent; border: 1px solid #444; border-radius: 0.25rem; font-size: 1.1rem; color: white; box-sizing: border-box; text-align: center;",
                r#type: "password",
                placeholder: "Confirm PIN",
                value: "{confirm_pin}",
                oninput: move |e| confirm_pin.set(e.value()),
                onkeydown: move |e| if e.key() == Key::Enter { on_submit(()); }
            }

            // Button (Your original CSS)
            button {
                style: "width: 10rem; height: 2.5rem; background-color: #333; color: white; border: none; 
                border-radius: 1.375rem; font-size: 1rem; cursor: pointer; display: flex; justify-content: center; align-items: center; margin-top: 1.5rem;",
                onclick: move |_| on_submit(()),
                "Update PIN"
            }
        }
    }
}