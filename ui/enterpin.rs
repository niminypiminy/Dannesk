// src/ui/pin.rs
use dioxus_native::prelude::*;
use crate::utils::json_storage;

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
    let mut is_processing = use_signal(|| false);

    let pin_exists = use_memo(|| json_storage::read_json::<crate::pin::PinData>("pin.json").is_ok());

    let state = use_memo(move || {
        if !pin_exists() {
            if stored_pin_for_confirmation().is_empty() { PinState::SetPin } else { PinState::ConfirmPin }
        } else {
            PinState::EnterPin
        }
    });

    let mut run_submit = move || {
        if *is_processing.read() || input.read().is_empty() { return; }
        is_processing.set(true);

        let pin = input.read().clone();
        let current_state = state();
        let stored_pin = stored_pin_for_confirmation.read().clone();

        spawn(async move {
            match current_state {
                PinState::SetPin => {
                    stored_pin_for_confirmation.set(pin);
                    input.set(String::new());
                    error_msg.set(None);
                }
                PinState::ConfirmPin => {
                    if pin == stored_pin {
                        if crate::pin::set_pin(&pin).is_ok() {
                            on_unlock.call(());
                        } else {
                            error_msg.set(Some("ERR: STORAGE_IO_FAILURE".to_string()));
                            input.set(String::new());
                        }
                    } else {
                        error_msg.set(Some("ERR: BUFFER_MISMATCH".to_string()));
                        stored_pin_for_confirmation.set(String::new());
                        input.set(String::new());
                    }
                }
                PinState::EnterPin => {
                    if crate::pin::verify_pin(&pin).is_ok() {
                        on_unlock.call(());
                    } else {
                        let left = *attempts_left.peek() - 1;
                        attempts_left.set(left);
                        input.set(String::new());
                        error_msg.set(Some(if left == 0 { 
                            "CRITICAL: SYSTEM_LOCKOUT".into() 
                        } else { 
                            format!("AUTH_ERR{} ATTEMPTS_REMAINING", left) 
                        }));
                    }
                }
            }
            is_processing.set(false);
        });
    };

    let mut add_digit = move |digit: char| {
        if input.read().len() < 6 && !*is_processing.read() {
            input.with_mut(|s| s.push(digit));
            if input.read().len() == 6 {
                run_submit();
            }
        }
    };

    let title = match state() {
        PinState::EnterPin => "SECURE_GATEWAY_v1.0",
        PinState::SetPin => "PROTOCOL_INITIALIZATION",
        PinState::ConfirmPin => "VERIFICATION_SEQUENCE",
    };

    rsx! {
        style { {r#"
            .pin-page {
                height: 100vh;
                display: flex;
                align-items: center;
                justify-content: center;
                background: var(--bg-primary);
                font-family: 'JetBrains Mono', monospace;
                color: var(--text);
            }

            .terminal-frame {
                display: flex;
                flex-direction: column;
                border: 1px solid var(--border);
                background: #0a0a0a;
                padding: 2.5rem;
                position: relative;
                min-width: 650px;
            }

            .terminal-frame::before {
                content: "SYS_AUTH_ID: 0x88AF";
                position: absolute;
                top: -10px; left: 10px;
                background: #0a0a0a;
                padding: 0 5px;
                font-size: 0.55rem;
                color: var(--accent);
            }

            .main-layout {
                display: flex;
                align-items: center;
                justify-content: space-between;
                gap: 3rem;
            }

            .status-panel {
                flex: 1;
                display: flex;
                flex-direction: column;
            }

            .header-label {
                font-size: 0.65rem;
                color: var(--accent);
                margin-bottom: 0.5rem;
                letter-spacing: 2px;
            }

            .title-text {
                font-size: 1.1rem;
                font-weight: bold;
                margin-bottom: 2rem;
                color: var(--text);
            }

            .terminal-input-wrapper {
                display: flex;
                align-items: center;
                background: var(--bg-grid);
                border: 1px solid var(--border);
                padding: 1rem;
                margin-bottom: 1.5rem;
            }

            .bracket { color: var(--text-secondary); opacity: 0.4; font-weight: bold; font-size: 1.2rem; }
            
            .pin-field {
                flex: 1;
                background: transparent;
                border: none;
                outline: none;
                color: var(--accent);
                font-family: inherit;
                font-size: 1.5rem;
                letter-spacing: 0.8rem;
                text-align: center;
                padding: 0 1rem;
            }

            .status-msg {
                font-size: 0.65rem;
                color: var(--status-warn);
                min-height: 1.5rem;
            }

            .keypad {
                display: grid;
                grid-template-columns: repeat(3, 1fr);
                gap: 6px;
                width: 260px;
            }

            .num-key {
                /* FIX: Use Flex for perfect centering */
                display: flex;
                align-items: center;
                justify-content: center;
                
                background: rgba(255, 255, 255, 0.03);
                border: 1px solid var(--border);
                color: var(--text);
                height: 60px; /* Fixed height for symmetry */
                font-size: 1.2rem;
                cursor: pointer;
                font-family: inherit;
                transition: background 0.1s ease;
            }

            .num-key:hover {
                background: var(--accent);
                color: #000;
                border-color: var(--accent);
            }

            .special-key { 
                font-size: 0.7rem; 
                color: var(--text-secondary); 
            }
        "#} }

        div { class: "pin-page",
            div { class: "terminal-frame",
                div { class: "main-layout",
                    div { class: "status-panel",
                        div { class: "header-label", "TERMINAL_STATUS: AUTH_REQUIRED" }
                        div { class: "title-text", "{title}" }

                        div { class: "terminal-input-wrapper",
                            span { class: "bracket", "[" }
                            input {
                                class: "pin-field",
                                r#type: "password",
                                autofocus: true,
                                maxlength: "6",
                                value: "{input}",
                                oninput: move |evt| {
                                    let val = evt.value();
                                    if val.len() <= 6 && val.chars().all(|c| c.is_numeric()) {
                                        input.set(val.clone());
                                        if val.len() == 6 { run_submit(); }
                                    }
                                }
                            }
                            span { class: "bracket", "]" }
                        }

                        div { class: "status-msg", "{error_msg.read().clone().unwrap_or_default()}" }
                        
                        div { 
                            style: "margin-top: 1rem; font-size: 0.55rem; color: #444; text-transform: uppercase;",
                            "PROTOCOL_READY"
                            br {}
                            "SIGNAL_STRENGTH: GOOD"
                        }
                    }

                    div { class: "keypad",
                        for n in ["1", "2", "3", "4", "5", "6", "7", "8", "9"] {
                            button { 
                                class: "num-key", 
                                onclick: move |_| add_digit(n.chars().next().unwrap()), 
                                "{n}" 
                            }
                        }
                        button { 
                            class: "num-key special-key", 
                            onclick: move |_| input.set(String::new()), 
                            "CLR" 
                        }
                        button { 
                            class: "num-key", 
                            onclick: move |_| add_digit('0'), 
                            "0" 
                        }
                        button { 
                            class: "num-key special-key", 
                            onclick: move |_| { input.with_mut(|s| { s.pop(); }); }, 
                            "DEL" 
                        }
                    }
                }
            }
        }
    }
}