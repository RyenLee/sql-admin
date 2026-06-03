use super::infrastructure_error::InfrastructureError;

pub trait EncryptionService: Send + Sync {
    fn encrypt(&self, plaintext: &str) -> Result<String, InfrastructureError>;
    fn decrypt(&self, ciphertext: &str) -> Result<String, InfrastructureError>;
}