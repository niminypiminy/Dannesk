use dioxus::prelude::*;
use crate::context::BtcContext;
use crate::channel::BTCWalletProcessState;
use zeroize::{Zeroizing};

#[component]
pub fn view() -> Element {
    let btc_ctx = use_context::<BtcContext>();
    let mut btc_wallet_process = btc_ctx.btc_wallet_process;
    
    let mut seed_words = use_signal(|| vec![String::new(); 24]);
    let mut error_msg = use_signal(|| None::<String>);

    let on_continue = move |_| {
        let current_words = seed_words();
        let word_count = current_words.iter().filter(|w| !w.is_empty()).count();

        if word_count != 24 {
            error_msg.set(Some("Mnemonic must be exactly 24 words.".to_string()));
            return;
        }

let seed_phrase = Zeroizing::new(current_words.join(" "));

        btc_wallet_process.with_mut(|state: &mut BTCWalletProcessState | {
            if let Some(ref mut import) = state.import_wallet {
                import.seed = Some(seed_phrase);
                import.error = None;
                import.step = 2; 
            }
        });
    };

    rsx! {
        div {
            style: "
                display: flex; 
                flex-direction: column; 
                width: 100%; 
                align-items: center;",
            
            div { 
                style: "font-size: 1.5rem; margin: 0; margin-bottom: 0.5rem;",
                "Enter your 24-word mnemonic phrase:" 
            }
            div { 
                style: "font-size: 1rem; color: #888; margin-bottom: 1.5rem;",
                "For security reasons, only 24-word mnemonics are supported."
            }

            div {
                style: "
                    display: grid; 
                    grid-template-columns: repeat(auto-fit, minmax(7rem, 1fr)); 
                    gap: 0.25rem;
                    width: 100%; 
                    max-width: 31.5rem;
                    margin-left: auto;
                    margin-right: auto;
                ", 
                
               for i in 0..24 {
    div {
        key: "{i}",
        input {
            style: "
                display: block;
                box-sizing: border-box;
                width: 100%; 
                height: 2rem; 
                padding: 0.3125rem; 
                background-color: transparent; 
                border: 1px solid #444; 
                border-radius: 0.25rem;
                font-size: 0.875rem;
            ",
            value: "{seed_words.read()[i]}", // Using .read() for better Blitz reactivity
            spellcheck: false,
            oninput: move |evt: Event<FormData>| {
                // 1. CLIP: Remove newlines/carriage returns to prevent vertical layout breaks
                let val = evt.value().replace(['\n', '\r'], " ");
                error_msg.set(None);

                if val.trim().contains(' ') {
                    // Multi-word paste logic
                    let words: Vec<String> = val
                        .split_whitespace()
                        .map(|s| s.to_string())
                        .collect();
                    
                    let mut current_words = seed_words.peek().clone();
                    for (j, word) in words.iter().enumerate() {
                        if i + j < 24 {
                            // Ensure the individual words are trimmed of reckless whitespace
                            current_words[i + j] = word.trim().to_string();
                        }
                    }
                    seed_words.set(current_words);
                } else {
                    // Single word entry
                    let mut current_words = seed_words.peek().clone();
                    current_words[i] = val.trim().to_string();
                    seed_words.set(current_words);
                }
            }
                        }
                    }
                }
            }

            if let Some(err) = error_msg() {
                div {
                    style: "color: #ff4d4d; margin-top: 1rem; font-size: 1rem; font-weight: bold;",
                    "{err}"
                }
            }

            div { style: "height: 2rem;" } 

            button {
                style: "
                    width: 8.75rem; height: 2.25rem; background-color: #333; color: white; border: none; 
        border-radius: 1.375rem; font-size: 1rem; display: flex; cursor: pointer; justify-content: center; align-items: center; margin-top: 1rem;
                ",
                onclick: on_continue,
                "Continue"
            }
        }
    }
}