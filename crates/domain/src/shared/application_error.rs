use super::error::DomainError;
use super::infrastructure_error::InfrastructureError;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error(transparent)]
    Infrastructure(#[from] InfrastructureError),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Unauthorized")]
    Unauthorized,
}