use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};

const ENCRYPTION_KEY_ENV: &str = "ENCRYPTION_KEY";
const DEFAULT_KEY: &[u8; 32] = b"LiteAdmin2026DefaultEncryption!!";

fn get_key() -> [u8; 32] {
    if let Ok(key_b64) = std::env::var(ENCRYPTION_KEY_ENV) {
        if let Ok(key_bytes) = BASE64.decode(&key_b64) {
            let mut key = [0u8; 32];
            let len = key_bytes.len().min(32);
            key[..len].copy_from_slice(&key_bytes[..len]);
            return key;
        }
    }
    *DEFAULT_KEY
}

/// 生成安全的随机数
pub mod secure_nonce {
    const AES_GCM_NONCE_LENGTH: usize = 12;

    pub fn generate() -> Result<[u8; AES_GCM_NONCE_LENGTH], String> {
        let mut nonce = [0u8; AES_GCM_NONCE_LENGTH];
        getrandom::fill(&mut nonce)
            .map_err(|e| format!("Failed to generate secure nonce: {}", e))?;
        Ok(nonce)
    }
}

pub fn encrypt(plaintext: &str) -> Result<String, String> {
    let key = get_key();
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| e.to_string())?;

    let nonce_bytes = secure_nonce::generate()?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| e.to_string())?;

    let mut combined = Vec::with_capacity(12 + ciphertext.len());
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);

    Ok(BASE64.encode(&combined))
}

pub fn decrypt(encrypted: &str) -> Result<String, String> {
    let key = get_key();
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| e.to_string())?;

    let combined = BASE64.decode(encrypted).map_err(|e| e.to_string())?;

    if combined.len() < 12 {
        return Err("Invalid encrypted data".to_string());
    }

    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| e.to_string())?;

    String::from_utf8(plaintext).map_err(|e| e.to_string())
}
