use crate::state::AppState;
use sql_admin_api_types::*;
use sql_admin_domain::redb::value_objects::{RedbKey, RedbValue};
use tauri::State;

#[tauri::command]
pub async fn list_redb_tables(
    state: State<'_, AppState>,
    connection_id: String,
) -> Result<Vec<RedbTableSummary>, String> {
    let tables = state
        .redb_handler
        .list_tables(&connection_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(tables
        .into_iter()
        .map(|(name, key_count, stored_bytes)| RedbTableSummary {
            name,
            key_count,
            total_value_bytes: stored_bytes,
        })
        .collect())
}

#[tauri::command]
pub async fn query_redb_keys(
    state: State<'_, AppState>,
    connection_id: String,
    request: RedbQueryRequest,
) -> Result<RedbKeyList, String> {
    let keys = state
        .redb_handler
        .query_keys(
            &connection_id,
            &request.table,
            request.key_prefix.as_deref(),
            request.limit + 1, // Query one extra to determine has_more
            request.offset,
        )
        .await
        .map_err(|e| e.to_string())?;

    let has_more = keys.len() > request.limit as usize;
    let mut items: Vec<RedbKeyValue> = keys
        .into_iter()
        .take(request.limit as usize)
        .map(|(k, v)| RedbKeyValue {
            key: k.into_inner(),
            value: v.into_inner(),
            value_type: "json".to_string(),
        })
        .collect();

    // Client-side key_pattern filtering (server only supports prefix)
    if let Some(pattern) = &request.key_pattern {
        let regex = regex::Regex::new(pattern).ok();
        items.retain(|kv| match &regex {
            Some(re) => re.is_match(&kv.key),
            None => kv.key.contains(pattern),
        });
    }

    // NOTE: total is approximate (current page count). Redb does not provide
    // a count query, so we can only report the items returned in this page.
    // When has_more is true, the actual total is larger.
    let total = items.len() as u64;

    Ok(RedbKeyList {
        keys: items,
        total,
        has_more,
    })
}

#[tauri::command]
pub async fn edit_redb_key(
    state: State<'_, AppState>,
    connection_id: String,
    request: RedbEditRequest,
) -> Result<(), String> {
    let redb_key = RedbKey::new(request.key);
    let redb_value = match request.new_value {
        Some(v) => RedbValue::new(v),
        None => {
            return state
                .redb_handler
                .delete_key(&connection_id, &request.table, redb_key)
                .await
                .map_err(|e| e.to_string())
                .map(|_| ());
        }
    };
    state
        .redb_handler
        .edit_key(&connection_id, &request.table, redb_key, redb_value)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn batch_delete_redb_keys(
    state: State<'_, AppState>,
    connection_id: String,
    request: RedbBatchDeleteRequest,
) -> Result<u64, String> {
    let redb_keys: Vec<RedbKey> = request.keys.into_iter().map(RedbKey::new).collect();
    state
        .redb_handler
        .batch_delete_keys(&connection_id, &request.table, redb_keys)
        .await
        .map_err(|e| e.to_string())
}
