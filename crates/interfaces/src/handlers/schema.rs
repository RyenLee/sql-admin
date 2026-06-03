use axum::{
    extract::{Path, State},
    Json,
};
use sql_admin_api_types::{ApiResponse, ColumnInfo, SchemaInfo, TableDef, TableInfo};

use crate::error::AppResult;
use crate::state::AppState;

fn domain_table_info_to_dto(
    t: sql_admin_domain::shared::pool::TableInfo,
) -> TableInfo {
    TableInfo {
        name: t.name,
        row_count: None,
        columns: vec![],
    }
}

fn domain_table_def_to_dto(
    def: sql_admin_domain::shared::pool::TableDefinition,
) -> TableDef {
    let columns: Vec<ColumnInfo> = def.columns.into_iter().map(|c| {
        ColumnInfo {
            name: c.name,
            data_type: c.data_type,
            not_null: !c.nullable,
            default_value: None,
            is_primary_key: c.is_primary_key,
        }
    }).collect();

    let primary_key_names: Vec<String> = def.primary_keys.to_vec();

    let indexes: Vec<sql_admin_api_types::IndexInfo> = if !primary_key_names.is_empty() {
        vec![sql_admin_api_types::IndexInfo {
            name: "PRIMARY".to_string(),
            table_name: def.table_name.clone(),
            columns: primary_key_names,
            is_unique: true,
        }]
    } else {
        vec![]
    };

    TableDef {
        name: def.table_name,
        ddl: String::new(),
        columns,
        indexes,
        triggers: vec![],
        row_count: None,
    }
}

pub async fn get_schema(
    State(state): State<AppState>,
    Path(conn_id): Path<String>,
) -> AppResult<Json<ApiResponse<SchemaInfo>>> {
    let tables = state.query_handler.get_schema(&conn_id).await?;
    let schema_info = SchemaInfo {
        tables: tables.into_iter().map(domain_table_info_to_dto).collect(),
        views: vec![],
        indexes: vec![],
        triggers: vec![],
        schemas: vec![],
    };
    Ok(Json(ApiResponse::ok(schema_info)))
}

pub async fn get_table_def(
    State(state): State<AppState>,
    Path((conn_id, table)): Path<(String, String)>,
) -> AppResult<Json<ApiResponse<TableDef>>> {
    let result = state.query_handler.get_table_definition(&conn_id, &table).await?;
    Ok(Json(ApiResponse::ok(domain_table_def_to_dto(result))))
}