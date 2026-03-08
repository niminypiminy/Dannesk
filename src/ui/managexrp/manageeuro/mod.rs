//src/ui/managexrp/manageeuro/mod.rs
//dependent upon src/utils/enable_layout.rs 

use dioxus_native::prelude::*;
use crate::context::{EuroContext, GlobalContext, XrpContext};
pub mod enable_logic;
use enable_logic::EuroEnableLogic;
pub mod eurobalance;
use crate::utils::enable_token_layout::render_token_enable;   

#[component]
pub fn render_manage_euro() -> Element {
    let global = use_context::<GlobalContext>();
    let euro_ctx = use_context::<EuroContext>();
    let xrp_ctx = use_context::<XrpContext>();
    
    let (_, address_opt, _) = xrp_ctx.wallet_balance.read().clone();
    let wallet_address = address_opt.unwrap_or_else(|| "NULL_ADDR".to_string());
    let wallet_addr_for_ui = wallet_address.clone(); 

    let (_, has_euro, _) = euro_ctx.euro.read().clone();

    // State management (Signals)
    let input_mode = use_signal(|| "passphrase".to_string()); 
    let mut passphrase_val = use_signal(|| String::new());
    let mut bip39_val = use_signal(|| String::new());
    let mut seed_words = use_signal(|| vec![String::new(); 24]);
    let mut error_msg = use_signal(|| None::<String>);

    let on_submit = move |_| {
        let mode = input_mode();
        let p_val = passphrase_val(); 
        let s_words = seed_words();   
        let b39 = bip39_val();        
        let wallet_addr_clone = wallet_address.clone();
        
        let mut validation_error: Option<String> = None;

        match mode.as_str() {
            "passphrase" => {
                if p_val.trim().is_empty() {
                    validation_error = Some("ERR: PASSPHRASE_REQUIRED".to_string());
                }
            },
            "seed" => {
                let word_count = s_words.iter().filter(|w| !w.trim().is_empty()).count();
                if word_count != 24 {
                    validation_error = Some("ERR: MNEMONIC_LENGTH_MISMATCH".to_string());
                }
            },
            _ => { validation_error = Some("ERR: INVALID_MODE".to_string()); }
        }

        if let Some(err) = validation_error {
            error_msg.set(Some(err));
            return;
        }
        
        error_msg.set(None);
        let seed_string = s_words.iter().filter(|w| !w.is_empty()).map(|s| s.as_str()).collect::<Vec<_>>().join(" ");
        
        tokio::spawn(EuroEnableLogic::process(
            mode.clone(),
            p_val,
            seed_string,
            b39,
            wallet_addr_clone,
            "EUROP".to_string(),
            global.ws_tx.clone(),
        ));

        // Reset inputs
        passphrase_val.set(String::new());
        bip39_val.set(String::new());
        seed_words.set(vec![String::new(); 24]);
    };

    let reserve_info = format!("REQ_RESERVE: 0.20 XRP // ADDR: {}", wallet_addr_for_ui);

    rsx! {
        render_token_enable {
            symbol: "EUROP".to_string(),
            reserve_info,
            enable_btn_text: "ENABLE_EUROP".to_string(),
            has_token: has_euro,
            input_mode,
            passphrase_val,
            bip39_val,
            seed_words,
            error_msg,
            on_enable: on_submit,
            
            // This is the "children" content shown when has_euro is true
            eurobalance::render_euro_balance {}
        }
    }
}