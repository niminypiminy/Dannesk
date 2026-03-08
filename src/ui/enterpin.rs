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
            // Short breath to ensure the 6th digit renders
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
            
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
                            format!("AUTH_ERR: {} ATTEMPTS REMAINING", left) 
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

    rsx! {
        style { {r#"
            .pin-page {
                height: 100vh;
                display: flex;
                align-items: center;
                justify-content: center;
                background: #050505;
                font-family: 'JetBrains Mono', monospace;
                color: var(--text);
            }

            .terminal-frame {
                display: flex;
                flex-direction: column;
                border: 1px solid var(--border);
                background: #0a0a0a;
                padding: 2rem;
                width: 320px;
            }

            .terminal-header {
                font-size: 0.7rem;
                color: var(--accent);
                letter-spacing: 2px;
                margin-bottom: 2rem;
                border-bottom: 1px solid rgba(255,255,255,0.1);
                padding-bottom: 0.5rem;
                text-align: center;
            }

            .main-layout {
                display: flex;
                flex-direction: column;
                align-items: center;
                gap: 1.5rem;
            }

            .status-panel {
                width: 100%;
                display: flex;
                flex-direction: column;
                align-items: center;
                min-height: 80px;
            }

            .terminal-input-wrapper {
                display: flex;
                align-items: center;
                background: rgba(255,255,255,0.02);
                border: 1px solid var(--border);
                padding: 0.8rem;
                width: 100%;
                box-sizing: border-box;
                height: 54px;
            }

            .loading-text {
                flex: 1;
                text-align: center;
                font-size: 0.8rem;
                color: var(--accent);
                animation: pulse 1.5s infinite;
            }

            @keyframes pulse {
                0% { opacity: 1; }
                50% { opacity: 0.4; }
                100% { opacity: 1; }
            }

            .bracket { color: var(--accent); opacity: 0.5; font-size: 1.2rem; }
            
            .pin-field {
                flex: 1;
                background: transparent;
                border: none;
                outline: none;
                color: var(--text);
                font-family: inherit;
                font-size: 1.4rem;
                letter-spacing: 0.5rem;
                text-align: center;
            }

            .status-msg {
                margin-top: 0.5rem;
                font-size: 0.6rem;
                color: var(--status-warn);
                min-height: 1rem;
                text-align: center;
            }

            .keypad {
                display: grid;
                grid-template-columns: repeat(3, 1fr);
                gap: 8px;
                width: 100%;
                transition: opacity 0.3s ease;
            }

            .keypad.processing {
                opacity: 0.3;
                pointer-events: none;
            }

            .num-key {
                display: flex;
                align-items: center;
                justify-content: center;
                background: rgba(255, 255, 255, 0.03);
                border: 1px solid var(--border);
                color: var(--text);
                height: 55px;
                font-size: 1.1rem;
                cursor: pointer;
                transition: all 0.1s ease;
            }

            .num-key:active {
                background: var(--accent);
                color: #000;
            }

            .special-key { 
                font-size: 0.65rem; 
                opacity: 0.6;
            }
        "#} }

        div { class: "pin-page",
            div { class: "terminal-frame",
                div { class: "terminal-header", "SECURE_GATEWAY_v0.3" }

                div { class: "main-layout",
                    div { class: "status-panel",
                        div { class: "terminal-input-wrapper",
                            span { class: "bracket", ">>" }
                            if *is_processing.read() {
                                span { class: "loading-text", "DECRYPTING_HASH..." }
                            } else {
                                input {
                                    class: "pin-field",
                                    r#type: "password",
                                    autofocus: true,
                                    value: "{input}",
                                    oninput: move |evt| {
                                        let val = evt.value();
                                        if val.len() <= 6 && val.chars().all(|c| c.is_numeric()) {
                                            input.set(val.clone());
                                            if val.len() == 6 { run_submit(); }
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "status-msg", "{error_msg.read().clone().unwrap_or_default()}" }
                    }

                    // Keypad dims and disables during Argon2 hashing
                    div { 
                        class: if *is_processing.read() { "keypad processing" } else { "keypad" },
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