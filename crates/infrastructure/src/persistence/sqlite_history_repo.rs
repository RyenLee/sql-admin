use async_trait::async_trait;
use sql_admin_domain::history::aggregate::QueryHistory;
use sql_admin_domain::history::repository::QueryHistoryRepository;
use sql_admin_domain::shared::error::DomainError;
use sqlx::{Row, SqlitePool};

pub struct SqliteQueryHistoryRepository {
    pool: SqlitePool,
}

impl SqliteQueryHistoryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

fn row_to_history(row: sqlx::sqlite::SqliteRow) -> Result<QueryHistory, DomainError> {
    let (history, _) = QueryHistory::create(
        row.get("connection_id"),
        row.get("connection_name"),
        row.get("query_text"),
        row.get::<Option<i64>, _>("execution_time_ms").map(|t| t as u64),
        row.get("rows_count"),
        row.get("success"),
        row.get("error_message"),
    );
    Ok(history)
}

#[async_trait]
impl QueryHistoryRepository for SqliteQueryHistoryRepository {
    async fn save(&self, history: &QueryHistory) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            INSERT INTO query_history (id, connection_id, connection_name, query_text, execution_time_ms, rows_count, success, error_message, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(history.id())
        .bind(history.connection_id())
        .bind(history.connection_name())
        .bind(history.query_text())
        .bind(history.execution_time_ms())
        .bind(history.rows_count())
        .bind(history.success())
        .bind(history.error_message())
        .bind(history.created_at())
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::StorageError(format!("Failed to save query history: {}", e)))?;

        Ok(())
    }

    async fn find_recent(&self, limit: u64) -> Result<Vec<QueryHistory>, DomainError> {
        let rows = sqlx::query(
            r#"
            SELECT id, connection_id, connection_name, query_text, execution_time_ms, rows_count, success, error_message, created_at
            FROM query_history
            ORDER BY created_at DESC
            LIMIT ?
            "#,
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::StorageError(format!("Failed to query history: {}", e)))?;

        rows.into_iter().map(row_to_history).collect()
    }

    async fn delete_by_id(&self, id: &str) -> Result<bool, DomainError> {
        let result = sqlx::query("DELETE FROM query_history WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::StorageError(format!("Failed to delete history item: {}", e)))?;

        Ok(result.rows_affected() > 0)
    }

    async fn delete_all(&self) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM query_history")
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::StorageError(format!("Failed to clear history: {}", e)))?;

        Ok(())
    }
}