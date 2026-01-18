use aes_gcm::{aead::{generic_array::GenericArray, Aead, KeyInit}, Aes256Gcm};
use base64::Engine;
use ring::pbkdf2;
use std::num::NonZeroU32;
use zeroize::{Zeroize, Zeroizing}; // Import Zeroizing

pub fn decrypt_data(
    passphrase: Zeroizing<String>, 
    encrypted_data: String,
    salt: String,
    iv: String,
) -> Result<String, String> {
    let encrypted_data = base64::engine::general_purpose::STANDARD
        .decode(&encrypted_data)
        .map_err(|e| format!("Invalid base64 data: {:?}", e))?;
    let salt = base64::engine::general_purpose::STANDARD
        .decode(&salt)
        .map_err(|e| format!("Invalid salt: {:?}", e))?;
    let iv = base64::engine::general_purpose::STANDARD
        .decode(&iv)
        .map_err(|e| format!("Invalid IV: {:?}", e))?;

    match derive_key_from_passphrase(passphrase.as_str(), &salt) {
        Ok(mut key) => {
            let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));
            
            // Decrypt directly into a mutable buffer
            let decrypted_bytes = cipher
                .decrypt(GenericArray::from_slice(&iv), encrypted_data.as_slice())
                .map_err(|e| format!("Decryption failed: {:?}", e))?;

            // Convert to String without cloning to keep memory footprint small
            let result = String::from_utf8(decrypted_bytes)
                .map_err(|e| {
                    // Note: if this fails, decrypted_bytes is consumed and dropped
                    format!("Invalid UTF-8: {:?}", e)
                })?;

            // Clean up the derived key
            key.zeroize();

            // The 'result' is now owned by the caller. 
            // In authenticate_wallet, you correctly wrap this in Zeroizing::new().
            Ok(result)
        }
        Err(e) => Err(e),
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
    
    // Using the zeroize trait is slightly more robust than fill(0)
    key.zeroize(); 
    
    Ok(result)
}