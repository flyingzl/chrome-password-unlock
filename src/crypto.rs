use crate::models::DecryptError;
use aes::Aes128;
use cbc::cipher::block_padding::Pkcs7;
use cbc::cipher::{BlockDecryptMut, KeyIvInit};

type Aes128CbcDec = cbc::Decryptor<Aes128>;

/// Chrome encryption configuration constants
pub const SALT: &[u8] = b"saltysalt";
pub const ITERATIONS: u32 = 1003;
pub const KEY_LENGTH: usize = 16;
pub const IV: &[u8; 16] = b"                "; // 16 spaces

/// Derive key using PBKDF2
pub fn derive_key(master_password: &str) -> Vec<u8> {
    let mut key = vec![0u8; KEY_LENGTH];
    pbkdf2::pbkdf2_hmac::<sha1::Sha1>(master_password.as_bytes(), SALT, ITERATIONS, &mut key);
    key
}

/// Decrypt Chrome password
///
/// # Arguments
/// - `derived_key`: Derived AES key
/// - `encrypted_password`: Encrypted password data (Chrome removes the 3-byte "v10" prefix)
///
/// # Returns
/// Decrypted plaintext password
pub fn decrypt_password(
    derived_key: &[u8],
    encrypted_password: &[u8],
) -> Result<String, DecryptError> {
    if derived_key.len() != KEY_LENGTH {
        return Err(DecryptError::CryptoError(format!(
            "Invalid key length: expected {}, got {}",
            KEY_LENGTH,
            derived_key.len()
        )));
    }

    // First 3 bytes of Chrome password are "v10" version identifier, need to skip
    let encrypted_data = if encrypted_password.len() > 3 {
        &encrypted_password[3..]
    } else {
        encrypted_password
    };

    // Decrypt using AES-128-CBC
    let decryptor = Aes128CbcDec::new(derived_key.into(), IV.into());
    let mut buffer = encrypted_data.to_vec();

    // Decrypt and remove PKCS7 padding
    let decrypted_len = decryptor
        .decrypt_padded_mut::<Pkcs7>(&mut buffer)
        .map_err(|e| {
            tracing::debug!("Password decryption failed: {}", e);
            DecryptError::CryptoError(format!("Decryption failed: {}", e))
        })?
        .len();

    buffer.truncate(decrypted_len);

    let result = String::from_utf8(buffer)
        .map_err(|e| DecryptError::CryptoError(format!("Invalid UTF-8: {}", e)))?;

    tracing::debug!("Successfully decrypted password");
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_key() {
        let master_password = "test_password";
        let key = derive_key(master_password);
        assert_eq!(key.len(), KEY_LENGTH);
    }

    #[test]
    fn test_iv_length() {
        assert_eq!(IV.len(), 16);
    }
}
