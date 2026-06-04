use crate::api::client;
use crate::components::code_snippets::CodeSnippetPanel;
use crate::components::data_table::DataTable;
use crate::components::query_history::QueryHistoryPanel;
use crate::components::sql_editor::SqlEditor;
use crate::state::use_app_state;
use crate::tab_manager::{Tab, TabKind, make_query_tab_id};
use crate::utils::sql_formatter::format_sql;
use leptos::html;
use leptos::prelude::*;
use serde_json::Value;
use sql_admin_api_types::{
    DeleteRowRequest, EditRowRequest, ExecuteQueryRequest, InsertRowRequest, QueryResult,
    SaveQueryHistoryRequest, SchemaInfo,
};
use wasm_bindgen_futures::spawn_local;

#[cfg(target_arch = "wasm32")]
use leptos::wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
use js_sys::Array;
#[cfg(target_arch = "wasm32")]
use leptos::wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
use leptos::web_sys::Blob;

#[cfg(target_arch = "wasm32")]
fn download_file(filename: &str, _mime_type: &str, content: &str) {
    let Some(window) = leptos::web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Ok(blob) = Blob::new_with_str_sequence(&Array::from_iter(std::iter::once(
        JsValue::from_str(content),
    ))) else {
        return;
    };
    let Ok(url) = leptos::web_sys::Url::create_object_url_with_blob(&blob) else {
        return;
    };
    let Ok(a) = document.create_element("a") else {
        return;
    };
    let _ = a.set_attribute("href", &url);
    let _ = a.set_attribute("download", filename);
    let _ = a.set_attribute("style", "display: none");
    let Some(body) = document.body() else { return };
    let _ = body.append_child(&a);
    if let Some(el) = a.dyn_ref::<leptos::web_sys::HtmlElement>() {
        el.click();
    }
    let _ = body.remove_child(&a);
    let _ = leptos::web_sys::Url::revoke_object_url(&url);
}

#[cfg(not(target_arch = "wasm32"))]
fn download_file(_filename: &str, _mime_type: &str, _content: &str) {}

#[component]
pub fn QueryEditor() -> impl IntoView {
    let app_state = use_app_state();
    let connections = app_state.connections;
    let (selected_connection_id, set_selected_connection_id) = signal(None::<String>);
    let (schema, set_schema) = signal(None::<SchemaInfo>);
    let (selected_table, set_selected_table) = signal(None::<String>);
    let (query, set_query) = signal("SELECT * FROM sqlite_master LIMIT 10;".to_string());
    let (result, set_result) = signal(None::<QueryResult>);
    let (is_loading, set_is_loading) = signal(false);
    let (error_msg, set_error_msg) = signal(None::<String>);
    let (current_page, set_current_page) = signal(0i64);
    let (table_name, set_table_name) = signal(String::new());
    let (pk_column, set_pk_column) = signal("id".to_string());
    let (status_msg, set_status_msg) = signal(None::<(String, bool)>);
    let (show_insert_form, set_show_insert_form) = signal(false);
    let (insert_values, set_insert_values) = signal(Vec::<String>::new());
    let page_size: i64 = 100;

    let mounted = StoredValue::new(false);
    Effect::new(move |_| {
        if !mounted.get_value() {
            mounted.set_value(true);

            app_state.tab_manager.update(|tm| {
                let tab = Tab {
                    id: make_query_tab_id(String::new()),
                    kind: TabKind::Query {
                        connection_id: String::new(),
                    },
                    title: "SQL Query".to_string(),
                };
                tm.ensure_tab(tab);
            });
        }
    });

    // Load schema when connection changes
    Effect::new(move |_| {
        let conn_id = selected_connection_id.get();
        set_schema.set(None);
        set_selected_table.set(None);
        if let Some(id) = conn_id {
            let set_schema = set_schema;
            spawn_local(async move {
                match client::get_schema(id).await {
                    Ok(s) => set_schema.set(Some(s)),
                    Err(e) => leptos::logging::error!("Failed to load schema: {}", e),
                }
            });
        }
    });

    let execute_query = move |query_text: String| {
        let conn_id_opt = selected_connection_id.get();
        let all_conns = connections.get();
        let conn_name = all_conns
            .iter()
            .find(|c| Some(c.id.clone()) == conn_id_opt)
            .map(|c| c.name.clone())
            .unwrap_or_default();

        let Some(conn_id) = conn_id_opt else {
            set_error_msg.set(Some(
                "Please select a connection and enter a query".to_string(),
            ));
            return;
        };
        if query_text.is_empty() {
            set_error_msg.set(Some(
                "Please select a connection and enter a query".to_string(),
            ));
            return;
        }

        let conn_id_1 = conn_id.clone();
        let conn_id_2 = conn_id.clone();
        let conn_id_3 = conn_id.clone();
        let qtext = query_text.clone();
        let cname = conn_name.clone();
        set_is_loading.set(true);
        set_error_msg.set(None);

        let conn_name_for_status = cname.clone();
        let db_type_for_status = all_conns
            .iter()
            .find(|c| Some(c.id.clone()) == Some(conn_id.clone()))
            .map(|c| c.database_type.to_string());

        app_state.status.update(|s| {
            s.connected_db = Some(conn_name_for_status.clone());
            s.db_type = db_type_for_status.clone();
            s.status_text = "Executing...".to_string();
        });

        spawn_local(async move {
            let req = ExecuteQueryRequest {
                connection_id: conn_id_1,
                query: query_text,
            };

            match client::execute_query(req).await {
                Ok(query_result) => {
                    let rows_count = query_result.rows.len() as i64;
                    let exec_time = query_result.execution_time_ms;

                    app_state.status.update(|s| {
                        s.row_count = Some(rows_count as usize);
                        s.exec_time = exec_time.map(|t| format!("{}ms", t));
                        s.status_text = "Query OK".to_string();
                    });

                    let _ = client::save_query_history(SaveQueryHistoryRequest {
                        connection_id: conn_id_2,
                        connection_name: cname,
                        query_text: qtext,
                        execution_time_ms: exec_time,
                        rows_count: Some(rows_count),
                        success: true,
                        error_message: None,
                    })
                    .await;

                    set_result.set(Some(query_result));
                }
                Err(e) => {
                    app_state.status.update(|s| {
                        s.status_text = "Query Error".to_string();
                    });

                    let _ = client::save_query_history(SaveQueryHistoryRequest {
                        connection_id: conn_id_3,
                        connection_name: cname,
                        query_text: qtext,
                        execution_time_ms: None,
                        rows_count: None,
                        success: false,
                        error_message: Some(e.clone()),
                    })
                    .await;

                    set_error_msg.set(Some(e));
                    set_result.set(None);
                }
            }

            set_is_loading.set(false);
        });
    };

    let execute_full = move |_| {
        execute_query(query.get());
    };

    let load_page = move |page: i64| {
        let Some(conn_id) = selected_connection_id.get() else {
            return;
        };
        set_current_page.set(page);
        spawn_local(async move {
            if let Ok(data) =
                client::get_table_data(conn_id, &query.get(), page_size, page * page_size).await
            {
                set_result.set(Some(data));
            }
        });
    };

    let select_history = move |sql: String| {
        set_query.set(sql);
    };

    let export_csv = move |_| {
        let Some(qr) = result.get() else {
            return;
        };
        let mut csv = String::new();
        csv.push_str(&qr.columns.join(","));
        csv.push('\n');
        for row in &qr.rows {
            let values: Vec<String> = row
                .iter()
                .map(|v| match v {
                    Value::Null => String::new(),
                    Value::String(s) => format!("\"{}\"", s.replace('"', "\"\"")),
                    other => other.to_string(),
                })
                .collect();
            csv.push_str(&values.join(","));
            csv.push('\n');
        }
        download_file("export.csv", "text/csv", &csv);
    };

    let export_json = move |_| {
        let Some(qr) = result.get() else {
            return;
        };
        let json_rows: Vec<serde_json::Value> = qr
            .rows
            .iter()
            .map(|row| {
                let mut obj = serde_json::Map::new();
                for (i, col) in qr.columns.iter().enumerate() {
                    obj.insert(col.clone(), row[i].clone());
                }
                serde_json::Value::Object(obj)
            })
            .collect();
        let json_str = serde_json::to_string_pretty(&json_rows).unwrap_or_default();
        download_file("export.json", "application/json", &json_str);
    };

    let handle_cell_edit = move |(row_idx, col_idx, new_val): (usize, usize, String)| {
        let Some(conn_id) = selected_connection_id.get() else {
            set_status_msg.set(Some((
                "Please set table name and primary key for editing".to_string(),
                false,
            )));
            return;
        };
        let Some(query_result) = result.get() else {
            set_status_msg.set(Some((
                "Please set table name and primary key for editing".to_string(),
                false,
            )));
            return;
        };
        let current_table = table_name.get();
        let current_pk = pk_column.get();

        if current_table.is_empty() {
            set_status_msg.set(Some((
                "Please set table name and primary key for editing".to_string(),
                false,
            )));
            return;
        }

        let columns = &query_result.columns;

        let Some(pk_idx) = columns.iter().position(|c| c == &current_pk) else {
            set_status_msg.set(Some((
                format!("Primary key column '{}' not found in result", current_pk),
                false,
            )));
            return;
        };

        let pk_value = query_result.rows[row_idx][pk_idx].clone();
        let col_name = columns[col_idx].clone();
        let new_value = if new_val.is_empty() || new_val == "NULL" {
            None
        } else {
            Some(Value::String(new_val.clone()))
        };

        let req = EditRowRequest {
            connection_id: conn_id.clone(),
            table_name: current_table.clone(),
            primary_key_column: current_pk.clone(),
            primary_key_value: pk_value,
            column: col_name.clone(),
            new_value,
        };

        spawn_local(async move {
            match client::edit_row(conn_id.clone(), req).await {
                Ok(_) => {
                    set_status_msg.set(Some(("Cell updated successfully".to_string(), true)));
                    set_result.update(|r| {
                        if let Some(qr) = r
                            && row_idx < qr.rows.len()
                            && col_idx < qr.rows[0].len()
                        {
                            let val = if new_val.is_empty() || new_val == "NULL" {
                                Value::Null
                            } else {
                                Value::String(new_val.clone())
                            };
                            qr.rows[row_idx][col_idx] = val;
                        }
                    });
                }
                Err(e) => {
                    set_status_msg.set(Some((format!("Edit failed: {}", e), false)));
                }
            }
        });
    };

    let handle_row_delete = move |row_idx: usize| {
        let Some(conn_id) = selected_connection_id.get() else {
            set_status_msg.set(Some((
                "Please set table name and primary key for editing".to_string(),
                false,
            )));
            return;
        };
        let Some(query_result) = result.get() else {
            set_status_msg.set(Some((
                "Please set table name and primary key for editing".to_string(),
                false,
            )));
            return;
        };
        let current_table = table_name.get();
        let current_pk = pk_column.get();

        if current_table.is_empty() {
            set_status_msg.set(Some((
                "Please set table name and primary key for editing".to_string(),
                false,
            )));
            return;
        }

        let columns = &query_result.columns;

        let Some(pk_idx) = columns.iter().position(|c| c == &current_pk) else {
            set_status_msg.set(Some((
                format!("Primary key column '{}' not found in result", current_pk),
                false,
            )));
            return;
        };

        let pk_value = query_result.rows[row_idx][pk_idx].clone();

        let req = DeleteRowRequest {
            connection_id: conn_id.clone(),
            table_name: current_table.clone(),
            primary_key_column: current_pk.clone(),
            primary_key_value: pk_value,
        };

        spawn_local(async move {
            match client::delete_row(conn_id.clone(), req).await {
                Ok(qr) => {
                    let rows_affected = qr.rows_affected.unwrap_or(0);
                    set_status_msg.set(Some((
                        format!("Row deleted ({} row(s) affected)", rows_affected),
                        true,
                    )));
                    set_result.update(|r| {
                        if let Some(qr) = r
                            && row_idx < qr.rows.len()
                        {
                            qr.rows.remove(row_idx);
                        }
                    });
                }
                Err(e) => {
                    set_status_msg.set(Some((format!("Delete failed: {}", e), false)));
                }
            }
        });
    };

    let handle_insert_row = move |_| {
        let Some(conn_id) = selected_connection_id.get() else {
            set_status_msg.set(Some((
                "Please select a connection and set table name first".to_string(),
                false,
            )));
            return;
        };
        let Some(query_result) = result.get() else {
            set_status_msg.set(Some((
                "Please execute a query first to determine columns".to_string(),
                false,
            )));
            return;
        };
        let current_table = table_name.get();
        if current_table.is_empty() {
            set_status_msg.set(Some((
                "Please set table name for inserting".to_string(),
                false,
            )));
            return;
        }

        let columns = query_result.columns.clone();
        let values = insert_values.get();
        let json_values: Vec<Value> = columns.iter().enumerate().map(|(i, _)| {
            let v = values.get(i).cloned().unwrap_or_default();
            if v.is_empty() || v.eq_ignore_ascii_case("null") {
                Value::Null
            } else if let Ok(n) = v.parse::<i64>() {
                Value::Number(n.into())
            } else if let Ok(f) = v.parse::<f64>() {
                serde_json::Number::from_f64(f).map(Value::Number).unwrap_or(Value::String(v))
            } else if v.eq_ignore_ascii_case("true") {
                Value::Bool(true)
            } else if v.eq_ignore_ascii_case("false") {
                Value::Bool(false)
            } else {
                Value::String(v)
            }
        }).collect();

        let req = InsertRowRequest {
            connection_id: conn_id.clone(),
            table_name: current_table.clone(),
            columns,
            values: json_values,
        };

        spawn_local(async move {
            match client::insert_row(conn_id, req).await {
                Ok(qr) => {
                    let rows_affected = qr.rows_affected.unwrap_or(0);
                    set_status_msg.set(Some((
                        format!("Row inserted ({} row(s) affected)", rows_affected),
                        true,
                    )));
                    set_show_insert_form.set(false);
                }
                Err(e) => {
                    set_status_msg.set(Some((format!("Insert failed: {}", e), false)));
                }
            }
        });
    };

    let file_input_ref: NodeRef<html::Input> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    let handle_import_file = move |ev: leptos::ev::Event| {
        let Some(input) = ev
            .target()
            .and_then(|t| t.dyn_into::<leptos::web_sys::HtmlInputElement>().ok())
        else {
            return;
        };
        let Some(file_list) = input.files() else {
            return;
        };
        let Some(file) = file_list.item(0) else {
            return;
        };

        let Some(conn_id) = selected_connection_id.get() else {
            set_status_msg.set(Some((
                "Please select a connection first".to_string(),
                false,
            )));
            return;
        };
        let set_status = set_status_msg;
        let set_load = set_is_loading;
        set_load.set(true);
        spawn_local(async move {
            let text_promise = file.text();
            let Ok(js_value) = wasm_bindgen_futures::JsFuture::from(text_promise).await else {
                set_load.set(false);
                return;
            };
            let content = js_value.as_string().unwrap_or_default();

            let req = sql_admin_api_types::ImportSqlRequest {
                connection_id: conn_id.clone(),
                sql_content: content.clone(),
                transaction_mode: sql_admin_api_types::TransactionMode::default(),
            };

            match client::import_sql(conn_id, req).await {
                Ok(import_result) => {
                    let msg = if import_result.errors.is_empty() {
                        format!(
                            "Import completed: {} statements executed in {}ms",
                            import_result.statements_executed,
                            import_result.execution_time_ms.unwrap_or(0)
                        )
                    } else {
                        format!(
                            "Import: {} succeeded, {} failed in {}ms",
                            import_result.statements_executed,
                            import_result.errors.len(),
                            import_result.execution_time_ms.unwrap_or(0)
                        )
                    };
                    set_status.set(Some((msg, import_result.errors.is_empty())));
                }
                Err(e) => {
                    set_status.set(Some((format!("Import failed: {}", e), false)));
                }
            }
            set_load.set(false);
        });
    };

    #[cfg(not(target_arch = "wasm32"))]
    let handle_import_file = move |_ev: leptos::ev::Event| {};

    view! {
        <div class="h-full flex gap-4">
            <div class="flex-1 flex flex-col min-w-0">
                <div class="bg-white dark:bg-gray-800 rounded-lg shadow mb-4 flex-shrink-0">
                    <div class="p-3 border-b dark:border-gray-700 flex items-center gap-4">
                        <div class="flex-1">
                            <select
                                class="w-full px-3 py-2 border dark:border-gray-600 dark:bg-gray-700 dark:text-gray-200 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                                on:change=move |ev| {
                                    let val = event_target_value(&ev);
                                    if val.is_empty() {
                                        set_selected_connection_id.set(None);
                                    } else {
                                        set_selected_connection_id.set(Some(val));
                                    }
                                }
                                prop:value=move || selected_connection_id.get().unwrap_or_default()
                            >
                                <option value="">"Select Connection"</option>
                                {move || {
                                    connections.get().into_iter()
                                        .filter(|conn| conn.database_type != sql_admin_api_types::DatabaseType::Redb)
                                        .map(|conn| {
                                        view! {
                                            <option value=conn.id.clone()>{conn.name}</option>
                                        }
                                    }).collect_view()
                                }}
                            </select>
                        </div>
                        <div class="flex-1">
                            <select
                                class="w-full px-3 py-2 border dark:border-gray-600 dark:bg-gray-700 dark:text-gray-200 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                                disabled=move || schema.get().is_none()
                                on:change=move |ev| {
                                    let val = event_target_value(&ev);
                                    if val.is_empty() {
                                        set_selected_table.set(None);
                                    } else {
                                        set_selected_table.set(Some(val.clone()));
                                        set_query.set(format!("SELECT * FROM {} LIMIT 100;", val));
                                    }
                                }
                                prop:value=move || selected_table.get().unwrap_or_default()
                            >
                                <option value="">"Select Table"</option>
                                {move || {
                                    schema.get().map(|s| {
                                        s.tables.into_iter().map(|t| {
                                            let name = t.name.clone();
                                            view! {
                                                <option value=t.name>{name}</option>
                                            }
                                        }).collect_view()
                                    }).unwrap_or_default()
                                }}
                            </select>
                        </div>
                        <button
                            class="px-4 py-2 bg-blue-600 text-white rounded-md text-sm hover:bg-blue-700 disabled:opacity-50"
                            disabled=is_loading
                            on:click=move |_: leptos::ev::MouseEvent| execute_full(())
                        >
                            {move || if is_loading.get() { "Running..." } else { "Execute" }}
                        </button>
                        <button
                            class="px-4 py-2 bg-green-600 text-white rounded-md text-sm hover:bg-green-700 disabled:opacity-50"
                            disabled=is_loading
                            on:click=move |_| {
                                if let Some(ref el) = file_input_ref.get() {
                                    el.click();
                                }
                            }
                        >
                            "Import SQL"
                        </button>
                        <input
                            type="file"
                            accept=".sql,.txt"
                            class="hidden"
                            node_ref=file_input_ref
                            on:change=handle_import_file
                        />
                    </div>

                    <div class="px-3 py-2 flex items-center gap-1.5 flex-wrap">
                        <span class="text-[10px] text-gray-400 font-medium uppercase tracking-wider mr-1">Tx</span>
                        <button
                            class="px-2 py-1 text-xs rounded border border-gray-300 hover:bg-gray-100 text-gray-600 disabled:opacity-50"
                            disabled=is_loading
                            on:click=move |_| {
                                set_query.set("BEGIN TRANSACTION;".to_string());
                                execute_full(());
                            }
                        >
                            "BEGIN"
                        </button>
                        <button
                            class="px-2 py-1 text-xs rounded border border-gray-300 hover:bg-green-50 hover:border-green-300 text-green-700 disabled:opacity-50"
                            disabled=is_loading
                            on:click=move |_| {
                                set_query.set("COMMIT;".to_string());
                                execute_full(());
                            }
                        >
                            "COMMIT"
                        </button>
                        <button
                            class="px-2 py-1 text-xs rounded border border-gray-300 hover:bg-red-50 hover:border-red-300 text-red-700 disabled:opacity-50"
                            disabled=is_loading
                            on:click=move |_| {
                                set_query.set("ROLLBACK;".to_string());
                                execute_full(());
                            }
                        >
                            "ROLLBACK"
                        </button>
                        <div class="w-px h-4 bg-gray-300 mx-1"></div>
                        <button
                            class="px-2 py-1 text-xs rounded border border-gray-300 hover:bg-purple-50 hover:border-purple-300 text-purple-700 disabled:opacity-50"
                            disabled=is_loading
                            on:click=move |_| {
                                let q = query.get();
                                let new_q = if q.to_uppercase().trim_start().starts_with("EXPLAIN") {
                                    q
                                } else {
                                    format!("EXPLAIN {}", q)
                                };
                                set_query.set(new_q);
                                execute_full(());
                            }
                        >
                            "EXPLAIN"
                        </button>
                        <div class="w-px h-4 bg-gray-300 mx-1"></div>
                        <button
                            class="px-2 py-1 text-xs rounded border border-gray-300 hover:bg-orange-50 hover:border-orange-300 text-orange-700 disabled:opacity-50"
                            disabled=is_loading
                            on:click=move |_| {
                                set_query.set("VACUUM;".to_string());
                                execute_full(());
                            }
                        >
                            "VACUUM"
                        </button>
                        <button
                            class="px-2 py-1 text-xs rounded border border-gray-300 hover:bg-orange-50 hover:border-orange-300 text-orange-700 disabled:opacity-50"
                            disabled=is_loading
                            on:click=move |_| {
                                set_query.set("ANALYZE;".to_string());
                                execute_full(());
                            }
                        >
                            "ANALYZE"
                        </button>
                        <div class="w-px h-4 bg-gray-300 mx-1"></div>
                        <button
                            class="px-2 py-1 text-xs rounded border border-gray-300 hover:bg-indigo-50 hover:border-indigo-300 text-indigo-700 disabled:opacity-50"
                            disabled=is_loading
                            on:click=move |_| {
                                let q = query.get();
                                let formatted = format_sql(&q);
                                set_query.set(formatted);
                            }
                        >
                            "Format"
                        </button>
                    </div>

                    <SqlEditor query=query set_query=set_query on_execute=Callback::new(move |()| execute_full(())) />
                </div>

                {move || {
                    if let Some(err) = error_msg.get() {
                        view! {
                            <div class="bg-red-50 border border-red-200 rounded-lg p-3 mb-4 flex-shrink-0">
                                <p class="text-red-700 text-sm">{err}</p>
                            </div>
                        }.into_any()
                    } else {
                        let _: () = view! {};
                        ().into_any()
                    }
                }}

                {move || {
                    if let Some((msg, is_ok)) = status_msg.get() {
                        view! {
                            <div class=if is_ok { "border rounded-lg p-3 mb-4 flex-shrink-0 bg-green-50 border-green-200" } else { "border rounded-lg p-3 mb-4 flex-shrink-0 bg-red-50 border-red-200" }>
                                <p class=if is_ok { "text-sm text-green-700" } else { "text-sm text-red-700" }>{msg}</p>
                            </div>
                        }.into_any()
                    } else {
                        let _: () = view! {};
                        ().into_any()
                    }
                }}

                {move || if let Some(query_result) = result.get() {
                    let row_count = query_result.rows.len();
                    let exec_time = query_result.execution_time_ms.map(|t| format!("{}ms", t)).unwrap_or_default();
                    let total_pages = if row_count > 0 { (row_count as i64 + page_size - 1) / page_size } else { 1 };
                    let page = current_page.get();
                    let columns = query_result.columns.clone();
                    let rows = query_result.rows.clone();

                    view! {
                        <div class="bg-white dark:bg-gray-800 rounded-lg shadow flex-1 flex flex-col min-h-0">
                            <div class="p-3 border-b dark:border-gray-700 flex justify-between items-center flex-shrink-0">
                                <div class="flex items-center gap-4">
                                    <h3 class="font-semibold text-sm">"Results"</h3>
                                    <div class="flex items-center gap-2 text-xs">
                                        <span class="text-gray-400">"Table:"</span>
                                        <input
                                            type="text"
                                            class="w-32 px-2 py-1 border rounded text-xs focus:outline-none focus:ring-1 focus:ring-blue-400"
                                            placeholder="table_name"
                                            prop:value=table_name
                                            on:input=move |ev| set_table_name.set(event_target_value(&ev))
                                        />
                                        <span class="text-gray-400">"PK:"</span>
                                        <input
                                            type="text"
                                            class="w-20 px-2 py-1 border rounded text-xs focus:outline-none focus:ring-1 focus:ring-blue-400"
                                            placeholder="id"
                                            prop:value=pk_column
                                            on:input=move |ev| set_pk_column.set(event_target_value(&ev))
                                        />
                                        <span class="text-gray-300 text-xs">"(for editing)"</span>
                                    </div>
                                    <button
                                        class="px-2 py-1 text-xs rounded border border-green-300 bg-green-50 hover:bg-green-100 text-green-700 disabled:opacity-50"
                                        disabled=move || table_name.get().is_empty()
                                        on:click=move |_| {
                                            if let Some(qr) = result.get() {
                                                set_insert_values.set(vec![String::new(); qr.columns.len()]);
                                                set_show_insert_form.set(true);
                                            }
                                        }
                                    >
                                        "+ Insert Row"
                                    </button>
                                </div>
                                <div class="text-xs text-gray-500 flex items-center gap-3">
                                    <span>{row_count}" rows"</span>
                                    {if !exec_time.is_empty() {
                                        view! { <span class="text-green-600">{exec_time}</span> }.into_any()
                                    } else {
                                        let _: () = view! {};
                                        ().into_any()
                                    }}
                                    <span class="text-gray-300">"|"</span>
                                    <button
                                        class="text-blue-600 hover:text-blue-800 hover:underline"
                                        on:click=export_csv
                                    >
                                        "CSV"
                                    </button>
                                    <button
                                        class="text-blue-600 hover:text-blue-800 hover:underline"
                                        on:click=export_json
                                    >
                                        "JSON"
                                    </button>
                                </div>
                            </div>

                            <DataTable
                                columns=columns
                                rows=rows
                                editable=true
                                on_cell_edit=Callback::new(handle_cell_edit)
                                on_row_delete=Callback::new(handle_row_delete)
                            />

                            {move || if show_insert_form.get() {
                                let qr = result.get();
                                let cols = qr.map(|q| q.columns).unwrap_or_default();
                                let current_vals = insert_values.get();
                                view! {
                                    <div class="border-t dark:border-gray-700 p-3 bg-green-50 dark:bg-green-900/20 flex-shrink-0">
                                        <div class="flex items-center justify-between mb-2">
                                            <span class="text-xs font-medium text-green-800 dark:text-green-300">"Insert New Row"</span>
                                            <button
                                                class="text-xs text-gray-500 hover:text-gray-700"
                                                on:click=move |_| set_show_insert_form.set(false)
                                            >"Cancel"</button>
                                        </div>
                                        <div class="flex flex-wrap gap-2">
                                            {cols.into_iter().enumerate().map(|(i, col)| {
                                                let val = current_vals.get(i).cloned().unwrap_or_default();
                                                view! {
                                                    <div class="flex items-center gap-1">
                                                        <label class="text-[10px] text-gray-500 font-medium">{col.clone()}</label>
                                                        <input
                                                            type="text"
                                                            class="w-24 px-2 py-1 border rounded text-xs focus:outline-none focus:ring-1 focus:ring-green-400"
                                                            placeholder="NULL"
                                                            prop:value=val
                                                            on:input=move |ev| {
                                                                set_insert_values.update(|v| {
                                                                    while v.len() <= i { v.push(String::new()); }
                                                                    v[i] = event_target_value(&ev);
                                                                });
                                                            }
                                                        />
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                        <div class="mt-2 flex justify-end">
                                            <button
                                                class="px-3 py-1 bg-green-600 text-white text-xs rounded hover:bg-green-700"
                                                on:click=handle_insert_row
                                            >"Insert"</button>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                let _: () = view! {};
                                ().into_any()
                            }}

                            {if row_count as i64 > page_size {
                                view! {
                                    <div class="p-2 border-t dark:border-gray-700 flex items-center justify-center gap-2 flex-shrink-0">
                                        <button
                                            class="px-3 py-1 text-sm rounded border hover:bg-gray-100 disabled:opacity-50"
                                            disabled=page == 0
                                            on:click=move |_| load_page(page - 1)
                                        >
                                            "Prev"
                                        </button>
                                        <span class="text-sm text-gray-600">
                                            {format!("Page {} / {}", page + 1, total_pages)}
                                        </span>
                                        <button
                                            class="px-3 py-1 text-sm rounded border hover:bg-gray-100 disabled:opacity-50"
                                            disabled=page >= total_pages - 1
                                            on:click=move |_| load_page(page + 1)
                                        >
                                            "Next"
                                        </button>
                                    </div>
                                }.into_any()
                            } else {
                                let _: () = view! {};
                                ().into_any()
                            }}
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="bg-white dark:bg-gray-800 rounded-lg shadow flex-1 flex items-center justify-center text-gray-400 dark:text-gray-500">
                            "Execute a query to see results"
                        </div>
                    }.into_any()
                }}
            </div>

            <div class="w-72 flex-shrink-0 h-full flex flex-col gap-4">
                <div class="flex-1 min-h-0">
                    <CodeSnippetPanel on_select=Callback::new(move |sql| set_query.set(sql)) />
                </div>
                <div class="flex-1 min-h-0">
                    <QueryHistoryPanel on_select=Callback::new(select_history) />
                </div>
            </div>
        </div>
    }
}
