use std::collections::HashMap;
use std::cell::RefCell;
use uuid::Uuid;
use crate::channel::BTCImport;
use zeroize::Zeroize;

thread_local! {
    static BUFFER_STORAGE: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

pub fn get_or_init_buffer_id(create_state: &mut BTCImport) -> String {
    if let Some(id) = create_state.buffer_id.clone() {
        id
    } else {
        let new_id = Uuid::new_v4().to_string();
        create_state.buffer_id = Some(new_id.clone());
        new_id
    }
}

pub fn get_buffer(buffer_id: &str) -> String {
    BUFFER_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .entry(buffer_id.to_string())
            .or_insert(String::new())
            .clone()
    })
}

pub fn update_buffer(buffer_id: &str, value: String) {
    BUFFER_STORAGE.with(|storage| {
        storage.borrow_mut().insert(buffer_id.to_string(), value);
    });
}

pub fn clear_buffer(buffer_id: &str) {
    BUFFER_STORAGE.with(|storage| {
        if let Some(mut value) = storage.borrow_mut().remove(buffer_id) {
            // Zeroize the sensitive data
            value.zeroize();
        }
    });
}