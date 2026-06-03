use super::infrastructure_error::InfrastructureError;

#[async_trait::async_trait]
pub trait EncryptionService: Send + Sync {
    fn encrypt(&self, plaintext: &str) -> Result<String, InfrastructureError>;
    fn decrypt(&self, ciphertext: &str) -> Result<String, InfrastructureError>;
}