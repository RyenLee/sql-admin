use crate::config::AppConfig;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub enum DynPool {
    Postgres(sqlx::postgres::PgPool),
    MySql(sqlx::mysql::MySqlPool),
    Sqlite(sqlx::sqlite::SqlitePool),
}

#[derive(Clone)]
pub struct AppState {
    pub db_pool: SqlitePool,
    #[allow(dead_code)]
    pub config: AppConfig,
    pub connection_pools: Arc<RwLock<HashMap<String, DynPool>>>,
}
