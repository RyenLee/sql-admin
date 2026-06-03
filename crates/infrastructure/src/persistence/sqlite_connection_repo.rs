use async_trait::async_trait;
use sql_admin_domain::connection::aggregate::Connection;
use sql_admin_domain::connection::repository::ConnectionRepository;
use sql_admin_domain::connection::value_objects::{
    ConnectionConfig, DatabaseType, EncryptedPassword,
};
use sql_admin_domain::shared::error::DomainError;
use sqlx::{Row, SqlitePool};

pub struct SqliteConnectionRepository {
    pool: SqlitePool,
}

impl SqliteConnectionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

fn row_to_connection(
    row: sqlx::sqlite::SqliteRow,
) -> Result<Connection, DomainError> {
    let db_type_str: String = row.get("database_type");
    let database_type = DatabaseType::try_from(db_type_str.as_str())
        .map_err(|e| DomainError::InvalidConnectionConfig(format!("Unknown database type in storage: {}", e)))?;

    let encrypted_password: Option<String> = row.get("password");
    let encrypted = EncryptedPassword::new(encrypted_password.unwrap_or_default());

    let port: i32 = row.get("port");
    let port = if port <= 0 { 5432 } else { port as u16 };

    let config = ConnectionConfig::new(
        row.get("host"),
        port,
        row.get("database"),
        row.get("username"),
        encrypted,
        database_type.clone(),
    )?;

    let name: String = row.get("name");
    let id: String = row.get("id");
    let created_at = row.get("created_at");
    let updated_at = row.get("updated_at");

    Ok(Connection::reconstitute(
        id,
        name,
        database_type,
        config,
        created_at,
        updated_at,
    ))
}

#[async_trait]
impl ConnectionRepository for SqliteConnectionRepository {
    async fn save(&self, conn: &Connection) -> Result<(), DomainError> {
        let db_type_str = conn.database_type().to_string();
        let encrypted = conn.encrypted_password().as_str().to_string();

        sqlx::query(
            r#"
            INSERT INTO connections (id, name, database_type, host, port, database, username, password, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                database_type = excluded.database_type,
                host = excluded.host,
                port = excluded.port,
                database = excluded.database,
                username = excluded.username,
                password = excluded.password,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(conn.id())
        .bind(conn.name())
        .bind(&db_type_str)
        .bind(conn.host())
        .bind(conn.port() as i32)
        .bind(conn.database())
        .bind(conn.username())
        .bind(&encrypted)
        .bind(conn.created_at())
        .bind(conn.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::StorageError(format!("Failed to save connection: {}", e)))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Connection>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, name, database_type, host, port, database, username, password, created_at, updated_at
            FROM connections
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::StorageError(format!("Failed to find connection: {}", e)))?;

        row.map(row_to_connection)
            .transpose()
    }

    async fn find_all(&self) -> Result<Vec<Connection>, DomainError> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, database_type, host, port, database, username, password, created_at, updated_at
            FROM connections
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::StorageError(format!("Failed to list connections: {}", e)))?;

        rows.into_iter()
            .map(row_to_connection)
            .collect()
    }

    async fn delete(&self, id: &str) -> Result<bool, DomainError> {
        let result = sqlx::query("DELETE FROM connections WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::StorageError(format!("Failed to delete connection: {}", e)))?;

        Ok(result.rows_affected() > 0)
    }
}