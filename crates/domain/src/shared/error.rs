#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Connection not found: {0}")]
    ConnectionNotFound(String),

    #[error("Invalid connection config: {0}")]
    InvalidConnectionConfig(String),

    #[error("Query validation failed: {0}")]
    QueryValidationFailed(String),

    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),

    #[error("Row operation failed: {0}")]
    RowOperationFailed(String),

    #[error("Redb operation failed: {0}")]
    RedbOperationFailed(String),

    #[error("Business rule violated: {0}")]
    BusinessRuleViolated(String),

    #[error("Storage operation failed: {0}")]
    StorageError(String),
}