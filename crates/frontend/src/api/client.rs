use sql_admin_api_types::{
    ApiResponse, Connection, CreateConnectionRequest, DeleteRowRequest, EditRowRequest,
    ExecuteQueryRequest, ImportResult, ImportSqlRequest, InsertRowRequest, QueryHistory,
    QueryResult, RedbBatchDeleteRequest, RedbEditRequest, RedbKeyList, RedbQueryRequest,
    RedbTableSummary, SaveQueryHistoryRequest, SchemaInfo, TableDef, UpdateConnectionRequest,
};

const API_BASE: &str = "/api";

// ---------------------------------------------------------------------------
// Runtime detection
// ---------------------------------------------------------------------------

/// Check if running inside a Tauri WebView by detecting `window.__TAURI__`
#[cfg(all(feature = "tauri", target_arch = "wasm32"))]
fn is_tauri() -> bool {
    use wasm_bindgen::JsValue;
    let window = match web_sys::window() {
        Some(w) => w,
        None => return false,
    };
    js_sys::Reflect::has(&window, &JsValue::from_str("__TAURI__")).unwrap_or(false)
}

// ---------------------------------------------------------------------------
// Tauri IPC helper
// ---------------------------------------------------------------------------

/// Convert a JsValue error to a human-readable String
#[cfg(all(feature = "tauri", target_arch = "wasm32"))]
fn js_error_to_string(e: wasm_bindgen::JsValue) -> String {
    use wasm_bindgen::JsCast;
    if let Some(s) = e.as_string() {
        return s;
    }
    if let Some(obj) = e.dyn_ref::<js_sys::Object>() {
        if let Ok(msg) = js_sys::Reflect::get(obj, &wasm_bindgen::JsValue::from_str("message")) {
            if let Some(s) = msg.as_string() {
                return s;
            }
        }
    }
    format!("{:?}", e)
}

/// Invoke a Tauri command via `window.__TAURI__.core.invoke()`
#[cfg(all(feature = "tauri", target_arch = "wasm32"))]
async fn tauri_invoke<T: serde::de::DeserializeOwned>(
    cmd: &str,
    args: serde_json::Value,
) -> Result<T, String> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;

    let window = web_sys::window().ok_or("No window")?;
    let tauri_obj = js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str("__TAURI__"))
        .map_err(|_| "Not in Tauri environment".to_string())?;
    let core_obj = js_sys::Reflect::get(&tauri_obj, &wasm_bindgen::JsValue::from_str("core"))
        .map_err(|_| "Tauri core not found".to_string())?;
    let invoke_fn =
        js_sys::Reflect::get(&core_obj, &wasm_bindgen::JsValue::from_str("invoke"))
            .map_err(|_| "Tauri invoke not found".to_string())?;

    let args_json =
        serde_json::to_string(&args).map_err(|e| format!("Serialization error: {}", e))?;
    let args_js =
        js_sys::JSON::parse(&args_json).map_err(|e| format!("JSON parse error: {:?}", e))?;

    let invoke_fn: js_sys::Function = invoke_fn
        .dyn_into()
        .map_err(|_| "invoke is not a function".to_string())?;
    let result_promise = invoke_fn
        .call2(&core_obj, &wasm_bindgen::JsValue::from_str(cmd), &args_js)
        .map_err(|e| format!("Invoke call error: {:?}", e))?;

    let result_js = JsFuture::from(js_sys::Promise::from(result_promise))
        .await
        .map_err(js_error_to_string)?;

    // Handle void/undefined results (commands returning ())
    if result_js.is_undefined() || result_js.is_null() {
        return serde_json::from_str("null").map_err(|e| format!("Deserialization error: {}", e));
    }

    let result_json = js_sys::JSON::stringify(&result_js)
        .map_err(|e| format!("JSON stringify error: {:?}", e))?
        .as_string()
        .ok_or("Result is not a string".to_string())?;

    serde_json::from_str(&result_json).map_err(|e| format!("Deserialization error: {}", e))
}

// ---------------------------------------------------------------------------
// HTTP helpers
// ---------------------------------------------------------------------------

async fn parse_api_response<T: serde::de::DeserializeOwned>(
    response: gloo_net::http::Response,
) -> Result<T, String> {
    if !response.ok() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("HTTP {}: {}", status, body));
    }
    let api_response: ApiResponse<T> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;
    api_response
        .data
        .ok_or_else(|| api_response.error.unwrap_or_else(|| "Unknown error".to_string()))
}

// ---------------------------------------------------------------------------
// Connection commands
// ---------------------------------------------------------------------------

pub async fn list_connections() -> Result<Vec<Connection>, String> {
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    if is_tauri() {
        return tauri_invoke("list_connections", serde_json::json!({})).await;
    }
    let response = gloo_net::http::Request::get(&format!("{}/connections", API_BASE))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    parse_api_response(response).await
}

pub async fn create_connection(req: CreateConnectionRequest) -> Result<Connection, String> {
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    if is_tauri() {
        return tauri_invoke("create_connection", serde_json::json!({ "request": req })).await;
    }
    let response = gloo_net::http::Request::post(&format!("{}/connections", API_BASE))
        .json(&req)
        .map_err(|e| format!("Serialization error: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    parse_api_response(response).await
}

#[allow(dead_code)]
pub async fn get_connection(id: String) -> Result<Connection, String> {
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    if is_tauri() {
        return tauri_invoke("get_connection", serde_json::json!({ "id": id })).await;
    }
    let response = gloo_net::http::Request::get(&format!("{}/connections/{}", API_BASE, id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    parse_api_response(response).await
}

pub async fn update_connection(
    id: String,
    req: CreateConnectionRequest,
) -> Result<Connection, String> {
    let update_req = UpdateConnectionRequest {
        name: Some(req.name),
        host: Some(req.host),
        port: Some(req.port),
        database: Some(req.database),
        username: Some(req.username),
        password: req.password,
    };
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    if is_tauri() {
        return tauri_invoke(
            "update_connection",
            serde_json::json!({ "id": id, "request": update_req }),
        )
        .await;
    }
    let response = gloo_net::http::Request::put(&format!("{}/connections/{}", API_BASE, id))
        .json(&update_req)
        .map_err(|e| format!("Serialization error: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    parse_api_response(response).await
}

pub async fn delete_connection(id: String) -> Result<bool, String> {
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    if is_tauri() {
        return tauri_invoke("delete_connection", serde_json::json!({ "id": id })).await;
    }
    let response = gloo_net::http::Request::delete(&format!("{}/connections/{}", API_BASE, id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    parse_api_response(response).await
}

pub async fn test_connection(id: String) -> Result<String, String> {
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    if is_tauri() {
        return tauri_invoke("test_connection", serde_json::json!({ "id": id })).await;
    }
    let response = gloo_net::http::Request::post(&format!("{}/connections/{}/test", API_BASE, id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    parse_api_response(response).await
}

pub async fn test_connection_request(req: CreateConnectionRequest) -> Result<String, String> {
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    if is_tauri() {
        return tauri_invoke("test_connection_request", serde_json::json!({ "request": req })).await;
    }
    let response = gloo_net::http::Request::post(&format!("{}/connections/test", API_BASE))
        .json(&req)
        .map_err(|e| format!("Serialization error: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    parse_api_response(response).await
}

// ---------------------------------------------------------------------------
// Query commands
// ---------------------------------------------------------------------------

pub async fn execute_query(req: ExecuteQueryRequest) -> Result<QueryResult, String> {
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    if is_tauri() {
        return tauri_invoke("execute_query", serde_json::json!({ "request": req })).await;
    }
    let response = gloo_net::http::Request::post(&format!("{}/query", API_BASE))
        .json(&req)
        .map_err(|e| format!("Serialization error: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    parse_api_response(response).await
}

pub async fn get_table_data(
    id: String,
    table: &str,
    limit: i64,
    offset: i64,
) -> Result<QueryResult, String> {
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    if is_tauri() {
        return tauri_invoke(
            "get_table_data",
            serde_json::json!({
                "request": {
                    "connection_id": id,
                    "table": table,
                    "limit": limit,
                    "offset": offset,
                }
            }),
        )
        .await;
    }
    let response = gloo_net::http::Request::get(&format!(
        "{}/connections/{}/tables/{}/data?limit={}&offset={}",
        API_BASE, id, table, limit, offset
    ))
    .send()
    .await
    .map_err(|e| format!("Network error: {}", e))?;
    parse_api_response(response).await
}

// ---------------------------------------------------------------------------
// Schema commands
// ---------------------------------------------------------------------------

pub async fn get_schema(id: String) -> Result<SchemaInfo, String> {
    leptos::logging::log!("info[API Request] get_schema - id: {}", id);
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    if is_tauri() {
        return tauri_invoke("get_schema", serde_json::json!({ "connectionId": id })).await;
    }
    let response = gloo_net::http::Request::get(&format!("{}/connections/{}/schema", API_BASE, id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    parse_api_response(response).await
}

pub async fn get_table_def(id: String, table: &str) -> Result<TableDef, String> {
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    if is_tauri() {
        return tauri_invoke(
            "get_table_def",
            serde_json::json!({ "connectionId": id, "tableName": table }),
        )
        .await;
    }
    let response = gloo_net::http::Request::get(&format!(
        "{}/connections/{}/tables/{}/def",
        API_BASE, id, table
    ))
    .send()
    .await
    .map_err(|e| format!("Network error: {}", e))?;
    parse_api_response(response).await
}

// ---------------------------------------------------------------------------
// History commands
// ---------------------------------------------------------------------------

pub async fn get_query_history() -> Result<Vec<QueryHistory>, String> {
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    if is_tauri() {
        return tauri_invoke("get_query_history", serde_json::json!({})).await;
    }
    let response = gloo_net::http::Request::get(&format!("{}/history", API_BASE))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    parse_api_response(response).await
}

pub async fn save_query_history(req: SaveQueryHistoryRequest) -> Result<QueryHistory, String> {
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    if is_tauri() {
        return tauri_invoke("save_query_history", serde_json::json!({ "request": req })).await;
    }
    let response = gloo_net::http::Request::post(&format!("{}/history", API_BASE))
        .json(&req)
        .map_err(|e| format!("Serialization error: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    parse_api_response(response).await
}

pub async fn clear_query_history() -> Result<(), String> {
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    if is_tauri() {
        return tauri_invoke("clear_query_history", serde_json::json!({})).await;
    }
    let response = gloo_net::http::Request::delete(&format!("{}/history", API_BASE))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    if !response.ok() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("HTTP {}: {}", status, body));
    }
    Ok(())
}

pub async fn delete_query_history_item(id: String) -> Result<(), String> {
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    if is_tauri() {
        let _: bool =
            tauri_invoke("delete_query_history_item", serde_json::json!({ "id": id })).await?;
        return Ok(());
    }
    let response = gloo_net::http::Request::delete(&format!("{}/history/{}", API_BASE, id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    if !response.ok() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("HTTP {}: {}", status, body));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Data edit commands
// ---------------------------------------------------------------------------

pub async fn edit_row(id: String, req: EditRowRequest) -> Result<QueryResult, String> {
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    if is_tauri() {
        return tauri_invoke("edit_row", serde_json::json!({ "connectionId": id, "request": req })).await;
    }
    let response =
        gloo_net::http::Request::post(&format!("{}/connections/{}/edit-row", API_BASE, id))
            .json(&req)
            .map_err(|e| format!("Serialization error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;
    parse_api_response(response).await
}

/// Delete a row from a table
pub async fn delete_row(id: String, req: DeleteRowRequest) -> Result<QueryResult, String> {
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    if is_tauri() {
        return tauri_invoke("delete_row", serde_json::json!({ "connectionId": id, "request": req })).await;
    }
    let response =
        gloo_net::http::Request::post(&format!("{}/connections/{}/delete-row", API_BASE, id))
            .json(&req)
            .map_err(|e| format!("Serialization error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;
    parse_api_response(response).await
}

/// Insert a row into a table
pub async fn insert_row(id: String, req: InsertRowRequest) -> Result<QueryResult, String> {
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    if is_tauri() {
        return tauri_invoke("insert_row", serde_json::json!({ "connectionId": id, "request": req })).await;
    }
    let response =
        gloo_net::http::Request::post(&format!("{}/connections/{}/insert-row", API_BASE, id))
            .json(&req)
            .map_err(|e| format!("Serialization error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;
    parse_api_response(response).await
}

// ---------------------------------------------------------------------------
// Import commands
// ---------------------------------------------------------------------------

#[allow(dead_code)]
pub async fn import_sql(id: String, req: ImportSqlRequest) -> Result<ImportResult, String> {
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    if is_tauri() {
        return tauri_invoke("import_sql", serde_json::json!({ "connectionId": id, "request": req })).await;
    }
    let response =
        gloo_net::http::Request::post(&format!("{}/connections/{}/import", API_BASE, id))
            .json(&req)
            .map_err(|e| format!("Serialization error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;
    parse_api_response(response).await
}

// ---------------------------------------------------------------------------
// Redb browser commands
// ---------------------------------------------------------------------------

pub async fn list_redb_tables(id: String) -> Result<Vec<RedbTableSummary>, String> {
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    if is_tauri() {
        return tauri_invoke("list_redb_tables", serde_json::json!({ "connectionId": id })).await;
    }
    let response =
        gloo_net::http::Request::get(&format!("{}/connections/{}/redb/tables", API_BASE, id))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;
    parse_api_response(response).await
}

pub async fn query_redb_keys(req: RedbQueryRequest) -> Result<RedbKeyList, String> {
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    if is_tauri() {
        let connection_id = req.connection_id.clone();
        return tauri_invoke(
            "query_redb_keys",
            serde_json::json!({
                "connectionId": connection_id,
                "request": req,
            }),
        )
        .await;
    }
    let response = gloo_net::http::Request::post(&format!(
        "{}/connections/{}/redb/query",
        API_BASE, req.connection_id
    ))
    .json(&req)
    .map_err(|e| format!("Serialization error: {}", e))?
    .send()
    .await
    .map_err(|e| format!("Network error: {}", e))?;
    parse_api_response(response).await
}

pub async fn edit_redb_key(req: RedbEditRequest) -> Result<String, String> {
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    if is_tauri() {
        let connection_id = req.connection_id.clone();
        let _: () = tauri_invoke(
            "edit_redb_key",
            serde_json::json!({
                "connectionId": connection_id,
                "request": req,
            }),
        )
        .await?;
        return Ok("ok".to_string());
    }
    let response = gloo_net::http::Request::post(&format!(
        "{}/connections/{}/redb/edit",
        API_BASE, req.connection_id
    ))
    .json(&req)
    .map_err(|e| format!("Serialization error: {}", e))?
    .send()
    .await
    .map_err(|e| format!("Network error: {}", e))?;
    parse_api_response(response).await
}

/// Batch delete Redb keys
pub async fn batch_delete_redb_keys(
    id: String,
    req: RedbBatchDeleteRequest,
) -> Result<u64, String> {
    #[cfg(all(feature = "tauri", target_arch = "wasm32"))]
    if is_tauri() {
        return tauri_invoke(
            "batch_delete_redb_keys",
            serde_json::json!({
                "connectionId": id,
                "request": req,
            }),
        )
        .await;
    }
    let response = gloo_net::http::Request::post(&format!(
        "{}/connections/{}/redb/batch-delete",
        API_BASE, id
    ))
    .json(&req)
    .map_err(|e| format!("Serialization error: {}", e))?
    .send()
    .await
    .map_err(|e| format!("Network error: {}", e))?;
    parse_api_response(response).await
}
