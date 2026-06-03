use std::sync::Arc;
use sql_admin_domain::history::aggregate::QueryHistory as DomainQueryHistory;
use sql_admin_domain::history::repository::QueryHistoryRepository;
use sql_admin_domain::shared::application_error::ApplicationError;

use crate::dto::{QueryHistory, SaveQueryHistoryRequest};

pub struct HistoryHandler {
    repo: Arc<dyn QueryHistoryRepository>,
}

impl HistoryHandler {
    pub fn new(repo: Arc<dyn QueryHistoryRepository>) -> Self {
        Self { repo }
    }

    pub async fn save(&self, cmd: SaveQueryHistoryRequest) -> Result<QueryHistory, ApplicationError> {
        let (history, _events) = DomainQueryHistory::create(
            cmd.connection_id,
            cmd.connection_name,
            cmd.query_text,
            cmd.execution_time_ms,
            cmd.rows_count,
            cmd.success,
            cmd.error_message,
        );

        self.repo.save(&history).await?;

        Ok(domain_history_to_dto(&history))
    }

    pub async fn list(&self) -> Result<Vec<QueryHistory>, ApplicationError> {
        let histories = self.repo.find_recent(200).await?;
        Ok(histories.iter().map(domain_history_to_dto).collect())
    }

    pub async fn delete_by_id(&self, id: &str) -> Result<bool, ApplicationError> {
        let deleted = self.repo.delete_by_id(id).await?;
        if !deleted {
            return Err(ApplicationError::NotFound(format!(
                "History item not found: {}",
                id
            )));
        }
        Ok(true)
    }

    pub async fn delete_all(&self) -> Result<(), ApplicationError> {
        self.repo.delete_all().await?;
        Ok(())
    }
}

fn domain_history_to_dto(h: &DomainQueryHistory) -> QueryHistory {
    QueryHistory {
        id: h.id().to_string(),
        connection_id: h.connection_id().to_string(),
        connection_name: h.connection_name().to_string(),
        query_text: h.query_text().to_string(),
        execution_time_ms: h.execution_time_ms().map(|t| t as u64),
        rows_count: h.rows_count(),
        success: h.success(),
        error_message: h.error_message().map(|s| s.to_string()),
        created_at: h.created_at(),
    }
}
