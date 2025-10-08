use aes_gcm::{aead::{generic_array::GenericArray, Aead, KeyInit}, Aes256Gcm};
use base64::Engine;
use ring::pbkdf2;
use std::num::NonZeroU32;
use zeroize::Zeroize;

pub fn decrypt_data(
    mut passphrase: String, // Take ownership to allow zeroing
    encrypted_data: String,
    salt: String,
    iv: String,
) -> Result<String, String> {
    let encrypted_data = base64::engine::general_purpose::STANDARD
        .decode(&encrypted_data)
        .map_err(|e| format!("Invalid encrypted data: {:?}", e))?;
    let salt = base64::engine::general_purpose::STANDARD
        .decode(&salt)
        .map_err(|e| format!("Invalid salt: {:?}", e))?;
    let iv = base64::engine::general_purpose::STANDARD
        .decode(&iv)
        .map_err(|e| format!("Invalid IV: {:?}", e))?;

    match derive_key_from_passphrase(&passphrase, &salt) {
        Ok(mut key) => {
            let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));
            let mut decrypted_data = cipher
                .decrypt(GenericArray::from_slice(&iv), encrypted_data.as_slice())
                .map_err(|e| format!("Decryption failed: {:?}", e))?;

            let result = String::from_utf8(decrypted_data.clone())
                .map_err(|e| format!("Invalid UTF-8: {:?}", e))?;

            // Zero out sensitive data
            passphrase.zeroize();
            key.zeroize();
            decrypted_data.zeroize();

            Ok(result)
        }
        Err(e) => {
            // Zero out sensitive data before returning error
            passphrase.zeroize();
            Err(e)
        }
    }
}

fn derive_key_from_passphrase(passphrase: &str, salt: &[u8]) -> Result<Vec<u8>, String> {
    let iterations = 500_000;
    let mut key = [0u8; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(iterations).unwrap(),
        salt,
        passphrase.as_bytes(),
        &mut key,
    );
    let result = key.to_vec();
    // Zero out the stack-allocated key
    key.fill(0);
    Ok(result)
}