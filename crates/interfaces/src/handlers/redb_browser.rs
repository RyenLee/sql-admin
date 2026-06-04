use axum::{
    extract::{Path, State},
    Json,
};
use sql_admin_api_types::{ApiResponse, RedbQueryRequest, RedbEditRequest, RedbBatchDeleteRequest, RedbTableSummary, RedbKeyList, RedbKeyValue};

use crate::error::AppResult;
use crate::state::AppState;

pub async fn list_tables(
    State(state): State<AppState>,
    Path(conn_id): Path<String>,
) -> AppResult<Json<ApiResponse<Vec<RedbTableSummary>>>> {
    let result = state.redb_handler.list_tables(&conn_id).await?;
    let tables: Vec<RedbTableSummary> = result
        .into_iter()
        .map(|(name, key_count, stored_bytes)| RedbTableSummary {
            name,
            key_count,
            total_value_bytes: stored_bytes,
        })
        .collect();
    Ok(Json(ApiResponse::ok(tables)))
}

pub async fn query_keys(
    State(state): State<AppState>,
    Path(conn_id): Path<String>,
    Json(req): Json<RedbQueryRequest>,
) -> AppResult<Json<ApiResponse<RedbKeyList>>> {
    let table = &req.table;
    let limit = req.limit;
    let offset = req.offset;
    let key_prefix = req.key_prefix.as_deref();
    // Query limit+1 to accurately determine has_more (consistent with Desktop command)
    let result = state.redb_handler.query_keys(&conn_id, table, key_prefix, limit + 1, offset).await?;
    let has_more = result.len() > limit as usize;
    let keys: Vec<RedbKeyValue> = result
        .into_iter()
        .take(limit as usize)
        .map(|(k, v)| RedbKeyValue {
            key: k.as_str().to_string(),
            value: v.as_json().clone(),
            value_type: match v.as_json() {
                serde_json::Value::Null => "null".to_string(),
                serde_json::Value::Bool(_) => "boolean".to_string(),
                serde_json::Value::Number(_) => "number".to_string(),
                serde_json::Value::String(_) => "string".to_string(),
                serde_json::Value::Array(_) => "array".to_string(),
                serde_json::Value::Object(_) => "object".to_string(),
            },
        })
        .collect();
    let total = keys.len() as u64;
    Ok(Json(ApiResponse::ok(RedbKeyList { keys, total, has_more })))
}

pub async fn edit_key(
    State(state): State<AppState>,
    Path(conn_id): Path<String>,
    Json(req): Json<RedbEditRequest>,
) -> AppResult<Json<ApiResponse<()>>> {
    let table = &req.table;
    let key_str = &req.key;
    let new_value = req.new_value.clone();

    let key = sql_admin_domain::redb::value_objects::RedbKey::new(key_str.clone());
    let value = sql_admin_domain::redb::value_objects::RedbValue::new(new_value.unwrap_or(serde_json::Value::Null));

    state.redb_handler.edit_key(&conn_id, table, key, value).await?;
    Ok(Json(ApiResponse::ok(())))
}

pub async fn batch_delete(
    State(state): State<AppState>,
    Path(conn_id): Path<String>,
    Json(req): Json<RedbBatchDeleteRequest>,
) -> AppResult<Json<ApiResponse<u64>>> {
    let table = &req.table;
    let keys: Vec<sql_admin_domain::redb::value_objects::RedbKey> = req
        .keys
        .into_iter()
        .map(sql_admin_domain::redb::value_objects::RedbKey::new)
        .collect();

    let result = state.redb_handler.batch_delete_keys(&conn_id, table, keys).await?;
    Ok(Json(ApiResponse::ok(result)))
}