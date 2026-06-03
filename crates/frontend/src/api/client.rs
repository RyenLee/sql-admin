use sql_admin_api_types::{
    ApiResponse, Connection, CreateConnectionRequest, EditRowRequest,
    ExecuteQueryRequest, ImportResult, ImportSqlRequest, QueryHistory,
    QueryResult, RedbEditRequest, RedbKeyList, RedbQueryRequest,
    RedbTableSummary, SaveQueryHistoryRequest, SchemaInfo, TableDef, UpdateConnectionRequest,
};

const API_BASE: &str = "/api";

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

pub async fn list_connections() -> Result<Vec<Connection>, String> {
    leptos::logging::log!("info[API Request] list_connections");

    let response = gloo_net::http::Request::get(&format!("{}/connections", API_BASE))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    parse_api_response(response).await
}

pub async fn list_redb_tables(id: String) -> Result<Vec<RedbTableSummary>, String> {
    let response =
        gloo_net::http::Request::get(&format!("{}/connections/{}/redb/tables", API_BASE, id))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

    parse_api_response(response).await
}

pub async fn query_redb_keys(req: RedbQueryRequest) -> Result<RedbKeyList, String> {
    let response =
        gloo_net::http::Request::post(&format!(
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
    let response =
        gloo_net::http::Request::post(&format!(
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

#[allow(dead_code)]
pub async fn import_sql(id: String, req: ImportSqlRequest) -> Result<ImportResult, String> {
    let response =
        gloo_net::http::Request::post(&format!("{}/connections/{}/import", API_BASE, id))
            .json(&req)
            .map_err(|e| format!("Serialization error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

    parse_api_response(response).await
}

pub async fn create_connection(req: CreateConnectionRequest) -> Result<Connection, String> {
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

    let response = gloo_net::http::Request::put(&format!("{}/connections/{}", API_BASE, id))
        .json(&update_req)
        .map_err(|e| format!("Serialization error: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    parse_api_response(response).await
}

pub async fn delete_connection(id: String) -> Result<bool, String> {
    let response = gloo_net::http::Request::delete(&format!("{}/connections/{}", API_BASE, id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    parse_api_response(response).await
}

pub async fn execute_query(req: ExecuteQueryRequest) -> Result<QueryResult, String> {
    let response = gloo_net::http::Request::post(&format!("{}/query", API_BASE))
        .json(&req)
        .map_err(|e| format!("Serialization error: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    parse_api_response(response).await
}

pub async fn test_connection(id: String) -> Result<String, String> {
    let response = gloo_net::http::Request::post(&format!("{}/connections/{}/test", API_BASE, id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    parse_api_response(response).await
}

pub async fn test_connection_request(req: CreateConnectionRequest) -> Result<String, String> {
    let response = gloo_net::http::Request::post(&format!("{}/connections/test", API_BASE))
        .json(&req)
        .map_err(|e| format!("Serialization error: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    parse_api_response(response).await
}

pub async fn get_schema(id: String) -> Result<SchemaInfo, String> {
    leptos::logging::log!("info[API Request] get_schema - id: {}", id);

    let response = gloo_net::http::Request::get(&format!("{}/connections/{}/schema", API_BASE, id))
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
    let response = gloo_net::http::Request::get(&format!(
        "{}/connections/{}/tables/{}/data?limit={}&offset={}",
        API_BASE, id, table, limit, offset
    ))
    .send()
    .await
    .map_err(|e| format!("Network error: {}", e))?;

    parse_api_response(response).await
}

pub async fn get_query_history() -> Result<Vec<QueryHistory>, String> {
    let response = gloo_net::http::Request::get(&format!("{}/history", API_BASE))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    parse_api_response(response).await
}

pub async fn save_query_history(req: SaveQueryHistoryRequest) -> Result<QueryHistory, String> {
    let response = gloo_net::http::Request::post(&format!("{}/history", API_BASE))
        .json(&req)
        .map_err(|e| format!("Serialization error: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    parse_api_response(response).await
}

pub async fn clear_query_history() -> Result<(), String> {
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

pub async fn edit_row(id: String, req: EditRowRequest) -> Result<QueryResult, String> {
    let response =
        gloo_net::http::Request::post(&format!("{}/connections/{}/edit-row", API_BASE, id))
            .json(&req)
            .map_err(|e| format!("Serialization error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

    parse_api_response(response).await
}

pub async fn get_table_def(id: String, table: &str) -> Result<TableDef, String> {
    let response = gloo_net::http::Request::get(&format!(
        "{}/connections/{}/tables/{}/def",
        API_BASE, id, table
    ))
    .send()
    .await
    .map_err(|e| format!("Network error: {}", e))?;

    parse_api_response(response).await
}
