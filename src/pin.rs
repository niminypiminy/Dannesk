// src/pin.rs
use base64::Engine;
use rand::Rng;
use ring::pbkdf2;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use crate::utils::json_storage;
use std::num::NonZeroU32;

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
    pub pin_hash: String, // Base64-encoded PBKDF2 hash
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

    let salt = rand::thread_rng().r#gen::<[u8; 16]>();
    let iterations = 320_000;
    let mut hash = [0u8; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(iterations).unwrap(),
        &salt,
        pin.as_bytes(),
        &mut hash,
    );

    let pin_data = PinData {
        pin_hash: base64::engine::general_purpose::STANDARD.encode(&hash),
        pin_salt: base64::engine::general_purpose::STANDARD.encode(&salt),
    };
    save_pin_data(&pin_data)?;
    Ok(())
}

pub fn verify_pin(pin: &str) -> Result<(), PinError> {
    let pin_data = load_pin_data().map_err(|_| PinError::PinNotSet)?;

    let stored_hash = base64::engine::general_purpose::STANDARD
        .decode(&pin_data.pin_hash)
        .map_err(|_| PinError::IncorrectPin)?;
    let salt = base64::engine::general_purpose::STANDARD
        .decode(&pin_data.pin_salt)
        .map_err(|_| PinError::IncorrectPin)?;

    let iterations = 320_000;
    let mut computed_hash = [0u8; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(iterations).unwrap(),
        &salt,
        pin.as_bytes(),
        &mut computed_hash,
    );

    if computed_hash == stored_hash.as_slice() {
        Ok(())
    } else {
        Err(PinError::IncorrectPin)
    }
}

pub fn change_pin(old_pin: &str, new_pin: &str) -> Result<(), PinError> {
    // Verify the old PIN
    verify_pin(old_pin)?;
    // Set the new PIN
    set_pin(new_pin)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::json_storage;

    #[test]
    fn test_set_and_verify_pin() {
        let _ = json_storage::remove_json("pin.json");

        set_pin("123456").expect("Failed to set PIN");
        verify_pin("123456").expect("Correct PIN should verify");
        assert!(matches!(verify_pin("654321"), Err(PinError::IncorrectPin)));
        assert!(matches!(set_pin("12345"), Err(PinError::InvalidPin)));

        let _ = json_storage::remove_json("pin.json");
    }

    #[test]
    fn test_change_pin() {
        let _ = json_storage::remove_json("pin.json");

        set_pin("123456").expect("Failed to set PIN");
        change_pin("123456", "654321").expect("Failed to change PIN");
        verify_pin("654321").expect("New PIN should verify");
        assert!(matches!(
            change_pin("wrong", "654321"),
            Err(PinError::IncorrectPin)
        ));
        assert!(matches!(
            change_pin("654321", "12345"),
            Err(PinError::InvalidPin)
        ));

        let _ = json_storage::remove_json("pin.json");
    }
}