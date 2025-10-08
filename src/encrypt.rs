use aes_gcm::{aead::{generic_array::GenericArray, Aead, KeyInit}, Aes256Gcm};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use rand::Rng;
use ring::pbkdf2;
use std::num::NonZeroU32;
use zeroize::Zeroize;

pub fn encrypt_data(
    mut passphrase: String, // Take ownership to allow zeroing
    mut seed: String,      // Take ownership to allow zeroing
) -> Result<(String, String, String), String> {
    let salt = rand::thread_rng().r#gen::<[u8; 16]>().to_vec();
    let iv = rand::thread_rng().r#gen::<[u8; 12]>().to_vec();

    match derive_key_from_passphrase(&passphrase, &salt) {
        Ok(mut key) => {
            let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));
            let encrypted_data = cipher
                .encrypt(GenericArray::from_slice(&iv), seed.as_bytes())
                .map_err(|e| e.to_string())?;

            let base64_encrypted = BASE64.encode(&encrypted_data);
            let base64_salt = BASE64.encode(&salt);
            let base64_iv = BASE64.encode(&iv);

            // Zero out sensitive data
            passphrase.zeroize();
            seed.zeroize();
            key.zeroize();

            Ok((base64_encrypted, base64_salt, base64_iv))
        }
        Err(e) => {
            // Zero out sensitive data before returning error
            passphrase.zeroize();
            seed.zeroize();
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