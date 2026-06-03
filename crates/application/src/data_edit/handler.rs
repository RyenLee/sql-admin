use std::sync::Arc;
use sql_admin_domain::connection::repository::ConnectionRepository;
use sql_admin_domain::connection::value_objects::DatabaseType;
use sql_admin_domain::shared::application_error::ApplicationError;
use sql_admin_domain::shared::crypto::EncryptionService;
use sql_admin_domain::shared::event::{DomainEvent, EventBus};
use sql_admin_domain::shared::pool::PoolFactory;

use crate::connection_pool_service::ConnectionPoolService;
use crate::dto::{DeleteRowRequest, EditRowRequest, InsertRowRequest};
use crate::utils::is_safe_identifier;

pub struct DataEditHandler {
    pool_service: ConnectionPoolService,
    event_bus: Arc<dyn EventBus>,
}

impl DataEditHandler {
    pub fn new(
        conn_repo: Arc<dyn ConnectionRepository>,
        pool_factory: Arc<dyn PoolFactory>,
        crypto: Arc<dyn EncryptionService>,
        event_bus: Arc<dyn EventBus>,
    ) -> Self {
        let pool_service = ConnectionPoolService::new(conn_repo, pool_factory, crypto);
        Self {
            pool_service,
            event_bus,
        }
    }

    pub async fn edit_row(&self, cmd: EditRowRequest) -> Result<u64, ApplicationError> {
        if !is_safe_identifier(&cmd.table_name) {
            return Err(ApplicationError::Validation(format!(
                "Invalid table name: {}",
                cmd.table_name
            )));
        }
        if !is_safe_identifier(&cmd.primary_key_column) {
            return Err(ApplicationError::Validation(format!(
                "Invalid column name: {}",
                cmd.primary_key_column
            )));
        }
        if !is_safe_identifier(&cmd.column) {
            return Err(ApplicationError::Validation(format!(
                "Invalid column name: {}",
                cmd.column
            )));
        }

        let (executor, db_type) = self.pool_service.get_executor_with_type(&cmd.connection_id).await?;
        let q = |s: &str| quote_identifier(s, &db_type);

        let value = cmd.new_value.map(|v| value_to_sql_literal(&v)).unwrap_or_else(|| "NULL".to_string());
        let pk_value = value_to_sql_literal(&cmd.primary_key_value);

        let sql = format!(
            "UPDATE {} SET {} = {} WHERE {} = {}",
            q(&cmd.table_name), q(&cmd.column), value, q(&cmd.primary_key_column), pk_value
        );

        let result = executor.execute_dml(&sql).await?;
        let affected = result.rows_affected;

        let event = DomainEvent::RowEdited {
            connection_id: cmd.connection_id.clone(),
            table_name: cmd.table_name.clone(),
            column: cmd.column.clone(),
            timestamp: chrono::Utc::now(),
        };
        let _ = self.event_bus.publish(event).await;

        Ok(affected)
    }

    pub async fn delete_row(&self, cmd: DeleteRowRequest) -> Result<u64, ApplicationError> {
        if !is_safe_identifier(&cmd.table_name) {
            return Err(ApplicationError::Validation(format!(
                "Invalid table name: {}",
                cmd.table_name
            )));
        }
        if !is_safe_identifier(&cmd.primary_key_column) {
            return Err(ApplicationError::Validation(format!(
                "Invalid column name: {}",
                cmd.primary_key_column
            )));
        }

        let (executor, db_type) = self.pool_service.get_executor_with_type(&cmd.connection_id).await?;
        let q = |s: &str| quote_identifier(s, &db_type);

        let pk_value = value_to_sql_literal(&cmd.primary_key_value);

        let sql = format!(
            "DELETE FROM {} WHERE {} = {}",
            q(&cmd.table_name), q(&cmd.primary_key_column), pk_value
        );

        let result = executor.execute_dml(&sql).await?;
        let affected = result.rows_affected;

        let event = DomainEvent::RowEdited {
            connection_id: cmd.connection_id.clone(),
            table_name: cmd.table_name.clone(),
            column: cmd.primary_key_column.clone(),
            timestamp: chrono::Utc::now(),
        };
        let _ = self.event_bus.publish(event).await;

        Ok(affected)
    }

    pub async fn insert_row(&self, cmd: InsertRowRequest) -> Result<u64, ApplicationError> {
        if !is_safe_identifier(&cmd.table_name) {
            return Err(ApplicationError::Validation(format!(
                "Invalid table name: {}",
                cmd.table_name
            )));
        }

        for col in &cmd.columns {
            if !is_safe_identifier(col) {
                return Err(ApplicationError::Validation(format!(
                    "Invalid column name: {}",
                    col
                )));
            }
        }

        if cmd.columns.len() != cmd.values.len() {
            return Err(ApplicationError::Validation(
                "Columns and values length mismatch".to_string(),
            ));
        }

        let (executor, db_type) = self.pool_service.get_executor_with_type(&cmd.connection_id).await?;
        let q = |s: &str| quote_identifier(s, &db_type);

        let cols: Vec<String> = cmd.columns.iter().map(|c| q(c)).collect();
        let vals: Vec<String> = cmd.values.iter().map(value_to_sql_literal).collect();

        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            q(&cmd.table_name),
            cols.join(", "),
            vals.join(", ")
        );

        let result = executor.execute_dml(&sql).await?;
        let affected = result.rows_affected;

        let event = DomainEvent::RowEdited {
            connection_id: cmd.connection_id.clone(),
            table_name: cmd.table_name.clone(),
            column: cmd.columns.join(", "),
            timestamp: chrono::Utc::now(),
        };
        let _ = self.event_bus.publish(event).await;

        Ok(affected)
    }
}

fn quote_identifier(name: &str, db_type: &DatabaseType) -> String {
    match db_type {
        DatabaseType::Mysql => format!("`{}`", name),
        _ => format!("\"{}\"", name),
    }
}

fn value_to_sql_literal(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "NULL".to_string(),
        serde_json::Value::Bool(b) => if *b { "TRUE".to_string() } else { "FALSE".to_string() },
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i.to_string()
            } else if let Some(f) = n.as_f64() {
                if f.is_finite() {
                    f.to_string()
                } else {
                    "NULL".to_string()
                }
            } else {
                n.to_string()
            }
        }
        serde_json::Value::String(s) => {
            let escaped = s.replace('\'', "''");
            format!("'{}'", escaped)
        }
        serde_json::Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(value_to_sql_literal).collect();
            items.join(", ")
        }
        serde_json::Value::Object(obj) => {
            let json_str = serde_json::to_string(obj).unwrap_or_default();
            let escaped = json_str.replace('\'', "''");
            format!("'{}'", escaped)
        }
    }
}
