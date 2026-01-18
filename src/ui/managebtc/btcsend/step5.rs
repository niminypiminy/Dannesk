// src/ui/managebtc/btcsend/step5.rs

use dioxus::prelude::*;
use crate::context::{GlobalContext, BtcContext};
use crate::ui::managebtc::btcsend::sendlogic::BTCSendLogic;

fn get_tab_style(active: bool) -> &'static str {
    if active {
        "padding: 0.25rem 1.25rem; background-color: white; color: black; border-radius: 1.5rem; border: none; cursor: pointer; font-size: 0.875rem; font-weight: bold;"
    } else {
        "padding: 0.25rem 1.25rem; background-color: transparent; color: #aaa; border: none; cursor: pointer; font-size: 0.875rem;"
    }
}

#[component]
pub fn view() -> Element {
    let global = use_context::<GlobalContext>();
    let btc_ctx = use_context::<BtcContext>();
    let mut btc_sign_transaction = btc_ctx.btc_sign_transaction;

    let (_, address_opt, _) = btc_ctx.bitcoin_wallet.read().clone();
    let wallet_address = address_opt.unwrap_or_else(|| "No Address".to_string());

    let mut input_mode = use_signal(|| "passphrase".to_string()); 
    let mut passphrase_val = use_signal(|| String::new());
    let mut bip39_val = use_signal(|| String::new());
    let mut seed_words = use_signal(|| vec![String::new(); 24]);

    let current_error = btc_sign_transaction.read()
        .send_transaction.as_ref()
        .and_then(|s| s.error.clone());

    let on_submit = move |_| {
        let mode = input_mode();
        let p_val = passphrase_val(); 
        let s_words = seed_words();   
        let b39 = bip39_val();        
        let wallet_addr_clone = wallet_address.clone();
        let last_v = btc_ctx.btc_modal.read().last_view.clone();

        let mut validation_error: Option<String> = None;

        match mode.as_str() {
            "passphrase" => {
                if p_val.trim().is_empty() {
                    validation_error = Some("Passphrase cannot be empty.".to_string());
                }
            },
            "seed" => {
                let word_count = s_words.iter().filter(|w| !w.trim().is_empty()).count();
                if word_count != 24 {
                    validation_error = Some("Mnemonic must be exactly 24 words.".to_string());
                }
            },
            _ => { validation_error = Some("Invalid input mode.".to_string()); }
        }

        if let Some(err) = validation_error {
             btc_sign_transaction.with_mut(|state| {
                if let Some(ref mut send) = state.send_transaction {
                    send.error = Some(err);
                }
            });
            return;
        }

        let (recipient, amount, asset, fee) = {
            let state = btc_sign_transaction.read();
            let st = state.send_transaction.as_ref().unwrap();
            (st.recipient.clone().unwrap_or_default(), st.amount.clone().unwrap_or_default(), st.asset.clone(), st.fee.clone())
        };
        
        let seed_string = s_words.iter().filter(|w| !w.is_empty()).map(|s| s.as_str()).collect::<Vec<_>>().join(" ");
        
        tokio::spawn(BTCSendLogic::process(
            mode.clone(),
            p_val,
            seed_string,
            b39,
            recipient,
            amount,
            fee,
            wallet_addr_clone,
            asset,
            global.ws_tx.clone(),
            last_v,
        ));

        passphrase_val.set(String::new());
        bip39_val.set(String::new());
        seed_words.set(vec![String::new(); 24]);
        
        btc_sign_transaction.with_mut(|state| {
            if let Some(ref mut send) = state.send_transaction {
                send.error = None; 
            }
        });
    };

    let passphrase_border = use_memo(move || {
        let val = passphrase_val();
        if !val.is_empty() { "1px solid #10B981" } else { "1px solid #444" }
    });

   rsx! {
    div {
        // Added border-box and consistent flex alignment
        style: "display: flex; flex-direction: column; width: 100%; align-items: center; box-sizing: border-box;",

        // Tabs - Matching XRP spacing and border-box
        div {
            style: "display: flex; gap: 1rem; padding: 0.5rem 1rem; background-color: rgba(30, 30, 30, 0.8); border-radius: 2rem; border: 1px solid rgba(255, 255, 255, 0.1); margin-bottom: 2rem; box-sizing: border-box;",
            button {
                style: "{get_tab_style(input_mode() == \"passphrase\")}",
                onclick: move |_| {
                    input_mode.set("passphrase".to_string());
                    seed_words.set(vec![String::new(); 24]);
                    bip39_val.set(String::new());
                },
                "Passphrase"
            }
            button {
                style: "{get_tab_style(input_mode() == \"seed\")}",
                onclick: move |_| {
                    input_mode.set("seed".to_string());
                    passphrase_val.set(String::new());
                },
                "Mnemonic"
            }
        }

        // Main Content Area
        div {
            style: "width: 100%; max-width: 35rem; display: flex; flex-direction: column; align-items: center; box-sizing: border-box;",

            // --- TOP INPUT SLOT ---
            if input_mode() == "seed" {
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(7rem, 1fr)); gap: 0.25rem; width: 100%; max-width: 31.5rem; margin-bottom: 1.5rem; box-sizing: border-box;",
                    for i in 0..24 {
                        div {
                            key: "{i}",
                            input {
                                // Updated to XRP 2rem height and centered white text
                                style: "display: block; box-sizing: border-box; width: 100%; height: 2rem; padding: 0.3125rem; background-color: transparent; border: 1px solid #444; border-radius: 0.25rem; font-size: 0.875rem; text-align: center; color: white;",
                                value: "{seed_words()[i]}",
                                oninput: move |evt: Event<FormData>| {
                                    let val = evt.value();
                                    if val.trim().contains(' ') {
                                        let words: Vec<String> = val.split_whitespace().map(|s| s.to_string()).collect();
                                        let mut current = seed_words.peek().clone();
                                        for (j, word) in words.iter().enumerate() {
                                            if i + j < 24 { current[i + j] = word.clone(); }
                                        }
                                        seed_words.set(current);
                                    } else {
                                        let mut current = seed_words.peek().clone();
                                        current[i] = val.trim().to_string();
                                        seed_words.set(current);
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                div { style: "width: 100%; max-width: 30rem; margin-bottom: 1rem; box-sizing: border-box;",
                    div { 
                        style: "display: flex; align-items: center; justify-content: space-between;",
                        label { style: "font-size: 0.875rem; margin-bottom: 0.5rem; color: #aaa;", "BIP39 Passphrase (25th Word)" }
                        span { style: "font-size: 0.7rem; color: #666;", "Optional" }
                    }
                    input {
                        // Updated to XRP 3rem height and 1.5rem font size
                        style: "box-sizing: border-box; width: 100%; height: 3rem; padding: 0.3125rem; background-color: transparent; border: 1px solid #444; border-radius: 0.25rem; font-size: 1.5rem; color: white;",
                        value: "{bip39_val()}",
                        oninput: move |e| bip39_val.set(e.value())
                    }
                }
            }

            // --- BOTTOM INPUT SLOT ---
            if input_mode() == "passphrase" {
                div { style: "width: 100%; max-width: 30rem; margin-bottom: 1.5rem; box-sizing: border-box;",
                    label { style: "font-size: 0.875rem; margin-bottom: 0.5rem; display: block; color: #aaa;", "Encryption Passphrase" }
                    input {
                        style: "box-sizing: border-box; width: 100%; height: 3rem; padding: 0.3125rem; background-color: transparent; border: {passphrase_border}; border-radius: 0.25rem; font-size: 1.5rem; color: white;",
                        value: "{passphrase_val()}",
                        oninput: move |e| passphrase_val.set(e.value())
                    }
                }
            } else {
                div { style: "width: 100%; max-width: 30rem; margin-bottom: 1.5rem; box-sizing: border-box;",
                    div { 
                        style: "display: flex; align-items: center; justify-content: space-between;",
                        label { style: "font-size: 0.875rem; margin-bottom: 0.5rem; color: #aaa;", "BIP39 Passphrase (25th Word)" }
                        span { style: "font-size: 0.7rem; color: #666;", "Optional" }
                    }
                    input {
                        style: "box-sizing: border-box; width: 100%; height: 3rem; padding: 0.3125rem; background-color: transparent; border: 1px solid #444; border-radius: 0.25rem; font-size: 1.5rem; color: white;",
                        value: "{bip39_val()}",
                        oninput: move |e| bip39_val.set(e.value())
                    }
                }
            }

            // --- ERROR MESSAGE ---
            if let Some(err) = current_error {
                div { style: "color: #ff4d4d; margin-bottom: 1rem; font-size: 0.875rem; font-weight: bold; text-align: center; max-width: 90%;", "{err}" }
            }

            // --- ACTION BUTTON ---
            button {
                style: "width: 8.75rem; height: 2.25rem; background-color: #333; color: white; border: none; border-radius: 1.375rem; font-size: 1rem; display: flex; cursor: pointer; justify-content: center; align-items: center; margin-top: 0.5rem;",
                onclick: on_submit,
                "Sign & Send"
            }
        }
    }
}
}