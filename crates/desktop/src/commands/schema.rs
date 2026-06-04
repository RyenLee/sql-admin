use tauri::State;
use sql_admin_api_types::*;
use crate::state::AppState;

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

    let indexes: Vec<IndexInfo> = if !primary_key_names.is_empty() {
        vec![IndexInfo {
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

#[tauri::command]
pub async fn get_schema(
    state: State<'_, AppState>,
    connection_id: String,
) -> Result<SchemaInfo, String> {
    let definitions = state.query_handler
        .get_schema_with_columns(&connection_id)
        .await
        .map_err(|e| e.to_string())?;

    let tables: Vec<TableInfo> = definitions.into_iter().map(|def| {
        let columns: Vec<ColumnInfo> = def.columns.into_iter().map(|c| ColumnInfo {
            name: c.name,
            data_type: c.data_type,
            not_null: !c.nullable,
            default_value: None,
            is_primary_key: c.is_primary_key,
        }).collect();
        TableInfo {
            name: def.table_name,
            row_count: None,
            columns,
        }
    }).collect();

    Ok(SchemaInfo {
        tables,
        views: vec![],
        indexes: vec![],
        triggers: vec![],
        schemas: vec![],
    })
}

#[tauri::command]
pub async fn get_table_def(
    state: State<'_, AppState>,
    connection_id: String,
    table_name: String,
) -> Result<TableDef, String> {
    let result = state.query_handler
        .get_table_definition(&connection_id, &table_name)
        .await
        .map_err(|e| e.to_string())?;
    Ok(domain_table_def_to_dto(result))
}
