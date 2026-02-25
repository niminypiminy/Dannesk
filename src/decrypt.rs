use aes_gcm::{aead::{generic_array::GenericArray, Aead, KeyInit}, Aes256Gcm};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use argon2::{Argon2, Algorithm, Version, Params};
use zeroize::{Zeroize, Zeroizing};

pub fn decrypt_data(
    passphrase: Zeroizing<String>, 
    encrypted_base64: &str,
    salt_base64: &str,
    iv_base64: &str,
) -> Result<Zeroizing<String>, String> {
    
    let encrypted_data = BASE64.decode(encrypted_base64).map_err(|e| e.to_string())?;
    let salt = BASE64.decode(salt_base64).map_err(|e| e.to_string())?;
    let iv = BASE64.decode(iv_base64).map_err(|e| e.to_string())?;

    match derive_key_from_passphrase(passphrase.as_str(), &salt) {
        Ok(key) => {
            let cipher = Aes256Gcm::new(GenericArray::from_slice(key.as_ref()));
            
            let decrypted_bytes = cipher
                .decrypt(GenericArray::from_slice(&iv), encrypted_data.as_ref())
                .map_err(|e| e.to_string())?;
            
            let decrypted_string = String::from_utf8(decrypted_bytes).map_err(|e| e.to_string())?;
            
            // key drops here and is zeroized.
            // Wrap the newly decrypted string in Zeroizing to keep it safe.
            Ok(Zeroizing::new(decrypted_string)) 
        }
        Err(e) => Err(e),
    }
}

fn derive_key_from_passphrase(passphrase: &str, salt: &[u8]) -> Result<Zeroizing<Vec<u8>>, String> {
    let mut key = [0u8; 32];
    
    // Parameters must match encryption exactly
    let params = Params::new(65536, 3, 4, None).map_err(|e| e.to_string())?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    
    argon2.hash_password_into(passphrase.as_bytes(), salt, &mut key)
        .map_err(|e| e.to_string())?;
        
    let result = Zeroizing::new(key.to_vec());
    key.zeroize(); 
    
    Ok(result)
}