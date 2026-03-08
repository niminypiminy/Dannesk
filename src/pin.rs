use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use crate::utils::json_storage;
use argon2::{Argon2, Algorithm, Version, Params};
use zeroize::{Zeroize};

#[derive(Debug)]
pub enum PinError {
    InvalidPin,
    IoError(String),
    PinNotSet,
    IncorrectPin,
}

impl fmt::Display for PinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PinError::InvalidPin => write!(f, "Invalid PIN: must be a six-digit number"),
            PinError::IoError(e) => write!(f, "IO error: {}", e),
            PinError::PinNotSet => write!(f, "PIN not set"),
            PinError::IncorrectPin => write!(f, "Incorrect PIN"),
        }
    }
}

impl Error for PinError {}

#[derive(Serialize, Deserialize, Debug)]
pub struct PinData {
    pub pin_hash: String, // Base64-encoded Argon2 hash
    pub pin_salt: String, // Base64-encoded salt
}

pub fn load_pin_data() -> Result<PinData, PinError> {
    json_storage::read_json("pin.json")
        .map_err(|e| PinError::IoError(e.to_string()))
}

pub fn save_pin_data(pin_data: &PinData) -> Result<(), PinError> {
    json_storage::write_json("pin.json", pin_data)
        .map_err(|e| PinError::IoError(e.to_string()))?;
    Ok(())
}

pub fn set_pin(pin: &str) -> Result<(), PinError> {
    if !pin.chars().all(|c| c.is_digit(10)) || pin.len() != 6 {
        return Err(PinError::InvalidPin);
    }

    let salt: [u8; 16] = rand::rng().random();
    let mut hash = [0u8; 32];

    // Use consistent parameters: 64MB RAM, 3 iterations, 4 threads
    let params = Params::new(65536, 3, 4, None).map_err(|_| PinError::InvalidPin)?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    argon2.hash_password_into(pin.as_bytes(), &salt, &mut hash)
        .map_err(|_| PinError::InvalidPin)?;

    let pin_data = PinData {
        pin_hash: BASE64.encode(&hash),
        pin_salt: BASE64.encode(&salt),
    };

    hash.zeroize(); 
    save_pin_data(&pin_data)?;
    Ok(())
}

pub fn verify_pin(pin: &str) -> Result<(), PinError> {
    let pin_data = load_pin_data().map_err(|_| PinError::PinNotSet)?;

    let stored_hash = BASE64.decode(&pin_data.pin_hash)
        .map_err(|_| PinError::IncorrectPin)?;
    let salt = BASE64.decode(&pin_data.pin_salt)
        .map_err(|_| PinError::IncorrectPin)?;

    let mut computed_hash = [0u8; 32];
    let params = Params::new(65536, 3, 4, None).map_err(|_| PinError::IncorrectPin)?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    argon2.hash_password_into(pin.as_bytes(), &salt, &mut computed_hash)
        .map_err(|_| PinError::IncorrectPin)?;

  
    let is_valid = computed_hash == stored_hash.as_slice();
    computed_hash.zeroize();

    if is_valid {
        Ok(())
    } else {
        Err(PinError::IncorrectPin)
    }
}

pub fn change_pin(old_pin: &str, new_pin: &str) -> Result<(), PinError> {
    verify_pin(old_pin)?;
    set_pin(new_pin)?;
    Ok(())
}
