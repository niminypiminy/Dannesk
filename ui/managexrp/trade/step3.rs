use dioxus_native::prelude::*;
use crate::context::{GlobalContext, XrpContext};
use crate::ui::managexrp::trade::tradelogic::TradeLogic;
use crate::utils::send_auth_layout::SendAuthForm;

#[component]
pub fn view() -> Element {
    let global = use_context::<GlobalContext>();
    let xrp_ctx = use_context::<XrpContext>();
    let mut trade_ctx = xrp_ctx.trade;

    let (_, address_opt, _) = xrp_ctx.wallet_balance.read().clone();
    let wallet_address = address_opt.unwrap_or_else(|| "NULL_ADDR".to_string());

    let input_mode = use_signal(|| "passphrase".to_string());
    let passphrase_val = use_signal(|| String::new());
    let bip39_val = use_signal(|| String::new());
    let seed_words = use_signal(|| vec![String::new(); 24]);

    let current_error = trade_ctx.read()
        .send_trade.as_ref()
        .and_then(|s| s.error.clone());

    let on_submit = move |_| {
        let mode = input_mode();
        let p_val = passphrase_val();
        let s_words = seed_words.read();
        let b39 = bip39_val();
        let last_v = xrp_ctx.xrp_modal.read().last_view.clone();

        let mut validation_error: Option<String> = None;

        // 1. Credential Validation (Auth)
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
            _ => { validation_error = Some("ERR: INVALID_AUTH_MODE".to_string()); }
        }

        // 2. Trade Data Gathering
        let (base, quote, amount, price, flags) = {
            let state = trade_ctx.read();
            if let Some(st) = state.send_trade.as_ref() {
                (
                    st.base_asset.clone().unwrap_or_default(),
                    st.quote_asset.clone().unwrap_or_default(),
                    st.amount.clone().unwrap_or_default(),
                    st.limit_price.clone().unwrap_or_default(),
                    st.flags.clone().unwrap_or_default(),
                )
            } else {
                Default::default()
            }
        };

        // 3. Logic Validation (Trade Specific)
        if validation_error.is_none() {
            if base == quote {
                 validation_error = Some("ERR: ASSET_PAIR_IDENTITY_CONFLICT".to_string());
            } else if amount.parse::<f64>().unwrap_or(0.0) <= 0.0 {
                 validation_error = Some("ERR: INVALID_QUANTITY".to_string());
            } else if price.parse::<f64>().unwrap_or(0.0) <= 0.0 {
                 validation_error = Some("ERR: INVALID_LIMIT_PRICE".to_string());
            }
        }

        if let Some(err) = validation_error {
            trade_ctx.with_mut(|state| {
                if let Some(ref mut send) = state.send_trade {
                    send.error = Some(err);
                }
            });
            return;
        }

        let seed_string = s_words.iter().filter(|w| !w.is_empty()).cloned().collect::<Vec<_>>().join(" ");

        // 4. Execution
        tokio::spawn(TradeLogic::process(
            mode,
            p_val,
            seed_string,
            b39,
            base,
            quote,
            amount,
            price,
            flags,
            wallet_address.clone(),
            global.ws_tx.clone(),
            last_v,
        ));
    };

    rsx! {
        SendAuthForm {
            step_title: "TRANSACTION_AUTHORIZATION // STEP_03".to_string(),
            network_label: "XRPL_MAINNET".to_string(),
            input_mode,
            passphrase_val,
            seed_words,
            bip39_val,
            current_error,
            on_submit,
            on_clear_error: move |_| {
                trade_ctx.with_mut(|s| {
                    if let Some(ref mut t) = s.send_trade { 
                        t.error = None; 
                    }
                });
            }
        }
    }
}