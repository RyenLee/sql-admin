use std::sync::Arc;
use sql_admin_domain::connection::repository::ConnectionRepository;
use sql_admin_domain::shared::application_error::ApplicationError;
use sql_admin_domain::shared::crypto::EncryptionService;
use sql_admin_domain::shared::pool::PoolFactory;

use crate::connection_pool_service::ConnectionPoolService;
use crate::dto::{ImportResult, ImportSqlRequest, TransactionMode};

pub struct ImportHandler {
    pool_service: ConnectionPoolService,
}

impl ImportHandler {
    pub fn new(
        conn_repo: Arc<dyn ConnectionRepository>,
        pool_factory: Arc<dyn PoolFactory>,
        crypto: Arc<dyn EncryptionService>,
    ) -> Self {
        let pool_service = ConnectionPoolService::new(conn_repo, pool_factory, crypto);
        Self {
            pool_service,
        }
    }

    pub async fn import_sql(&self, cmd: ImportSqlRequest) -> Result<ImportResult, ApplicationError> {
        // Validate sql_content size (max 10MB)
        const MAX_SQL_SIZE: usize = 10 * 1024 * 1024;
        if cmd.sql_content.len() > MAX_SQL_SIZE {
            return Err(ApplicationError::Validation(format!(
                "SQL content too large: {} bytes, maximum {} bytes",
                cmd.sql_content.len(),
                MAX_SQL_SIZE
            )));
        }

        let executor = self.pool_service.get_executor(&cmd.connection_id).await?;
        let start = std::time::Instant::now();

        let statements = split_sql_statements(&cmd.sql_content);
        let total = statements.len();

        match cmd.transaction_mode {
            TransactionMode::AllOrNothing => {
                // Execute all statements; on first error, stop and report
                let mut executed = 0u32;
                let mut errors = Vec::new();

                for (i, stmt) in statements.iter().enumerate() {
                    let trimmed = stmt.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    match executor.execute_dml(trimmed).await {
                        Ok(_result) => {
                            executed += 1;
                        }
                        Err(e) => {
                            errors.push(format!("Statement {} error: {}", i + 1, e));
                            // Stop on first error in AllOrNothing mode
                            break;
                        }
                    }
                }

                if !errors.is_empty() {
                    tracing::warn!(
                        module = "import_handler",
                        total_statements = total,
                        executed,
                        "Import failed in AllOrNothing mode, {} of {} statements executed before error",
                        executed,
                        total
                    );
                }

                Ok(ImportResult {
                    statements_executed: executed,
                    errors,
                    execution_time_ms: Some(start.elapsed().as_millis() as u64),
                })
            }
            TransactionMode::ContinueOnError => {
                let mut executed = 0u32;
                let mut errors = Vec::new();

                for (i, stmt) in statements.iter().enumerate() {
                    let trimmed = stmt.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    match executor.execute_dml(trimmed).await {
                        Ok(_result) => {
                            executed += 1;
                        }
                        Err(e) => {
                            errors.push(format!("Statement {} error: {}", i + 1, e));
                        }
                    }
                }

                Ok(ImportResult {
                    statements_executed: executed,
                    errors,
                    execution_time_ms: Some(start.elapsed().as_millis() as u64),
                })
            }
        }
    }
}

fn split_sql_statements(sql: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut in_backtick = false;
    let mut in_line_comment = false;
    let mut in_block_comment = false;
    let mut chars = sql.chars().peekable();

    while let Some(ch) = chars.next() {
        if in_line_comment {
            if ch == '\n' {
                in_line_comment = false;
            }
            continue;
        }

        if in_block_comment {
            if ch == '*' && chars.peek() == Some(&'/') {
                chars.next();
                in_block_comment = false;
            }
            continue;
        }

        if in_single_quote {
            current.push(ch);
            if ch == '\'' {
                in_single_quote = false;
            }
            continue;
        }

        if in_double_quote {
            current.push(ch);
            if ch == '"' {
                in_double_quote = false;
            }
            continue;
        }

        if in_backtick {
            current.push(ch);
            if ch == '`' {
                in_backtick = false;
            }
            continue;
        }

        match ch {
            '\'' => {
                in_single_quote = true;
                current.push(ch);
            }
            '"' => {
                in_double_quote = true;
                current.push(ch);
            }
            '`' => {
                in_backtick = true;
                current.push(ch);
            }
            '-' if chars.peek() == Some(&'-') => {
                chars.next();
                in_line_comment = true;
            }
            '/' if chars.peek() == Some(&'*') => {
                chars.next();
                in_block_comment = true;
            }
            ';' => {
                let trimmed = current.trim().to_string();
                if !trimmed.is_empty() {
                    statements.push(trimmed);
                }
                current = String::new();
            }
            _ => {
                current.push(ch);
            }
        }
    }

    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() {
        statements.push(trimmed);
    }

    statements
}
