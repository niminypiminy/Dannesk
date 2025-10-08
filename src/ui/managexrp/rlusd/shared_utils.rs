// src/ui/managexrp/rlusd/shared_utils.rs
use std::cell::RefCell;
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use zeroize::Zeroize;

#[derive(Clone, Serialize, Deserialize)]
pub struct TrustlineStateBuffer {
    pub trustline_limit: u64,
    pub passphrase: String,
    pub seed: String, // New field for seed
    pub input_mode: String, // New field for input mode
    pub error: Option<String>,
    pub done: bool,
    pub buffer_id: Option<String>,
}

impl Default for TrustlineStateBuffer {
    fn default() -> Self {
        TrustlineStateBuffer {
            trustline_limit: 0,
            passphrase: String::new(),
            seed: String::new(),
            input_mode: "Passphrase".to_string(), // Default to Passphrase
            error: None,
            done: false,
            buffer_id: None,
        }
    }
}

impl Zeroize for TrustlineStateBuffer {
    fn zeroize(&mut self) {
        self.trustline_limit = 0;
        self.passphrase.zeroize();
        self.seed.zeroize(); // Zeroize seed for security
        self.input_mode.zeroize();
        self.error = None;
        self.done = false;
        self.buffer_id = None;
    }
}

thread_local! {
    pub static BUFFER_STORAGE: RefCell<HashMap<String, TrustlineStateBuffer>> = RefCell::new(HashMap::new());
}

pub fn get_or_init_buffer_id(state: &mut TrustlineStateBuffer) -> String {
    if let Some(id) = state.buffer_id.clone() {
        id
    } else {
        let new_id = Uuid::new_v4().to_string();
        state.buffer_id = Some(new_id.clone());
        new_id
    }
}

pub fn update_buffers(
    buffer_id: &str,
    trustline_limit: u64,
    passphrase: String,
    seed: String, // Add seed parameter
    input_mode: String, // Add input_mode parameter
    error: Option<String>,
    done: bool,
) {
    BUFFER_STORAGE.with(|storage| {
        let mut state = storage
            .borrow()
            .get(buffer_id)
            .cloned()
            .unwrap_or_else(TrustlineStateBuffer::default);
        state.trustline_limit = trustline_limit;
        state.passphrase = passphrase;
        state.seed = seed; // Store seed
        state.input_mode = input_mode; // Store input mode
        state.error = error;
        state.done = done;
        state.buffer_id = Some(buffer_id.to_string());
        storage.borrow_mut().insert(buffer_id.to_string(), state);
    });
}

pub fn clear_buffer(buffer_id: &str) {
    BUFFER_STORAGE.with(|storage| {
        if let Some(mut state) = storage.borrow_mut().remove(buffer_id) {
            state.zeroize(); // Use Zeroize implementation
        }
    });
}