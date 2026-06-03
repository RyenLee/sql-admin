use chrono::{DateTime, Utc};
use crate::id;
use crate::shared::event::DomainEvent;

#[derive(Debug)]
pub struct QueryHistory {
    id: String,
    connection_id: String,
    connection_name: String,
    query_text: String,
    execution_time_ms: Option<i64>,
    rows_count: Option<i64>,
    success: bool,
    error_message: Option<String>,
    created_at: DateTime<Utc>,
}

impl QueryHistory {
    pub fn create(
        connection_id: String,
        connection_name: String,
        query_text: String,
        execution_time_ms: Option<u64>,
        rows_count: Option<i64>,
        success: bool,
        error_message: Option<String>,
    ) -> (Self, Vec<DomainEvent>) {
        let now = Utc::now();
        let id = id::generate_id_string();

        let history = Self {
            id,
            connection_id,
            connection_name,
            query_text,
            execution_time_ms: execution_time_ms.map(|t| t as i64),
            rows_count,
            success,
            error_message,
            created_at: now,
        };

        let events: Vec<DomainEvent> = Vec::new();

        (history, events)
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn connection_id(&self) -> &str {
        &self.connection_id
    }

    pub fn connection_name(&self) -> &str {
        &self.connection_name
    }

    pub fn query_text(&self) -> &str {
        &self.query_text
    }

    pub fn execution_time_ms(&self) -> Option<i64> {
        self.execution_time_ms
    }

    pub fn rows_count(&self) -> Option<i64> {
        self.rows_count
    }

    pub fn success(&self) -> bool {
        self.success
    }

    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_query_history() {
        let (history, events) = QueryHistory::create(
            "conn-123".to_string(),
            "My DB".to_string(),
            "SELECT 1".to_string(),
            Some(42),
            Some(1),
            true,
            None,
        );
        assert!(!history.id().is_empty());
        assert_eq!(history.connection_id(), "conn-123");
        assert_eq!(history.query_text(), "SELECT 1");
        assert_eq!(history.execution_time_ms(), Some(42));
        assert!(history.success());
        assert!(history.error_message().is_none());
        assert_eq!(events.len(), 0);
    }
}