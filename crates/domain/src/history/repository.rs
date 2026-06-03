use async_trait::async_trait;
use crate::shared::error::DomainError;
use super::aggregate::QueryHistory;

#[async_trait]
pub trait QueryHistoryRepository: Send + Sync {
    async fn save(&self, history: &QueryHistory) -> Result<(), DomainError>;
    async fn find_recent(&self, limit: u64) -> Result<Vec<QueryHistory>, DomainError>;
    async fn delete_by_id(&self, id: &str) -> Result<bool, DomainError>;
    async fn delete_all(&self) -> Result<(), DomainError>;
}