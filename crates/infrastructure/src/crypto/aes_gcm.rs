use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use sql_admin_domain::shared::crypto::EncryptionService;
use sql_admin_domain::shared::infrastructure_error::InfrastructureError;

const ENCRYPTION_KEY_ENV: &str = "ENCRYPTION_KEY";

pub struct AesGcmEncryptionService {
    key: [u8; 32],
}

impl AesGcmEncryptionService {
    pub fn new() -> Result<Self, InfrastructureError> {
        let key = if let Ok(key_str) = std::env::var(ENCRYPTION_KEY_ENV) {
            if key_str.is_empty() {
                return Err(InfrastructureError::EncryptionError(
                    "ENCRYPTION_KEY is empty".to_string(),
                ));
            }

            let key_bytes = if let Ok(decoded) = BASE64.decode(&key_str) {
                decoded
            } else {
                key_str.as_bytes().to_vec()
            };

            if key_bytes.len() < 16 {
                return Err(InfrastructureError::EncryptionError(format!(
                    "ENCRYPTION_KEY too short: {} bytes, minimum 16",
                    key_bytes.len()
                )));
            }

            let mut key = [0u8; 32];
            let len = key_bytes.len().min(32);
            key[..len].copy_from_slice(&key_bytes[..len]);
            key
        } else {
            return Err(InfrastructureError::EncryptionError(
                "ENCRYPTION_KEY environment variable not set".to_string(),
            ));
        };
        Ok(Self { key })
    }

    pub fn with_default_key() -> Self {
        let default_key: &[u8; 32] = b"LiteAdmin2026DefaultEncryption!!";
        Self { key: *default_key }
    }

    fn generate_nonce() -> Result<[u8; 12], InfrastructureError> {
        let mut nonce = [0u8; 12];
        getrandom::fill(&mut nonce).map_err(|e| {
            InfrastructureError::EncryptionError(format!("Failed to generate nonce: {}", e))
        })?;
        Ok(nonce)
    }
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
}
