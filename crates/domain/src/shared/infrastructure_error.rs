#[derive(Debug, thiserror::Error)]
pub enum InfrastructureError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Connection pool creation failed: {0}")]
    PoolCreationFailed(String),

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Event bus error: {0}")]
    EventBusError(String),
}