// src/ui/managexrp/rlusd/send/buffers.rs
use std::cell::RefCell;
use std::collections::HashMap;
use zeroize::Zeroize;

thread_local! {
    pub static BUFFER_STORAGE: RefCell<HashMap<String, (String, String, String, String, String)>> = RefCell::new(HashMap::new());
}

pub fn get_buffers(buffer_id: &str) -> (String, String, String, String, String) {
    BUFFER_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .entry(buffer_id.to_string())
            .or_insert((
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                "Passphrase".to_string(), // Default input_mode to "Passphrase"
            ))
            .clone()
    })
}

pub fn update_buffers(
    buffer_id: &str,
    address: String,
    usd_amount: String,
    passphrase: String,
    seed: String,
    input_mode: String,
) {
    BUFFER_STORAGE.with(|storage| {
        storage.borrow_mut().insert(
            buffer_id.to_string(),
            (address, usd_amount, passphrase, seed, input_mode),
        );
    });
}

pub fn clear_buffer(buffer_id: &str) {
    BUFFER_STORAGE.with(|storage| {
        if let Some((mut address, mut usd_amount, mut passphrase, mut seed, mut input_mode)) =
            storage.borrow_mut().remove(buffer_id)
        {
            // Zero out all buffers for extra security
            address.zeroize();
            usd_amount.zeroize();
            passphrase.zeroize();
            seed.zeroize();
            input_mode.zeroize();
        }
    });
}