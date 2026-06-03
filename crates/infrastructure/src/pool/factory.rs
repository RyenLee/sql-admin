use async_trait::async_trait;
use redb::{ReadableDatabase, ReadableTable, ReadableTableMetadata, TableHandle};
use sql_admin_domain::connection::value_objects::DatabaseType;
use sql_admin_domain::shared::infrastructure_error::InfrastructureError;
use sql_admin_domain::shared::pool::{
    PoolFactory as DomainPoolFactory, QueryExecutor, RedbExecutor,
};
use std::sync::Arc;

use super::cached_pool::CachedPoolManager;
use super::executor::DynQueryExecutor;

#[derive(Clone)]
pub enum DynPool {
    Postgres(sqlx::postgres::PgPool),
    Mysql(sqlx::mysql::MySqlPool),
    Sqlite(sqlx::sqlite::SqlitePool),
    Redb(Arc<redb::Database>),
}

#[async_trait]
pub trait PoolFactory: Send + Sync {
    async fn create_pool(
        &self,
        database_type: &DatabaseType,
        host: &str,
        port: u16,
        database: &str,
        username: &str,
        password: &str,
    ) -> Result<DynPool, InfrastructureError>;
}

pub struct DefaultPoolFactory;

#[async_trait]
impl PoolFactory for DefaultPoolFactory {
    async fn create_pool(
        &self,
        database_type: &DatabaseType,
        host: &str,
        port: u16,
        database: &str,
        username: &str,
        password: &str,
    ) -> Result<DynPool, InfrastructureError> {
        match database_type {
            DatabaseType::Postgres => {
                let conn_str = format!(
                    "postgres://{}:{}@{}:{}/{}",
                    username, password, host, port, database
                );
                let p = sqlx::postgres::PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&conn_str)
                    .await
                    .map_err(|e| {
                        InfrastructureError::PoolCreationFailed(format!(
                            "Postgres pool creation failed: {}",
                            e
                        ))
                    })?;
                Ok(DynPool::Postgres(p))
            }
            DatabaseType::Mysql => {
                let conn_str = format!(
                    "mysql://{}:{}@{}:{}/{}",
                    username, password, host, port, database
                );
                let p = sqlx::mysql::MySqlPoolOptions::new()
                    .max_connections(5)
                    .connect(&conn_str)
                    .await
                    .map_err(|e| {
                        InfrastructureError::PoolCreationFailed(format!(
                            "MySQL pool creation failed: {}",
                            e
                        ))
                    })?;
                Ok(DynPool::Mysql(p))
            }
            DatabaseType::Sqlite => {
                let conn_str = format!("sqlite://{}?mode=rwc", database);
                let p = sqlx::sqlite::SqlitePoolOptions::new()
                    .max_connections(5)
                    .connect(&conn_str)
                    .await
                    .map_err(|e| {
                        InfrastructureError::PoolCreationFailed(format!(
                            "SQLite pool creation failed: {}",
                            e
                        ))
                    })?;
                Ok(DynPool::Sqlite(p))
            }
            DatabaseType::Redb => {
                let path = database.to_string();
                let db = tokio::task::spawn_blocking(move || {
                    redb::Database::open(path).map_err(|e| {
                        InfrastructureError::PoolCreationFailed(format!(
                            "Redb database open failed: {}",
                            e
                        ))
                    })
                })
                .await
                .map_err(|e| {
                    InfrastructureError::PoolCreationFailed(format!("Spawn blocking failed: {}", e))
                })??;
                Ok(DynPool::Redb(Arc::new(db)))
            }
        }
    }
}

pub struct CachedDomainPoolFactory {
    manager: Arc<CachedPoolManager<DefaultPoolFactory>>,
}

impl Default for CachedDomainPoolFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl CachedDomainPoolFactory {
    pub fn new() -> Self {
        Self {
            manager: Arc::new(CachedPoolManager::new(DefaultPoolFactory)),
        }
    }

    pub fn invalidate(&self, connection_id: &str) {
        let manager = self.manager.clone();
        let cid = connection_id.to_string();
        tokio::spawn(async move {
            manager.invalidate(&cid).await;
        });
    }

    async fn get_redb_db(
        &self,
        connection_id: &str,
    ) -> Result<Arc<redb::Database>, InfrastructureError> {
        let dyn_pool = self.manager.get_cached_or_err(connection_id).await?;
        match &dyn_pool {
            DynPool::Redb(db) => Ok(db.clone()),
            _ => Err(InfrastructureError::DatabaseError(
                "Redb operations only supported for Redb connections".to_string(),
            )),
        }
    }
}

#[async_trait]
impl DomainPoolFactory for CachedDomainPoolFactory {
    async fn create_pool(
        &self,
        connection_id: &str,
        database_type: &DatabaseType,
        host: &str,
        port: u16,
        database: &str,
        username: &str,
        password: &str,
    ) -> Result<Arc<dyn QueryExecutor>, InfrastructureError> {
        let dyn_pool = self
            .manager
            .get_or_create(
                connection_id,
                database_type,
                host,
                port,
                database,
                username,
                password,
            )
            .await?;
        Ok(Arc::new(DynQueryExecutor::new(dyn_pool)))
    }

    async fn invalidate_pool(&self, connection_id: &str) {
        self.invalidate(connection_id);
    }
}

#[async_trait]
impl RedbExecutor for CachedDomainPoolFactory {
    async fn list_redb_tables(
        &self,
        connection_id: &str,
    ) -> Result<Vec<(String, u64)>, InfrastructureError> {
        let db = self.get_redb_db(connection_id).await?;

        let result = tokio::task::spawn_blocking(
            move || -> Result<Vec<(String, u64)>, InfrastructureError> {
                let read_txn = db.begin_read().map_err(|e| {
                    InfrastructureError::DatabaseError(format!("Redb begin read failed: {}", e))
                })?;

                let table_list = read_txn.list_tables().map_err(|e| {
                    InfrastructureError::DatabaseError(format!("Redb list tables failed: {}", e))
                })?;

                let mut tables = Vec::new();
                for handle in table_list {
                    let table_name = handle.name().to_string();
                    let key_count = redb_try_key_count(&read_txn, &table_name);
                    tables.push((table_name, key_count));
                }

                Ok(tables)
            },
        )
        .await
        .map_err(|e| {
            InfrastructureError::DatabaseError(format!("Spawn blocking failed: {}", e))
        })??;

        Ok(result)
    }

    async fn query_redb_keys(
        &self,
        connection_id: &str,
        table: &str,
        key_prefix: Option<&str>,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<(String, serde_json::Value)>, InfrastructureError> {
        let db = self.get_redb_db(connection_id).await?;

        let table_name = table.to_string();
        let prefix = key_prefix.map(|s| s.to_string());
        let result = tokio::task::spawn_blocking(
            move || -> Result<Vec<(String, serde_json::Value)>, InfrastructureError> {
                let read_txn = db.begin_read().map_err(|e| {
                    InfrastructureError::DatabaseError(format!("Redb begin read failed: {}", e))
                })?;

                redb_try_query_table(&read_txn, &table_name, prefix.as_deref(), limit, offset)
            },
        )
        .await
        .map_err(|e| {
            InfrastructureError::DatabaseError(format!("Spawn blocking failed: {}", e))
        })??;

        Ok(result)
    }

    async fn edit_redb_key(
        &self,
        connection_id: &str,
        table: &str,
        key: &str,
        value: serde_json::Value,
    ) -> Result<(), InfrastructureError> {
        let db = self.get_redb_db(connection_id).await?;

        let table_name = table.to_string();
        let key_str = key.to_string();
        let val_str = match &value {
            serde_json::Value::String(s) => s.clone(),
            other => other.to_string(),
        };

        tokio::task::spawn_blocking(move || -> Result<(), InfrastructureError> {
            let write_txn = db.begin_write().map_err(|e| {
                InfrastructureError::DatabaseError(format!("Redb begin write failed: {}", e))
            })?;
            {
                let mut table_def = write_txn
                    .open_table(redb::TableDefinition::<&str, &str>::new(&table_name))
                    .map_err(|_| {
                        InfrastructureError::DatabaseError(format!(
                            "Redb table '{}' not found",
                            table_name
                        ))
                    })?;

                table_def
                    .insert(
                        &*Box::leak(key_str.clone().into_boxed_str()),
                        &*Box::leak(val_str.clone().into_boxed_str()),
                    )
                    .map_err(|e| {
                        InfrastructureError::DatabaseError(format!("Redb insert failed: {}", e))
                    })?;
            }
            write_txn.commit().map_err(|e| {
                InfrastructureError::DatabaseError(format!("Redb commit failed: {}", e))
            })?;
            Ok(())
        })
        .await
        .map_err(|e| {
            InfrastructureError::DatabaseError(format!("Spawn blocking failed: {}", e))
        })??;

        Ok(())
    }

    async fn batch_delete_redb_keys(
        &self,
        connection_id: &str,
        table: &str,
        keys: &[String],
    ) -> Result<u64, InfrastructureError> {
        let db = self.get_redb_db(connection_id).await?;

        let table_name = table.to_string();
        let keys_to_delete: Vec<String> = keys.to_vec();

        let deleted = tokio::task::spawn_blocking(move || -> Result<u64, InfrastructureError> {
            let write_txn = db.begin_write().map_err(|e| {
                InfrastructureError::DatabaseError(format!("Redb begin write failed: {}", e))
            })?;
            let mut count: u64 = 0;
            {
                let mut table_def = write_txn
                    .open_table(redb::TableDefinition::<&str, &str>::new(&table_name))
                    .map_err(|_| {
                        InfrastructureError::DatabaseError(format!(
                            "Redb table '{}' not found",
                            table_name
                        ))
                    })?;

                for key in &keys_to_delete {
                    let removed = table_def.remove(key.as_str()).map_err(|e| {
                        InfrastructureError::DatabaseError(format!("Redb remove failed: {}", e))
                    })?;
                    if removed.is_some() {
                        count += 1;
                    }
                }
            }
            write_txn.commit().map_err(|e| {
                InfrastructureError::DatabaseError(format!("Redb commit failed: {}", e))
            })?;
            Ok(count)
        })
        .await
        .map_err(|e| {
            InfrastructureError::DatabaseError(format!("Spawn blocking failed: {}", e))
        })??;

        Ok(deleted)
    }
}

/// Detect the actual key/value type names of a redb table by triggering a TableTypeMismatch error.
fn redb_detect_table_types(
    read_txn: &redb::ReadTransaction,
    table_name: &str,
) -> Result<(String, String), InfrastructureError> {
    // Use a dummy type to trigger TableTypeMismatch which reveals the actual types
    match read_txn.open_table(redb::TableDefinition::<&str, &str>::new(table_name)) {
        Ok(_) => Ok(("&str".to_string(), "&str".to_string())),
        Err(redb::TableError::TableTypeMismatch { key, value, .. }) => {
            let key_name = key.name().to_string();
            let value_name = value.name().to_string();
            tracing::info!(
                table_name = %table_name,
                key_type = %key_name,
                value_type = %value_name,
                "Detected redb table types from mismatch error"
            );
            Ok((key_name, value_name))
        }
        Err(redb::TableError::TableIsMultimap(_)) => {
            Err(InfrastructureError::DatabaseError(format!(
                "Redb table '{}' is a multimap table, which is not supported",
                table_name
            )))
        }
        Err(redb::TableError::TableDoesNotExist(_)) => Err(InfrastructureError::DatabaseError(
            format!("Redb table '{}' does not exist", table_name),
        )),
        Err(e) => Err(InfrastructureError::DatabaseError(format!(
            "Redb error detecting types for table '{}': {}",
            table_name, e
        ))),
    }
}

/// Try to open a redb table and get its key count.
/// First detects the actual types, then opens with the correct type.
fn redb_try_key_count(read_txn: &redb::ReadTransaction, table_name: &str) -> u64 {
    // Try untyped first for len()
    if let Ok(handles) = read_txn.list_tables() {
        for handle in handles {
            if handle.name() == table_name {
                if let Ok(untyped) = read_txn.open_untyped_table(handle) {
                    return untyped.len().unwrap_or(0);
                }
            }
        }
    }
    0
}

/// Try to query a redb table by detecting its actual types first, then opening with the correct type.
fn redb_try_query_table(
    read_txn: &redb::ReadTransaction,
    table_name: &str,
    prefix: Option<&str>,
    limit: u64,
    offset: u64,
) -> Result<Vec<(String, serde_json::Value)>, InfrastructureError> {
    // Step 1: Detect actual key/value type names
    let (key_type_name, value_type_name) = redb_detect_table_types(read_txn, table_name)?;

    tracing::info!(
        table_name = %table_name,
        key_type = %key_type_name,
        value_type = %value_type_name,
        "Attempting to open redb table with detected types"
    );

    // Step 2: Try to open with the detected types
    macro_rules! try_match {
        ($k:ty, $v:ty, $k_name:expr, $v_name:expr) => {
            if key_type_name == $k_name && value_type_name == $v_name {
                return redb_query_with_types::<$k, $v>(
                    read_txn, table_name, prefix, limit, offset,
                );
            }
        };
    }

    // Key/Value type combinations with their redb type names
    // All reasonable combinations of common redb Key/Value types
    try_match!(&str, &str, "&str", "&str");
    try_match!(&str, &[u8], "&str", "&[u8]");
    try_match!(&str, Vec<u8>, "&str", "alloc::vec::Vec<u8>");
    try_match!(&str, u64, "&str", "u64");
    try_match!(&str, i64, "&str", "i64");
    try_match!(&str, u32, "&str", "u32");
    try_match!(&str, i32, "&str", "i32");
    try_match!(&str, f64, "&str", "f64");
    try_match!(&str, bool, "&str", "bool");
    try_match!(&[u8], &[u8], "&[u8]", "&[u8]");
    try_match!(&[u8], &str, "&[u8]", "&str");
    try_match!(&[u8], Vec<u8>, "&[u8]", "alloc::vec::Vec<u8>");
    try_match!(&[u8], u64, "&[u8]", "u64");
    try_match!(&[u8], i64, "&[u8]", "i64");
    try_match!(&[u8], u32, "&[u8]", "u32");
    try_match!(&[u8], i32, "&[u8]", "i32");
    try_match!(&[u8], u128, "&[u8]", "u128");
    try_match!(&[u8], f64, "&[u8]", "f64");
    try_match!(&[u8], bool, "&[u8]", "bool");
    try_match!(u128, &str, "u128", "&str");
    try_match!(u128, &[u8], "u128", "&[u8]");
    try_match!(u128, Vec<u8>, "u128", "alloc::vec::Vec<u8>");
    try_match!(u128, u64, "u128", "u64");
    try_match!(u128, i64, "u128", "i64");
    try_match!(u128, u32, "u128", "u32");
    try_match!(u128, u128, "u128", "u128");
    try_match!(u64, &str, "u64", "&str");
    try_match!(u64, &[u8], "u64", "&[u8]");
    try_match!(u64, Vec<u8>, "u64", "alloc::vec::Vec<u8>");
    try_match!(u64, u64, "u64", "u64");
    try_match!(u64, i64, "u64", "i64");
    try_match!(u64, u32, "u64", "u32");
    try_match!(u64, u128, "u64", "u128");
    try_match!(u32, &str, "u32", "&str");
    try_match!(u32, &[u8], "u32", "&[u8]");
    try_match!(u32, Vec<u8>, "u32", "alloc::vec::Vec<u8>");
    try_match!(u32, u64, "u32", "u64");
    try_match!(u32, u32, "u32", "u32");
    try_match!(i64, &str, "i64", "&str");
    try_match!(i64, &[u8], "i64", "&[u8]");
    try_match!(i64, Vec<u8>, "i64", "alloc::vec::Vec<u8>");
    try_match!(i64, i64, "i64", "i64");
    try_match!(i64, u64, "i64", "u64");
    try_match!(i32, &str, "i32", "&str");
    try_match!(i32, &[u8], "i32", "&[u8]");
    try_match!(i32, Vec<u8>, "i32", "alloc::vec::Vec<u8>");
    try_match!(i32, i32, "i32", "i32");
    try_match!(String, &str, "alloc::string::String", "&str");
    try_match!(String, &[u8], "alloc::string::String", "&[u8]");
    try_match!(
        String,
        String,
        "alloc::string::String",
        "alloc::string::String"
    );
    try_match!(
        String,
        Vec<u8>,
        "alloc::string::String",
        "alloc::vec::Vec<u8>"
    );
    try_match!(String, u64, "alloc::string::String", "u64");
    try_match!(String, i64, "alloc::string::String", "i64");

    // If no exact match, try brute-force approach
    macro_rules! try_query {
        ($k:ty, $v:ty) => {
            if let Ok(result) =
                redb_query_with_types::<$k, $v>(read_txn, table_name, prefix, limit, offset)
            {
                return Ok(result);
            }
        };
    }

    try_query!(&str, &str);
    try_query!(&str, Vec<u8>);
    try_query!(&str, u64);
    try_query!(&str, i64);
    try_query!(&[u8], &[u8]);
    try_query!(&[u8], &str);
    try_query!(&[u8], Vec<u8>);
    try_query!(u128, Vec<u8>);
    try_query!(u128, &str);
    try_query!(u64, Vec<u8>);
    try_query!(u64, &str);
    try_query!(u32, Vec<u8>);
    try_query!(i64, Vec<u8>);
    try_query!(i32, Vec<u8>);
    try_query!(String, String);
    try_query!(String, Vec<u8>);

    Err(InfrastructureError::DatabaseError(format!(
        "Redb table '{}' has unsupported types: key={}, value={}. \
         Please add this type combination to the supported list.",
        table_name, key_type_name, value_type_name
    )))
}

fn redb_query_with_types<K, V>(
    read_txn: &redb::ReadTransaction,
    table_name: &str,
    prefix: Option<&str>,
    limit: u64,
    offset: u64,
) -> Result<Vec<(String, serde_json::Value)>, InfrastructureError>
where
    K: redb::Key + std::fmt::Debug + 'static,
    V: redb::Value + std::fmt::Debug + 'static,
{
    let table_def = read_txn
        .open_table(redb::TableDefinition::<K, V>::new(table_name))
        .map_err(|e| {
            InfrastructureError::DatabaseError(format!("Redb open_table failed: {}", e))
        })?;

    let mut iter = table_def
        .iter()
        .map_err(|e| InfrastructureError::DatabaseError(format!("Redb iteration failed: {}", e)))?;

    let mut all: Vec<(String, String)> = Vec::new();
    for item in iter.by_ref() {
        let (k, v) = item.map_err(|e| {
            InfrastructureError::DatabaseError(format!("Redb storage error: {}", e))
        })?;
        let key_debug = format!("{:?}", k.value());
        // Strip surrounding quotes for string-like keys (Debug adds quotes around &str/String)
        let key_str =
            if key_debug.starts_with('"') && key_debug.ends_with('"') && key_debug.len() >= 2 {
                key_debug[1..key_debug.len() - 1].to_string()
            } else {
                key_debug
            };
        let val_str = format!("{:?}", v.value());
        if let Some(p) = prefix
            && !key_str.starts_with(p)
        {
            continue;
        }
        all.push((key_str, val_str));
    }
    drop(iter);

    let total = all.len() as u64;
    let start = offset.min(total);
    let end = (offset + limit).min(total);
    let results: Vec<(String, serde_json::Value)> = all[start as usize..end as usize]
        .iter()
        .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
        .collect();

    Ok(results)
}
