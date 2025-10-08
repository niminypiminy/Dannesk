// src/ui/managexrp/btxsend/buffers.rs

use std::collections::HashMap;
use std::cell::RefCell;
use zeroize::Zeroize;

thread_local! {
    pub static BUFFER_STORAGE: RefCell<HashMap<String, (String, String, String, String, String, [String; 24], String)>> = RefCell::new(HashMap::new());
}

pub fn get_buffers(buffer_id: &str) -> (String, String, String, String, String, [String; 24], String) {
    BUFFER_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .entry(buffer_id.to_string())
            .or_insert((
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                vec![String::new(); 24].try_into().unwrap(), // Fix Copy trait error
                "Passphrase".to_string(),
            ))
            .clone()
    })
}
pub fn update_buffers(
    buffer_id: &str,
    address: String,
    btc_amount: String,
    usd_amount: String,
    passphrase: String,
    custom_fee: String,
    seed_words: [String; 24],
    input_mode: String,
) {
    BUFFER_STORAGE.with(|storage| {
        storage.borrow_mut().insert(
            buffer_id.to_string(),
            (address, btc_amount, usd_amount, passphrase, custom_fee, seed_words, input_mode),
        );
    });
}

pub fn clear_buffer(buffer_id: &str) {
    BUFFER_STORAGE.with(|storage| {
        if let Some((mut address, mut btc_amount, mut usd_amount, mut passphrase, mut custom_fee, mut seed_words, mut input_mode)) =
            storage.borrow_mut().remove(buffer_id)
        {
            // Zero out all string buffers for extra security
            address.zeroize();
            btc_amount.zeroize();
            usd_amount.zeroize();
            passphrase.zeroize();
            custom_fee.zeroize();
            for word in seed_words.iter_mut() {
                word.zeroize();
            }
            input_mode.zeroize();
        }
    });
}