use aes_gcm::{aead::{generic_array::GenericArray, Aead, KeyInit}, Aes256Gcm};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use rand::Rng;
use ring::pbkdf2;
use std::num::NonZeroU32;
use zeroize::{Zeroize, Zeroizing};

//zeroized all strings to be extra safe.

pub fn encrypt_data(
    passphrase: Zeroizing<String>, 
    seed: Zeroizing<String>,      
) -> Result<(String, String, String), String> {
    let salt = rand::thread_rng().r#gen::<[u8; 16]>().to_vec();
    let iv = rand::thread_rng().r#gen::<[u8; 12]>().to_vec();

    // Dereference string slice for derivation. 
    match derive_key_from_passphrase(passphrase.as_str(), &salt) {
        Ok(mut key) => {
            let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));
            
            // seed guard 
            let encrypted_data = cipher
                .encrypt(GenericArray::from_slice(&iv), seed.as_bytes())
                .map_err(|e| e.to_string())?;

            let base64_encrypted = BASE64.encode(&encrypted_data);
            let base64_salt = BASE64.encode(&salt);
            let base64_iv = BASE64.encode(&iv);

            
            key.zeroize();

            // passphrase and seed guards both wiped
            Ok((base64_encrypted, base64_salt, base64_iv))
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
    key.zeroize(); 
    Ok(result)
}