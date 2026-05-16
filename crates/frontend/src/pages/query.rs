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
use sql_admin_shared::{
    Connection, EditRowRequest, ExecuteQueryRequest, QueryResult, SaveQueryHistoryRequest,
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
    let window = leptos::web_sys::window().unwrap();
    let document = window.document().unwrap();
    let blob = Blob::new_with_str_sequence(&Array::from_iter(std::iter::once(JsValue::from_str(
        content,
    ))))
    .unwrap();
    let url = leptos::web_sys::Url::create_object_url_with_blob(&blob).unwrap();
    let a = document.create_element("a").unwrap();
    a.set_attribute("href", &url).unwrap();
    a.set_attribute("download", filename).unwrap();
    a.set_attribute("style", "display: none").unwrap();
    document.body().unwrap().append_child(&a).unwrap();
    a.dyn_ref::<leptos::web_sys::HtmlElement>().unwrap().click();
    document.body().unwrap().remove_child(&a).unwrap();
    leptos::web_sys::Url::revoke_object_url(&url).unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
fn download_file(_filename: &str, _mime_type: &str, _content: &str) {}

#[component]
pub fn QueryEditor() -> impl IntoView {
    let app_state = use_app_state();
    let dark_mode = app_state.dark_mode;
    let (connections, set_connections) = signal(Vec::<Connection>::new());
    let (selected_connection_id, set_selected_connection_id) = signal(None::<String>);
    let (query, set_query) = signal("SELECT * FROM sqlite_master LIMIT 10;".to_string());
    let (result, set_result) = signal(None::<QueryResult>);
    let (is_loading, set_is_loading) = signal(false);
    let (error_msg, set_error_msg) = signal(None::<String>);
    let (current_page, set_current_page) = signal(0i64);
    let (table_name, set_table_name) = signal(String::new());
    let (pk_column, set_pk_column) = signal("id".to_string());
    let (status_msg, set_status_msg) = signal(None::<(String, bool)>);
    let page_size: i64 = 100;

    let mounted = RwSignal::new(false);
    Effect::new(move |_| {
        if !mounted.get() {
            mounted.set(true);

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

            spawn_local(async move {
                if let Ok(conns) = client::list_connections().await {
                    set_connections.set(conns);
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

        if conn_id_opt.is_none() || query_text.is_empty() {
            set_error_msg.set(Some(
                "Please select a connection and enter a query".to_string(),
            ));
            return;
        }

        let conn_id = conn_id_opt.unwrap();
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
        let conn_id = selected_connection_id.get();
        if conn_id.is_none() {
            return;
        }
        set_current_page.set(page);
        spawn_local(async move {
            if let Ok(data) =
                client::get_table_data(conn_id.unwrap(), &query.get(), page_size, page * page_size)
                    .await
            {
                set_result.set(Some(data));
            }
        });
    };

    let select_history = move |sql: String| {
        set_query.set(sql);
    };

    let export_csv = move |_| {
        let current_result = result.get();
        if current_result.is_none() {
            return;
        }
        let qr = current_result.as_ref().unwrap();
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
        let current_result = result.get();
        if current_result.is_none() {
            return;
        }
        let qr = current_result.as_ref().unwrap();
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
        let conn_id = selected_connection_id.get();
        let current_result = result.get();
        let current_table = table_name.get();
        let current_pk = pk_column.get();

        if conn_id.is_none() || current_result.is_none() || current_table.is_empty() {
            set_status_msg.set(Some((
                "Please set table name and primary key for editing".to_string(),
                false,
            )));
            return;
        }

        let query_result = current_result.as_ref().unwrap();
        let columns = &query_result.columns;

        let pk_idx = columns.iter().position(|c| c == &current_pk);
        if pk_idx.is_none() {
            set_status_msg.set(Some((
                format!("Primary key column '{}' not found in result", current_pk),
                false,
            )));
            return;
        }

        let pk_value = query_result.rows[row_idx][pk_idx.unwrap()].clone();
        let col_name = columns[col_idx].clone();
        let new_value = if new_val.is_empty() || new_val == "NULL" {
            None
        } else {
            Some(Value::String(new_val.clone()))
        };

        let req = EditRowRequest {
            table_name: current_table.clone(),
            primary_key_column: current_pk.clone(),
            primary_key_value: pk_value,
            column: col_name.clone(),
            new_value,
        };

        spawn_local(async move {
            match client::edit_row(conn_id.unwrap(), req).await {
                Ok(_) => {
                    set_status_msg.set(Some(("Cell updated successfully".to_string(), true)));
                    set_result.update(|r| {
                        if let Some(qr) = r {
                            if row_idx < qr.rows.len() && col_idx < qr.rows[0].len() {
                                let val = if new_val.is_empty() || new_val == "NULL" {
                                    Value::Null
                                } else {
                                    Value::String(new_val.clone())
                                };
                                qr.rows[row_idx][col_idx] = val;
                            }
                        }
                    });
                }
                Err(e) => {
                    set_status_msg.set(Some((format!("Edit failed: {}", e), false)));
                }
            }
        });
    };

    let file_input_ref: NodeRef<html::Input> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    let handle_import_file = move |ev: leptos::ev::Event| {
        let input = ev
            .target()
            .and_then(|t| t.dyn_into::<leptos::web_sys::HtmlInputElement>().ok());
        if let Some(input) = input {
            let files = input.files();
            if let Some(file_list) = files {
                if let Some(file) = file_list.item(0) {
                    let conn_id = selected_connection_id.get();
                    if conn_id.is_none() {
                        set_status_msg.set(Some((
                            "Please select a connection first".to_string(),
                            false,
                        )));
                        return;
                    }
                    let set_status = set_status_msg;
                    let set_load = set_is_loading;
                    set_load.set(true);
                    spawn_local(async move {
                        let text_promise = file.text();
                        let js_value = wasm_bindgen_futures::JsFuture::from(text_promise)
                            .await
                            .unwrap();
                        let content = js_value.as_string().unwrap_or_default();

                        let req = sql_admin_shared::ImportSqlRequest {
                            sql_content: content.clone(),
                        };

                        match client::import_sql(conn_id.unwrap(), req).await {
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
                }
            }
        }
    };

    #[cfg(not(target_arch = "wasm32"))]
    let handle_import_file = move |_ev: leptos::ev::Event| {};

    view! {
        <div class="h-full flex gap-4">
            <div class="flex-1 flex flex-col min-w-0">
                <div class=move || {
                    if dark_mode.get() {
                        "bg-gray-800 rounded-lg shadow mb-4 flex-shrink-0"
                    } else {
                        "bg-white rounded-lg shadow mb-4 flex-shrink-0"
                    }
                }>
                    <div class=move || {
                        if dark_mode.get() {
                            "p-3 border-b border-gray-700 flex items-center gap-4"
                        } else {
                            "p-3 border-b flex items-center gap-4"
                        }
                    }>
                        <div class="flex-1">
                            <select
                                class=move || {
                                    if dark_mode.get() {
                                        "w-full px-3 py-2 bg-gray-700 border border-gray-600 text-gray-200 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    } else {
                                        "w-full px-3 py-2 border rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    }
                                }
                                on:change=move |ev| {
                                    let val = event_target_value(&ev);
                                    set_selected_connection_id.set(Some(val));
                                }
                            >
                                <option value="">"Select Connection"</option>
                                {move || {
                                    connections.get().into_iter().map(|conn| {
                                        view! {
                                            <option value=conn.id.clone()>{conn.name}</option>
                                        }
                                    }).collect_view()
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
                                    let _ = el.click();
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
                        view! {}.into_any()
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
                        view! {}.into_any()
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
                        <div class=move || {
                            if dark_mode.get() {
                                "bg-gray-800 rounded-lg shadow flex-1 flex flex-col min-h-0"
                            } else {
                                "bg-white rounded-lg shadow flex-1 flex flex-col min-h-0"
                            }
                        }>
                            <div class=move || {
                                if dark_mode.get() {
                                    "p-3 border-b border-gray-700 flex justify-between items-center flex-shrink-0"
                                } else {
                                    "p-3 border-b flex justify-between items-center flex-shrink-0"
                                }
                            }>
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
                                </div>
                                <div class="text-xs text-gray-500 flex items-center gap-3">
                                    <span>{row_count}" rows"</span>
                                    {if !exec_time.is_empty() {
                                        view! { <span class="text-green-600">{exec_time}</span> }.into_any()
                                    } else {
                                        view! {}.into_any()
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
                            />

                            {if row_count as i64 > page_size {
                                view! {
                                    <div class=move || {
                                        if dark_mode.get() {
                                            "p-2 border-t border-gray-700 flex items-center justify-center gap-2 flex-shrink-0"
                                        } else {
                                            "p-2 border-t flex items-center justify-center gap-2 flex-shrink-0"
                                        }
                                    }>
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
                                view! {}.into_any()
                            }}
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class=move || {
                            if dark_mode.get() {
                                "bg-gray-800 rounded-lg shadow flex-1 flex items-center justify-center text-gray-500"
                            } else {
                                "bg-white rounded-lg shadow flex-1 flex items-center justify-center text-gray-400"
                            }
                        }>
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
