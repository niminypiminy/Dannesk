use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::cell::RefCell;
use zeroize::Zeroize;

#[derive(Clone, Serialize, Deserialize)]
pub struct TradeState {
    pub step: u8,
    pub done: bool,
    pub base_asset: String,
    pub quote_asset: String,
    pub amount: String,
    pub limit_price: String,
    pub flags: Vec<String>,
    pub passphrase: String,
    pub seed: String, // New field for seed
    pub error: Option<String>,
    pub fee_percentage: f64,
    pub search_query: String, // Used as buffer_id
    pub input_mode: String, // New field for input mode
}

impl Default for TradeState {
    fn default() -> Self {
        TradeState {
            step: 1,
            done: false,
            base_asset: String::new(),
            quote_asset: String::new(),
            amount: String::new(),
            limit_price: String::new(),
            flags: Vec::new(),
            passphrase: String::new(),
            seed: String::new(), // Initialize seed
            error: None,
            fee_percentage: 0.0,
            search_query: String::new(),
            input_mode: "Passphrase".to_string(), // Default to Passphrase
        }
    }
}

impl Zeroize for TradeState {
    fn zeroize(&mut self) {
        self.step = 0;
        self.done = false;
        self.base_asset.clear();
        self.quote_asset.clear();
        self.amount.clear();
        self.limit_price.clear();
        self.flags.clear();
        self.passphrase.clear();
        self.seed.clear(); // Zeroize seed
        self.error = None;
        self.fee_percentage = 0.0;
        self.search_query.clear();
        self.input_mode.clear(); // Zeroize input_mode
    }
}

thread_local! {
    static BUFFER_STORAGE: RefCell<TradeState> = RefCell::new(TradeState::default());
}

pub fn get_or_init_buffer_id() -> String {
    BUFFER_STORAGE.with(|storage| {
        let mut trade_state = storage.borrow_mut();
        if trade_state.search_query.is_empty() {
            let new_id = Uuid::new_v4().to_string();
            trade_state.search_query = new_id.clone();
            trade_state.fee_percentage = 0.0;
            trade_state.input_mode = "Passphrase".to_string(); // Ensure default input_mode
            new_id
        } else {
            trade_state.search_query.clone()
        }
    })
}

pub fn update_buffers(
    buffer_id: &str,
    base_asset: String,
    quote_asset: String,
    amount: String,
    limit_price: String,
    flags: Vec<String>,
    passphrase: String,
    seed: String, // Add seed parameter
    step: u8,
    done: bool,
    error: Option<String>,
    fee_percentage: f64,
    search_query: String,
    input_mode: String, // Add input_mode parameter
) {
    BUFFER_STORAGE.with(|storage| {
        let mut trade_state = storage.borrow_mut();
        if trade_state.search_query == buffer_id {
            trade_state.base_asset = base_asset;
            trade_state.quote_asset = quote_asset;
            trade_state.amount = amount;
            trade_state.limit_price = limit_price;
            trade_state.flags = flags;
            trade_state.passphrase = passphrase;
            trade_state.seed = seed; // Update seed
            trade_state.step = step;
            trade_state.done = done;
            trade_state.error = error;
            trade_state.fee_percentage = fee_percentage;
            trade_state.search_query = search_query;
            trade_state.input_mode = input_mode; // Update input_mode
        }
    });
}

pub fn get_buffer(buffer_id: &str) -> Option<TradeState> {
    BUFFER_STORAGE.with(|storage| {
        let trade_state = storage.borrow();
        if trade_state.search_query == buffer_id {
            Some(trade_state.clone())
        } else {
            None
        }
    })
}

pub fn clear_buffer(buffer_id: &str) {
    BUFFER_STORAGE.with(|storage| {
        let mut trade_state = storage.borrow_mut();
        if trade_state.search_query == buffer_id {
            trade_state.zeroize();
            trade_state.search_query = String::new();
            trade_state.input_mode = "Passphrase".to_string(); // Reset to default
        }
    });
}

pub fn clear_all_buffers() {
    BUFFER_STORAGE.with(|storage| {
        let mut trade_state = storage.borrow_mut();
        trade_state.zeroize();
        trade_state.search_query = String::new();
        trade_state.input_mode = "Passphrase".to_string(); // Reset to default
    });
}