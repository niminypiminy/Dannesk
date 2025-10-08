use std::collections::HashMap;
use std::cell::RefCell;
use uuid::Uuid;
use zeroize::Zeroize;
use crate::channel::XRPImport;

thread_local! {
    static BUFFER_STORAGE: RefCell<HashMap<String, (String, String)>> = RefCell::new(HashMap::new());
}

pub fn get_or_init_buffer_id(import_state: &mut XRPImport) -> String {
    if let Some(id) = import_state.buffer_id.clone() {
        id
    } else {
        let new_id = Uuid::new_v4().to_string();
        import_state.buffer_id = Some(new_id.clone());
        new_id
    }
}

pub fn get_buffer(buffer_id: &str) -> (String, String) {
    BUFFER_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .entry(buffer_id.to_string())
            .or_insert((String::new(), String::new()))
            .clone()
    })
}

pub fn update_buffer(buffer_id: &str, seed_buffer: String, passphrase_buffer: String) {
    BUFFER_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .insert(buffer_id.to_string(), (seed_buffer, passphrase_buffer));
    });
}

pub fn clear_buffer(buffer_id: &str) {
    BUFFER_STORAGE.with(|storage| {
        if let Some((mut seed, mut passphrase)) = storage.borrow_mut().remove(buffer_id) {
            // Zeroize the sensitive data
            seed.zeroize();
            passphrase.zeroize();
        }
    });
}