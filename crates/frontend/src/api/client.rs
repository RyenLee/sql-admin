#![allow(dead_code)]

use sql_admin_shared::{
    ApiResponse, Connection, CreateConnectionRequest, DeleteRowRequest, EditRowRequest,
    ExecuteQueryRequest, ImportResult, ImportSqlRequest, InsertRowRequest, QueryHistory,
    QueryResult, SaveQueryHistoryRequest, SchemaInfo, TableDef, UpdateConnectionRequest,
};

const API_BASE: &str = "http://localhost:3000/api";

pub async fn list_connections() -> Result<Vec<Connection>, String> {
    leptos::logging::log!("info[API Request] list_connections");
    
    let response = gloo_net::http::Request::get(&format!("{}/connections", API_BASE))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let api_response: ApiResponse<Vec<Connection>> =
        response.json().await.map_err(|e| e.to_string())?;

    api_response
        .data
        .ok_or_else(|| api_response.error.unwrap_or_default())
}

#[allow(dead_code)]
pub async fn import_sql(id: String, req: ImportSqlRequest) -> Result<ImportResult, String> {
    let response =
        gloo_net::http::Request::post(&format!("{}/connections/{}/import", API_BASE, id))
            .json(&req)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

    let api_response: ApiResponse<ImportResult> =
        response.json().await.map_err(|e| e.to_string())?;

    api_response
        .data
        .ok_or_else(|| api_response.error.unwrap_or_default())
}

pub async fn create_connection(req: CreateConnectionRequest) -> Result<Connection, String> {
    let response = gloo_net::http::Request::post(&format!("{}/connections", API_BASE))
        .json(&req)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let api_response: ApiResponse<Connection> = response.json().await.map_err(|e| e.to_string())?;

    api_response
        .data
        .ok_or_else(|| api_response.error.unwrap_or_default())
}

#[allow(dead_code)]
pub async fn get_connection(id: String) -> Result<Connection, String> {
    let response = gloo_net::http::Request::get(&format!("{}/connections/{}", API_BASE, id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let api_response: ApiResponse<Connection> = response.json().await.map_err(|e| e.to_string())?;

    api_response
        .data
        .ok_or_else(|| api_response.error.unwrap_or_default())
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
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let api_response: ApiResponse<Connection> = response.json().await.map_err(|e| e.to_string())?;

    api_response
        .data
        .ok_or_else(|| api_response.error.unwrap_or_default())
}

pub async fn delete_connection(id: String) -> Result<String, String> {
    let response = gloo_net::http::Request::delete(&format!("{}/connections/{}", API_BASE, id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let api_response: ApiResponse<String> = response.json().await.map_err(|e| e.to_string())?;

    api_response
        .data
        .ok_or_else(|| api_response.error.unwrap_or_default())
}

pub async fn execute_query(req: ExecuteQueryRequest) -> Result<QueryResult, String> {
    let response = gloo_net::http::Request::post(&format!("{}/query", API_BASE))
        .json(&req)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let api_response: ApiResponse<QueryResult> =
        response.json().await.map_err(|e| e.to_string())?;

    api_response
        .data
        .ok_or_else(|| api_response.error.unwrap_or_default())
}

pub async fn test_connection(id: String) -> Result<String, String> {
    let response = gloo_net::http::Request::post(&format!("{}/connections/{}/test", API_BASE, id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let api_response: ApiResponse<String> = response.json().await.map_err(|e| e.to_string())?;

    api_response
        .data
        .ok_or_else(|| api_response.error.unwrap_or_default())
}

pub async fn test_connection_request(req: CreateConnectionRequest) -> Result<String, String> {
    let response = gloo_net::http::Request::post(&format!("{}/connections/test", API_BASE))
        .json(&req)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let api_response: ApiResponse<String> = response.json().await.map_err(|e| e.to_string())?;

    api_response
        .data
        .ok_or_else(|| api_response.error.unwrap_or_default())
}

pub async fn get_schema(id: String) -> Result<SchemaInfo, String> {
    leptos::logging::log!("info[API Request] get_schema - id: {}", id);
    
    let response = gloo_net::http::Request::get(&format!("{}/connections/{}/schema", API_BASE, id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let api_response: ApiResponse<SchemaInfo> = response.json().await.map_err(|e| e.to_string())?;

    api_response
        .data
        .ok_or_else(|| api_response.error.unwrap_or_default())
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
    .map_err(|e| e.to_string())?;

    let api_response: ApiResponse<QueryResult> =
        response.json().await.map_err(|e| e.to_string())?;

    api_response
        .data
        .ok_or_else(|| api_response.error.unwrap_or_default())
}

pub async fn get_query_history() -> Result<Vec<QueryHistory>, String> {
    let response = gloo_net::http::Request::get(&format!("{}/history", API_BASE))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let api_response: ApiResponse<Vec<QueryHistory>> =
        response.json().await.map_err(|e| e.to_string())?;

    api_response
        .data
        .ok_or_else(|| api_response.error.unwrap_or_default())
}

pub async fn save_query_history(req: SaveQueryHistoryRequest) -> Result<QueryHistory, String> {
    let response = gloo_net::http::Request::post(&format!("{}/history", API_BASE))
        .json(&req)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let api_response: ApiResponse<QueryHistory> =
        response.json().await.map_err(|e| e.to_string())?;

    api_response
        .data
        .ok_or_else(|| api_response.error.unwrap_or_default())
}

pub async fn clear_query_history() -> Result<(), String> {
    let response = gloo_net::http::Request::delete(&format!("{}/history", API_BASE))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err("Failed to clear query history".to_string());
    }

    Ok(())
}

pub async fn delete_query_history_item(id: String) -> Result<(), String> {
    let response = gloo_net::http::Request::delete(&format!("{}/history/{}", API_BASE, id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err("Failed to delete query history item".to_string());
    }

    Ok(())
}

pub async fn edit_row(id: String, req: EditRowRequest) -> Result<QueryResult, String> {
    let response =
        gloo_net::http::Request::post(&format!("{}/connections/{}/edit-row", API_BASE, id))
            .json(&req)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

    let api_response: ApiResponse<QueryResult> =
        response.json().await.map_err(|e| e.to_string())?;

    api_response
        .data
        .ok_or_else(|| api_response.error.unwrap_or_default())
}

pub async fn delete_row(id: String, req: DeleteRowRequest) -> Result<QueryResult, String> {
    let response =
        gloo_net::http::Request::post(&format!("{}/connections/{}/delete-row", API_BASE, id))
            .json(&req)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

    let api_response: ApiResponse<QueryResult> =
        response.json().await.map_err(|e| e.to_string())?;

    api_response
        .data
        .ok_or_else(|| api_response.error.unwrap_or_default())
}

pub async fn insert_row(id: String, req: InsertRowRequest) -> Result<QueryResult, String> {
    let response =
        gloo_net::http::Request::post(&format!("{}/connections/{}/insert-row", API_BASE, id))
            .json(&req)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

    let api_response: ApiResponse<QueryResult> =
        response.json().await.map_err(|e| e.to_string())?;

    api_response
        .data
        .ok_or_else(|| api_response.error.unwrap_or_default())
}

pub async fn get_table_def(id: String, table: &str) -> Result<TableDef, String> {
    let response = gloo_net::http::Request::get(&format!(
        "{}/connections/{}/tables/{}/def",
        API_BASE, id, table
    ))
    .send()
    .await
    .map_err(|e| e.to_string())?;

    let api_response: ApiResponse<TableDef> = response.json().await.map_err(|e| e.to_string())?;

    api_response
        .data
        .ok_or_else(|| api_response.error.unwrap_or_default())
}
