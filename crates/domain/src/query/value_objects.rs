use crate::shared::error::DomainError;

const BLOCKED_KEYWORDS: &[&str] = &[
    "DROP DATABASE",
    "SHUTDOWN",
    "GRANT ALL",
    "ALTER USER",
    "TRUNCATE",
    "DROP TABLE",
    "DROP SCHEMA",
    "ALTER SYSTEM",
];

#[derive(Clone, Debug, PartialEq)]
pub struct QueryText(String);

impl QueryText {
    pub fn new(sql: &str) -> Result<Self, DomainError> {
        if sql.trim().is_empty() {
            return Err(DomainError::QueryValidationFailed(
                "Query cannot be empty".to_string(),
            ));
        }
        let upper = sql.to_uppercase();
        for keyword in BLOCKED_KEYWORDS {
            if upper.contains(keyword) {
                return Err(DomainError::QueryValidationFailed(format!(
                    "Dangerous SQL operation blocked: {}",
                    keyword
                )));
            }
        }
        Ok(Self(sql.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn is_select_query(&self) -> bool {
        let trimmed = self.0.trim_start().to_uppercase();
        trimmed.starts_with("SELECT")
            || trimmed.starts_with("WITH")
            || trimmed.starts_with("SHOW")
            || trimmed.starts_with("DESCRIBE")
            || trimmed.starts_with("EXPLAIN")
            || trimmed.starts_with("PRAGMA")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExecutionMetrics {
    pub execution_time_ms: u64,
    pub rows_affected: Option<u64>,
    pub rows_count: Option<i64>,
}

impl ExecutionMetrics {
    pub fn new(execution_time_ms: u64, rows_affected: Option<u64>, rows_count: Option<i64>) -> Self {
        Self {
            execution_time_ms,
            rows_affected,
            rows_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_text_valid_select() {
        let result = QueryText::new("SELECT * FROM users");
        assert!(result.is_ok());
        let q = result.unwrap();
        assert!(q.is_select_query());
    }

    #[test]
    fn test_query_text_empty() {
        let result = QueryText::new("");
        assert!(result.is_err());
        match result.unwrap_err() {
            DomainError::QueryValidationFailed(msg) => {
                assert!(msg.contains("empty"));
            }
            _ => panic!("Expected QueryValidationFailed"),
        }
    }

    #[test]
    fn test_query_text_blocked_drop_database() {
        let result = QueryText::new("DROP DATABASE test");
        assert!(result.is_err());
    }

    #[test]
    fn test_query_text_blocked_truncate() {
        let result = QueryText::new("TRUNCATE TABLE users");
        assert!(result.is_err());
    }

    #[test]
    fn test_query_text_insert() {
        let result = QueryText::new("INSERT INTO users VALUES (1)");
        assert!(result.is_ok());
        let q = result.unwrap();
        assert!(!q.is_select_query());
    }

    #[test]
    fn test_query_text_with_cte() {
        let result = QueryText::new("WITH cte AS (SELECT 1) SELECT * FROM cte");
        assert!(result.is_ok());
        let q = result.unwrap();
        assert!(q.is_select_query());
    }
}