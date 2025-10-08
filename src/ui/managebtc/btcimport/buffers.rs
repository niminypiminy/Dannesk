use std::collections::HashMap;
use std::cell::RefCell;
use uuid::Uuid;
use zeroize::Zeroize;
use crate::channel::BTCImport;

thread_local! {
    static BUFFER_STORAGE: RefCell<HashMap<String, ([String; 24], String)>> = RefCell::new(HashMap::new());
}

pub fn get_or_init_buffer_id(import_state: &mut BTCImport) -> String {
    if let Some(id) = import_state.buffer_id.clone() {
        id
    } else {
        let new_id = Uuid::new_v4().to_string();
        import_state.buffer_id = Some(new_id.clone());
        new_id
    }
}

pub fn get_buffer(buffer_id: &str) -> ([String; 24], String) {
    BUFFER_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .entry(buffer_id.to_string())
            .or_insert((Default::default(), String::new()))
            .clone()
    })
}

pub fn update_buffer(buffer_id: &str, seed_words: [String; 24], passphrase_buffer: String) {
    BUFFER_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .insert(buffer_id.to_string(), (seed_words, passphrase_buffer));
    });
}

pub fn clear_buffer(buffer_id: &str) {
    BUFFER_STORAGE.with(|storage| {
        if let Some((mut seed_words, mut passphrase)) = storage.borrow_mut().remove(buffer_id) {
            // Zeroize the sensitive data
            for word in seed_words.iter_mut() {
                word.zeroize();
            }
            passphrase.zeroize();
        }
    });
}