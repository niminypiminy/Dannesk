// src/ui/managexrp/euro/modify/buffer_manager.rs
use crate::ui::managexrp::euro::shared_utils::{update_buffers, TrustlineStateBuffer};

#[derive(Clone, PartialEq, Debug)]
pub enum InputMode {
    Passphrase,
    Seed,
}

impl InputMode {
    pub fn to_string(&self) -> String {
        match self {
            InputMode::Passphrase => "Passphrase".to_string(),
            InputMode::Seed => "Seed".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "Seed" => InputMode::Seed,
            _ => InputMode::Passphrase,
        }
    }
}

pub struct BufferManager {
    buffer_id: String,
    trustline_limit: u64,
    passphrase: String,
    seed: String,
    input_mode: String,
    error: Option<String>,
    done: bool,
}

impl BufferManager {


    pub fn from_state_buffer(buffer_id: &str, state: TrustlineStateBuffer) -> Self {
        BufferManager {
            buffer_id: buffer_id.to_string(),
            trustline_limit: state.trustline_limit,
            passphrase: state.passphrase,
            seed: state.seed,
            input_mode: state.input_mode,
            error: state.error,
            done: state.done,
        }
    }

    pub fn to_state_buffer(&self) -> TrustlineStateBuffer {
        TrustlineStateBuffer {
            trustline_limit: self.trustline_limit,
            passphrase: self.passphrase.clone(),
            seed: self.seed.clone(),
            input_mode: self.input_mode.clone(),
            error: self.error.clone(),
            done: self.done,
            buffer_id: Some(self.buffer_id.clone()),
        }
    }

    pub fn update_buffers(&mut self) {
        update_buffers(
            &self.buffer_id,
            self.trustline_limit,
            self.passphrase.clone(),
            self.seed.clone(),
            self.input_mode.clone(),
            self.error.clone(),
            self.done,
        );
    }

    pub fn set_input_mode(&mut self, mode: InputMode) {
        match mode {
            InputMode::Passphrase => {
                self.seed.clear();
                self.input_mode = InputMode::Passphrase.to_string();
            }
            InputMode::Seed => {
                self.passphrase.clear();
                self.input_mode = InputMode::Seed.to_string();
            }
        }
        self.update_buffers();
    }

    pub fn update_trustline_limit(&mut self, limit: u64) {
        self.trustline_limit = limit;
        self.update_buffers();
    }

    pub fn update_passphrase(&mut self, passphrase: &str) {
        self.passphrase = passphrase.to_string();
        self.update_buffers();
    }

    pub fn update_seed(&mut self, seed: &str) {
        self.seed = seed.to_string();
        self.update_buffers();
    }

    pub fn set_error(&mut self, error: Option<String>) {
        self.error = error;
        self.update_buffers();
    }

    pub fn set_done(&mut self, done: bool) {
        self.done = done;
        self.update_buffers();
    }

    // Getters
    pub fn trustline_limit(&self) -> u64 {
        self.trustline_limit
    }

    pub fn passphrase(&self) -> &str {
        &self.passphrase
    }

    pub fn seed(&self) -> &str {
        &self.seed
    }

    pub fn input_mode(&self) -> &str {
        &self.input_mode
    }

    pub fn error(&self) -> Option<&String> {
        self.error.as_ref()
    }

    pub fn done(&self) -> bool {
        self.done
    }
}