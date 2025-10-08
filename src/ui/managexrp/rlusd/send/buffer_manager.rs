// src/ui/managexrp/rlusd/buffer_manager.rs
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
    address_buffer: String,
    usd_amount_buffer: String,
    passphrase_buffer: String,
    seed_buffer: String,
    input_mode: String,
}

impl BufferManager {
    pub fn new(
        buffer_id: &str,
        address_buffer: String,
        usd_amount_buffer: String,
        passphrase_buffer: String,
        seed_buffer: String,
        input_mode: String,
    ) -> Self {
        BufferManager {
            buffer_id: buffer_id.to_string(),
            address_buffer,
            usd_amount_buffer,
            passphrase_buffer,
            seed_buffer,
            input_mode,
        }
    }

    pub fn update_buffers(&mut self) {
        super::buffers::update_buffers(
            &self.buffer_id,
            self.address_buffer.clone(),
            self.usd_amount_buffer.clone(),
            self.passphrase_buffer.clone(),
            self.seed_buffer.clone(),
            self.input_mode.clone(),
        );
    }

    pub fn set_input_mode(&mut self, mode: InputMode) {
        match mode {
            InputMode::Passphrase => {
                self.seed_buffer.clear();
                self.input_mode = InputMode::Passphrase.to_string();
            }
            InputMode::Seed => {
                self.passphrase_buffer.clear();
                self.input_mode = InputMode::Seed.to_string();
            }
        }
        self.update_buffers();
    }

    pub fn update_address(&mut self, address: &str) {
        self.address_buffer = address.to_string();
        self.update_buffers();
    }

    pub fn update_usd_amount(&mut self, usd_amount: &str) {
        self.usd_amount_buffer = usd_amount.to_string();
        self.update_buffers();
    }

    pub fn update_passphrase(&mut self, passphrase: &str) {
        self.passphrase_buffer = passphrase.to_string();
        self.update_buffers();
    }

    pub fn update_seed(&mut self, seed: &str) {
        self.seed_buffer = seed.to_string();
        self.update_buffers();
    }

    // Getters
    pub fn address_buffer(&self) -> &str {
        &self.address_buffer
    }

    pub fn usd_amount_buffer(&self) -> &str {
        &self.usd_amount_buffer
    }

    pub fn passphrase_buffer(&self) -> &str {
        &self.passphrase_buffer
    }

    pub fn seed_buffer(&self) -> &str {
        &self.seed_buffer
    }

    pub fn input_mode(&self) -> &str {
        &self.input_mode
    }
}