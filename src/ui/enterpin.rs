// src/ui/enterpin.rs
use dioxus::prelude::*;
use crate::utils::json_storage;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq)]
enum PinState {
    EnterPin,
    SetPin,
    ConfirmPin,
}

#[component]
pub fn PinScreen(on_unlock: EventHandler<()>) -> Element {
    let mut input = use_signal(|| String::new());
    let mut error_msg = use_signal(|| None::<String>);
    let mut stored_pin_for_confirmation = use_signal(|| String::new()); 
    let mut attempts_left = use_signal(|| 5); 

    let pin_exists = use_memo(|| json_storage::read_json::<crate::pin::PinData>("pin.json").is_ok());

    let state = use_memo(move || {
        if !pin_exists() {
            if stored_pin_for_confirmation().is_empty() {
                PinState::SetPin
            } else {
                PinState::ConfirmPin
            }
        } else {
            PinState::EnterPin
        }
    });

    let mut trigger_error = move |msg: String| {
        error_msg.set(Some(msg.clone()));
        spawn(async move {
            tokio::time::sleep(Duration::from_secs(3)).await; 
            if error_msg.read().as_ref() == Some(&msg) {
                error_msg.set(None);
            }
        });
    };

    let mut reset_setup_state = move || {
        input.set(String::new());
        stored_pin_for_confirmation.set(String::new());
    };

    let mut submit = move || {
        let pin = input.read().clone();
        error_msg.set(None); 

        if state() == PinState::EnterPin && *attempts_left.read() == 0 {
             error_msg.set(Some("Too many failed attempts. App is locked.".to_string()));
             input.set(String::new());
             return;
        }

        if pin.len() != 6 {
            trigger_error("PIN must be 6 digits.".to_string());
            return;
        }

        match state() {
            PinState::SetPin => {
                stored_pin_for_confirmation.set(pin);
                input.set(String::new());
            }
            PinState::ConfirmPin => {
                let original_pin = stored_pin_for_confirmation.read().clone();
                if pin == original_pin {
                    match crate::pin::set_pin(&pin) {
                        Ok(()) => on_unlock.call(()),
                        Err(e) => {
                            trigger_error(format!("Error: {}", e));
                            reset_setup_state();
                        }
                    }
                } else {
                    trigger_error("PINs do not match. Resetting.".to_string());
                    reset_setup_state();
                }
            }
            PinState::EnterPin => {
                if crate::pin::verify_pin(&pin).is_ok() {
                    on_unlock.call(());
                } else {
                    let new_attempts = *attempts_left.read() - 1;
                    attempts_left.set(new_attempts);
                    input.set(String::new());

                    if new_attempts == 0 {
                        trigger_error("App is locked.".to_string());
                    } else {
                        trigger_error(format!("Incorrect. {} attempts left.", new_attempts));
                    }
                }
            }
        }
    };

    let on_input = move |evt: FormEvent| {
        if state() == PinState::EnterPin && *attempts_left.read() == 0 {
            return;
        }

        let val = evt.value();
        // Filter strictly for digits
        let filtered = val.chars()
            .filter(|c| c.is_ascii_digit())
            .take(6)
            .collect::<String>();

        input.set(filtered.clone());

        if filtered.len() == 6 {
            submit();
        }
    };

    // Standard keydown handling
    let on_keydown = move |evt: KeyboardEvent| {
        if evt.key() == Key::Enter && input().len() == 6 {
            submit();
        }
    };

    let title = match state() {
        PinState::EnterPin => "Enter PIN",
        PinState::SetPin => "Set PIN",
        PinState::ConfirmPin => "Confirm PIN",
    };

    let subtitle = match state() {
        PinState::EnterPin => "Enter your six-digit PIN to unlock.",
        PinState::SetPin => "Enter a new six-digit PIN.",
        PinState::ConfirmPin => "Re-enter the PIN to confirm.",
    };

    // Helper to determine what to show in the box (The Masking Logic)
    let get_digit_display = move |index: usize| -> String {
        let current_len = input.read().len();
        if index < current_len {
            return "●".to_string(); 
        }
        String::new()
    };

    // Helper to check if this specific box is "active" (next to receive input)
    let is_active = move |index: usize| -> bool {
        input.read().len() == index
    };
    
    // ----------------------------------------------------------------------
    // THE FIX: Helper to safely construct the class string outside of rsx!
    // ----------------------------------------------------------------------
    let get_pin_box_class = move |i: usize| -> String {
        let mut class = String::from("pin-box");

        // Use is_active helper
        if is_active(i) {
            class.push_str(" active");
        }
        // Check if the box has been filled
        if i < input.read().len() {
            class.push_str(" filled");
        }
        class
    };

    rsx! {
        style { {r#"
            .container {
                height: 100%;
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                color: #e0e0e0;
            }
            .title { font-size: 2.2rem; font-weight: 600; margin-bottom: 0.5rem; }
            .subtitle { font-size: 1.1rem; opacity: 0.7; margin-bottom: 2rem; }
            
            /* WRAPPER: Holds both the boxes and the invisible input */
            .pin-wrapper {
                position: relative;
                width: 320px;
                height: 40px;
                display: flex;
                justify-content: space-between;
                margin-bottom: 1rem;
            }

            /* THE VISUAL BOXES (Underneath) */
            .pin-box {
                width: 45px;
                height: 40px;
                
                /* --- START OF CHANGES FOR FULL BOXES --- */
                border: 2px solid #555; /* Full border instead of just bottom */
                border-radius: 16px;     /* Optional: Adds a slightly rounded corner */
                /* --- END OF CHANGES --- */

                display: flex;
                align-items: center;
                justify-content: center;
                font-size: 1rem;
                background-color: transparent;
                transition: border-color 0.2s, background-color 0.2s; 
                /* Added background-color transition for better focus effect */
            }

            .pin-box.active {
                border-color: #4f4f4fff;
                background-color: #5a5a5aff; /* Add subtle background when active */
            }

            .pin-box.filled {
                border-color: #404040ff;
            }

            /* THE PHANTOM INPUT (On Top) */
            .phantom-input {
                position: absolute;
                top: 0;
                left: 0;
                width: 100%;
                height: 100%;
                opacity: 0; /* Make it invisible */
                cursor: default;
                z-index: 10; 
                font-size: 40px; 
            }

            .error-message { 
                color: #ffbd9eff; 
                font-weight: bold; 
                min-height: 20px;
                text-align: center;
            }
        "#} }

        div { class: "container",
            h1 { class: "title", "{title}" }
            p { class: "subtitle", "{subtitle}" }

          div { class: "pin-wrapper",
                // 1. The Real (Invisible) Input – now with autofocus!
                input {
                    class: "phantom-input",
                    r#type: "text", 
                    inputmode: "numeric",
                    value: "{input}",
                    autofocus: true,  // ← This leverages the new Blitz focus events
                    tabindex: "0",    // For keyboard/tab nav
                    oninput: on_input,
                    onkeydown: on_keydown,
                    maxlength: "6",
                }

                // 2. The Visual Proxy (The 6 Boxes)
                for i in 0..6 {
                    div { 
                        // Call the safe helper function here
                        class: "{get_pin_box_class(i)}",
                        "{get_digit_display(i)}"
                    }
                }
            }
            
        
            p { class: "error-message", 
                if let Some(msg) = error_msg.read().as_ref() { "{msg}" } else { "" }
            }
        }
    }
}