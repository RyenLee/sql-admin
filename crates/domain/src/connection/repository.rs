use async_trait::async_trait;
use crate::shared::error::DomainError;
use super::aggregate::Connection;

#[async_trait]
pub trait ConnectionRepository: Send + Sync {
    async fn save(&self, conn: &Connection) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Connection>, DomainError>;
    async fn find_all(&self) -> Result<Vec<Connection>, DomainError>;
    async fn delete(&self, id: &str) -> Result<bool, DomainError>;
}