use crate::crypto;
use crate::id_generator;
use chrono::Utc;
use sql_admin_shared::{AppError, Connection, DatabaseType};
use sqlx::{Row, SqlitePool};

pub async fn create_connection(
    pool: &SqlitePool,
    name: &str,
    database_type: DatabaseType,
    host: &str,
    port: u16,
    database: &str,
    username: &str,
    password: Option<&str>,
) -> Result<Connection, AppError> {
    let id = id_generator::generate_id_string();
    let now = Utc::now();
    let db_type_str = match database_type {
        DatabaseType::Postgres => "postgres",
        DatabaseType::Mysql => "mysql",
        DatabaseType::Sqlite => "sqlite",
    };

    let encrypted_password = password
        .filter(|p| !p.is_empty())
        .map(|p| crypto::encrypt(p).unwrap_or_else(|_| p.to_string()));

    sqlx::query(
        r#"
        INSERT INTO connections (id, name, database_type, host, port, database, username, password, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(id.clone())
    .bind(name)
    .bind(db_type_str)
    .bind(host)
    .bind(port as i32)
    .bind(database)
    .bind(username)
    .bind(&encrypted_password)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    get_connection(pool, id).await
}

pub async fn list_connections(pool: &SqlitePool) -> Result<Vec<Connection>, AppError> {
    let rows = sqlx::query(
        r#"
        SELECT id, name, database_type, host, port, database, username, password, created_at, updated_at
        FROM connections
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(row_to_connection).collect())
}

pub async fn get_connection(pool: &SqlitePool, id: String) -> Result<Connection, AppError> {
    let row = sqlx::query(
        r#"
        SELECT id, name, database_type, host, port, database, username, password, created_at, updated_at
        FROM connections
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    row.map(row_to_connection)
        .ok_or(AppError::ConnectionNotFound)
}

pub async fn update_connection(
    pool: &SqlitePool,
    id: String,
    name: Option<&str>,
    host: Option<&str>,
    port: Option<i32>,
    database: Option<&str>,
    username: Option<&str>,
    password: Option<&str>,
) -> Result<Connection, AppError> {
    let now = Utc::now();

    let existing = get_connection(pool, id.clone()).await?;

    let new_name = name.unwrap_or(&existing.name);
    let new_host = host.unwrap_or(&existing.host);
    let new_port = port.unwrap_or(existing.port as i32);
    let new_database = database.unwrap_or(&existing.database);
    let new_username = username.unwrap_or(&existing.username);

    let encrypted_password = if password.is_some() {
        password
            .filter(|p| !p.is_empty())
            .map(|p| crypto::encrypt(p).unwrap_or_else(|_| p.to_string()))
    } else {
        existing.password
    };

    let result = sqlx::query(
        r#"
        UPDATE connections
        SET name = ?, host = ?, port = ?, database = ?, username = ?, password = ?, updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(new_name)
    .bind(new_host)
    .bind(new_port)
    .bind(new_database)
    .bind(new_username)
    .bind(&encrypted_password)
    .bind(now)
    .bind(&id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::ConnectionNotFound);
    }

    get_connection(pool, id).await
}

pub async fn delete_connection(pool: &SqlitePool, id: String) -> Result<bool, AppError> {
    let result = sqlx::query("DELETE FROM connections WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

fn row_to_connection(row: sqlx::sqlite::SqliteRow) -> Connection {
    let db_type_str: String = row.get("database_type");
    let database_type = match db_type_str.as_str() {
        "mysql" => DatabaseType::Mysql,
        "sqlite" => DatabaseType::Sqlite,
        _ => DatabaseType::Postgres,
    };

    let encrypted_password: Option<String> = row.get("password");
    let password = encrypted_password.map(|p| crypto::decrypt(&p).unwrap_or(p));

    Connection {
        id: row.get("id"),
        name: row.get("name"),
        database_type,
        host: row.get("host"),
        port: row.get::<i32, _>("port") as u16,
        database: row.get("database"),
        username: row.get("username"),
        password,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}
