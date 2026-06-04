use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use sql_admin_domain::shared::crypto::EncryptionService;
use sql_admin_domain::shared::infrastructure_error::InfrastructureError;

const ENCRYPTION_KEY_ENV: &str = "ENCRYPTION_KEY";
#[cfg(feature = "keyring")]
const KEYRING_SERVICE: &str = "sql-admin";
#[cfg(feature = "keyring")]
const KEYRING_USERNAME: &str = "encryption-key";

pub struct AesGcmEncryptionService {
    key: [u8; 32],
}

impl AesGcmEncryptionService {
    /// Create encryption service from environment variable or OS keyring.
    /// Falls back to generating a random key and storing it in the keyring.
    pub fn new() -> Result<Self, InfrastructureError> {
        // 1. Try environment variable first
        if let Ok(key_str) = std::env::var(ENCRYPTION_KEY_ENV) {
            if !key_str.is_empty() {
                return Self::from_key_string(&key_str);
            }
        }

        // 2. Try OS keyring
        #[cfg(feature = "keyring")]
        {
            if let Ok(entry) = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USERNAME) {
                if let Ok(stored_key) = entry.get_password() {
                    if !stored_key.is_empty() {
                        tracing::info!(
                            module = "crypto",
                            event = "key_loaded_from_keyring",
                            "Encryption key loaded from OS keyring"
                        );
                        return Self::from_key_string(&stored_key);
                    }
                }

                // No key in keyring yet, generate and store one
                let generated = Self::generate_random_key_b64()?;
                if entry.set_password(&generated).is_ok() {
                    tracing::info!(
                        module = "crypto",
                        event = "key_generated_and_stored",
                        "Generated and stored new encryption key in OS keyring"
                    );
                    return Self::from_key_string(&generated);
                }
            }
            tracing::warn!(
                module = "crypto",
                event = "keyring_unavailable",
                "OS keyring unavailable, falling back to generated key (not persisted)"
            );
        }

        #[cfg(not(feature = "keyring"))]
        {
            tracing::warn!(
                module = "crypto",
                event = "keyring_feature_disabled",
                "Keyring feature not enabled. Set ENCRYPTION_KEY env var for persistent encryption."
            );
        }

        Err(InfrastructureError::EncryptionError(
            "No encryption key available. Set ENCRYPTION_KEY environment variable or enable 'keyring' feature.".to_string(),
        ))
    }

    /// Create with a default key for development only.
    /// Logs a warning each time it is used.
    #[deprecated(
        since = "1.2.0",
        note = "Default key is insecure for production. Use `new()` with ENCRYPTION_KEY or keyring feature."
    )]
    pub fn with_default_key() -> Self {
        tracing::warn!(
            module = "crypto",
            event = "using_default_key",
            "WARNING: Using default encryption key. This is insecure and should only be used for development."
        );
        let default_key: &[u8; 32] = b"LiteAdmin2026DefaultEncryption!!";
        Self { key: *default_key }
    }

    fn from_key_string(key_str: &str) -> Result<Self, InfrastructureError> {
        let key_bytes = if let Ok(decoded) = BASE64.decode(key_str) {
            decoded
        } else {
            // Treat as raw string only if it looks like hex (all hex chars)
            let trimmed = key_str.trim();
            if trimmed.chars().all(|c| c.is_ascii_hexdigit()) && !trimmed.is_empty() {
                hex_decode(trimmed).unwrap_or_else(|| key_str.as_bytes().to_vec())
            } else {
                key_str.as_bytes().to_vec()
            }
        };

        if key_bytes.len() < 32 {
            return Err(InfrastructureError::EncryptionError(format!(
                "ENCRYPTION_KEY too short: {} bytes, minimum 32 required for AES-256",
                key_bytes.len()
            )));
        }

        let mut key = [0u8; 32];
        key.copy_from_slice(&key_bytes[..32]);
        Ok(Self { key })
    }

    #[cfg(feature = "keyring")]
    fn generate_random_key_b64() -> Result<String, InfrastructureError> {
        let mut key = [0u8; 32];
        getrandom::fill(&mut key).map_err(|e| {
            InfrastructureError::EncryptionError(format!("Failed to generate random key: {}", e))
        })?;
        Ok(BASE64.encode(key))
    }

    fn generate_nonce() -> Result<[u8; 12], InfrastructureError> {
        let mut nonce = [0u8; 12];
        getrandom::fill(&mut nonce).map_err(|e| {
            InfrastructureError::EncryptionError(format!("Failed to generate nonce: {}", e))
        })?;
        Ok(nonce)
    }
}

/// Best-effort hex decoding. Returns None if the string is not valid hex or has odd length.
fn hex_decode(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 {
        return None;
    }
    let mut bytes = Vec::with_capacity(s.len() / 2);
    for chunk in s.as_bytes().chunks(2) {
        let high = char::from(chunk[0]).to_digit(16)?;
        let low = char::from(chunk[1]).to_digit(16)?;
        bytes.push((high << 4 | low) as u8);
    }
    Some(bytes)
}

impl EncryptionService for AesGcmEncryptionService {
    fn encrypt(&self, plaintext: &str) -> Result<String, InfrastructureError> {
        let cipher = Aes256Gcm::new_from_slice(&self.key).map_err(|e| {
            InfrastructureError::EncryptionError(format!("Cipher init failed: {}", e))
        })?;

        let nonce_bytes = Self::generate_nonce()?;
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes()).map_err(|e| {
            InfrastructureError::EncryptionError(format!("Encryption failed: {}", e))
        })?;

        let mut combined = Vec::with_capacity(12 + ciphertext.len());
        combined.extend_from_slice(&nonce_bytes);
        combined.extend_from_slice(&ciphertext);

        Ok(BASE64.encode(&combined))
    }

    fn decrypt(&self, encrypted: &str) -> Result<String, InfrastructureError> {
        // Empty string means no password was stored (e.g. Redb/Sqlite connections)
        if encrypted.is_empty() {
            return Ok(String::new());
        }

        let cipher = Aes256Gcm::new_from_slice(&self.key).map_err(|e| {
            InfrastructureError::EncryptionError(format!("Cipher init failed: {}", e))
        })?;

        let combined = BASE64.decode(encrypted).map_err(|e| {
            InfrastructureError::EncryptionError(format!("Base64 decode failed: {}", e))
        })?;

        if combined.len() < 12 {
            return Err(InfrastructureError::EncryptionError(
                "Invalid encrypted data: too short".to_string(),
            ));
        }

        let (nonce_bytes, ciphertext) = combined.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|e| {
            InfrastructureError::EncryptionError(format!("Decryption failed: {}", e))
        })?;

        String::from_utf8(plaintext).map_err(|e| {
            InfrastructureError::EncryptionError(format!("UTF-8 decode failed: {}", e))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(deprecated)]
    fn test_encrypt_decrypt_roundtrip() {
        let svc = AesGcmEncryptionService::with_default_key();
        let plaintext = "hello world";
        let encrypted = svc.encrypt(plaintext).unwrap();
        let decrypted = svc.decrypt(&encrypted).unwrap();
        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_new_without_env_fails() {
        unsafe {
            std::env::remove_var("ENCRYPTION_KEY");
        }
        let result = AesGcmEncryptionService::new();
        assert!(result.is_err());
    }

    #[test]
    fn test_from_key_string_base64() {
        let key_b64 = BASE64.encode([0x42u8; 32]);
        let svc = AesGcmEncryptionService::from_key_string(&key_b64).unwrap();
        let encrypted = svc.encrypt("test").unwrap();
        let decrypted = svc.decrypt(&encrypted).unwrap();
        assert_eq!("test", decrypted);
    }

    #[test]
    fn test_from_key_string_too_short() {
        let key_b64 = BASE64.encode([0x42u8; 16]);
        let result = AesGcmEncryptionService::from_key_string(&key_b64);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_key_string_hex() {
        let hex_key = "4242424242424242424242424242424242424242424242424242424242424242";
        let svc = AesGcmEncryptionService::from_key_string(hex_key).unwrap();
        let encrypted = svc.encrypt("hex-test").unwrap();
        let decrypted = svc.decrypt(&encrypted).unwrap();
        assert_eq!("hex-test", decrypted);
    }
}
