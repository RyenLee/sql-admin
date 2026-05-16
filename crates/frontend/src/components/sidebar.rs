use crate::api::client;
use crate::state::AppState;
use crate::tab_manager::{Tab, TabKind, make_connections_tab_id, make_table_structure_tab_id};
use leptos::prelude::*;
use sql_admin_shared::{Connection, SchemaInfo};
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn DatabaseSidebar() -> impl IntoView {
    let app_state = use_context::<AppState>();
    let dark_mode = app_state
        .as_ref()
        .map(|s| s.dark_mode)
        .unwrap_or(RwSignal::new(false));
    let (connections, set_connections) = signal(Vec::<Connection>::new());
    let (expanded_conn, set_expanded_conn) = signal(None::<String>);
    let (schemas, set_schemas) = signal(std::collections::HashMap::<String, SchemaInfo>::new());
    let (expanded_tables, set_expanded_tables) = signal(std::collections::HashSet::<String>::new());
    let (loading_schema, set_loading_schema) = signal(None::<String>);
    let (refreshing, set_refreshing) = signal(false);
    let (refresh_error, set_refresh_error) = signal(None::<String>);

    let refresh_trigger = app_state
        .as_ref()
        .map(|s| s.refresh_trigger)
        .unwrap_or(RwSignal::new(0u32));

    Effect::new(move |_| {
        let _ = refresh_trigger.get();
        set_refreshing.set(true);
        set_refresh_error.set(None);
        spawn_local(async move {
            match client::list_connections().await {
                Ok(conns) => {
                    set_connections.set(conns);
                    set_refresh_error.set(None);
                    
                    let current_expanded = expanded_conn.get();
                    if let Some(conn_id) = current_expanded {
                        let current_schemas = schemas.get();
                        if current_schemas.contains_key(&conn_id) {
                            set_loading_schema.set(Some(conn_id.clone()));
                            match client::get_schema(conn_id.clone()).await {
                                Ok(schema) => {
                                    set_schemas.update(|s| {
                                        s.insert(conn_id, schema);
                                    });
                                }
                                Err(e) => {
                                    leptos::logging::error!("Failed to refresh schema: {}", e);
                                }
                            }
                            set_loading_schema.set(None);
                        }
                    }
                }
                Err(e) => {
                    let msg = format!("Failed to refresh connections: {}", e);
                    leptos::logging::error!("{}", msg);
                    set_refresh_error.set(Some(msg));
                }
            }
            set_refreshing.set(false);
        });
    });

    let toggle_connection = move |id: String| {
        let current = expanded_conn.get();
        if current == Some(id.clone()) {
            set_expanded_conn.set(None);
        } else {
            set_expanded_conn.set(Some(id.clone()));
            
            let current_schemas = schemas.get();
            if current_schemas.contains_key(&id) {
                return;
            }
            
            set_loading_schema.set(Some(id.clone()));

            spawn_local(async move {
                match client::get_schema(id.clone()).await {
                    Ok(schema) => {
                        set_schemas.update(|s| {
                            s.insert(id, schema);
                        });
                    }
                    Err(e) => {
                        leptos::logging::error!("Failed to load schema: {}", e);
                    }
                }
                set_loading_schema.set(None);
            });
        }
    };

    let open_connections_tab = {
        let app_state = app_state.clone();
        move || {
            let route = "/connections".to_string();
            if let Some(tm) = app_state.as_ref().map(|s| &s.tab_manager) {
                tm.update(|tm| {
                    let tab = Tab {
                        id: make_connections_tab_id(),
                        kind: TabKind::Connections,
                        title: "Connections".to_string(),
                    };
                    tm.ensure_tab(tab);
                });
            }
            if let Some(ref s) = app_state {
                s.pending_navigation.set(Some(route));
            }
        }
    };

    view! {
        <div class="p-3">
            <div class="flex items-center justify-between mb-3">
                <h2 class=move || {
                    if dark_mode.get() {
                        "text-sm font-semibold text-gray-300 uppercase tracking-wider"
                    } else {
                        "text-sm font-semibold text-gray-700 uppercase tracking-wider"
                    }
                }>"Explorer"</h2>
                {move || if refreshing.get() {
                    view! {
                        <span class="text-xs text-gray-400 animate-pulse">"Refreshing..."</span>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }}
            </div>

            {move || {
                if let Some(err) = refresh_error.get() {
                    view! {
                        <div class="mb-2 px-2 py-1 text-xs bg-red-100 text-red-700 rounded">
                            {err}
                        </div>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}

            <div class="space-y-1">
                {move || {
                    let conns = connections.get();
                    let current_expanded = expanded_conn.get();
                    let current_schemas = schemas.get();
                    let current_loading = loading_schema.get();
                    let current_expanded_tables = expanded_tables.get();

                    conns.into_iter().map(|conn| {
                        let conn_id = conn.id.clone();
                        let conn_id_1 = conn_id.clone();
                        let conn_id_2 = conn_id.clone();
                        let is_expanded = current_expanded == Some(conn_id.clone());
                        let is_loading = current_loading == Some(conn_id.clone());
                        let schema = current_schemas.get(&conn_id).cloned();
                        let conn_name = conn.name.clone();
                        let db_type = conn.database_type.to_string();
                        let expanded_tables_clone = current_expanded_tables.clone();

                        view! {
                            <div>
                                <div
                                    class=move || {
                                        if dark_mode.get() {
                                            "flex items-center px-2 py-1.5 rounded cursor-pointer hover:bg-gray-700 text-sm"
                                        } else {
                                            "flex items-center px-2 py-1.5 rounded cursor-pointer hover:bg-gray-100 text-sm"
                                        }
                                    }
                                    on:click=move |_| toggle_connection(conn_id_1.clone())
                                >
                                    <span class="mr-1 text-xs text-gray-400">
                                        {if is_expanded { "▼" } else { "▶" }}
                                    </span>
                                    <span class="mr-2">"●"</span>
                                    <span class=move || {
                                        if dark_mode.get() {
                                            "truncate font-medium text-gray-200"
                                        } else {
                                            "truncate font-medium text-gray-800"
                                        }
                                    }>{conn_name}</span>
                                    <span class="ml-auto text-xs text-gray-400">{db_type}</span>
                                </div>

                                {if is_expanded {
                                    if is_loading {
                                        view! {
                                            <div class="ml-6 py-2 text-xs text-gray-400">"Loading..."</div>
                                        }.into_any()
                                    } else if let Some(schema) = schema {
                                        view! {
                                            <div class="ml-4">
                                                <SchemaTree
                                                    conn_id=conn_id_2
                                                    schema=schema
                                                    expanded_tables=expanded_tables_clone
                                                    set_expanded_tables=set_expanded_tables
                                                />
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {}.into_any()
                                    }
                                } else {
                                    view! {}.into_any()
                                }}
                            </div>
                        }
                    }).collect_view()
                }}
            </div>

            <div class="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
                <div
                    class="text-xs text-blue-600 hover:text-blue-800 cursor-pointer"
                    on:click={
                        let open_connections_tab = open_connections_tab.clone();
                        move |_| open_connections_tab()
                    }
                >
                    "Manage Connections"
                </div>
            </div>
        </div>
    }
}

#[component]
fn SchemaTree(
    conn_id: String,
    schema: SchemaInfo,
    expanded_tables: std::collections::HashSet<String>,
    set_expanded_tables: WriteSignal<std::collections::HashSet<String>>,
) -> impl IntoView {
    let app_state = use_context::<AppState>();
    let dark_mode = app_state
        .as_ref()
        .map(|s| s.dark_mode)
        .unwrap_or(RwSignal::new(false));
    let tab_manager = app_state.as_ref().map(|s| s.tab_manager);

    view! {
        <div class="space-y-0.5">
            {if !schema.tables.is_empty() {
                view! {
                    <div>
                        <div class=move || {
                            if dark_mode.get() {
                                "flex items-center px-2 py-1 text-xs font-medium text-gray-300 uppercase tracking-wider"
                            } else {
                                "flex items-center px-2 py-1 text-xs font-medium text-gray-600 uppercase tracking-wider"
                            }
                        }>
                            <span class="mr-1">"▼"</span>
                            "Tables"
                            <span class="ml-1 text-gray-400">"(" {schema.tables.len()} ")"</span>
                        </div>
                        {schema.tables.into_iter().map(|table| {
                            let table_key = format!("{}:{}", conn_id, table.name);
                            let is_expanded = expanded_tables.contains(&table_key);
                            let table_name = table.name.clone();
                            let row_count = table.row_count;
                            let columns = table.columns.clone();
                            let key_clone = table_key.clone();
                            let tm = tab_manager.clone();
                            let app_state_sidebar = app_state.clone();
                            let conn_id_for_tab = conn_id.clone();
                            let table_name_for_tab = table_name.clone();

                            view! {
                                <div>
                                    <div
                                        class=move || {
                                            if dark_mode.get() {
                                                "flex items-center px-3 py-1 rounded cursor-pointer hover:bg-gray-700 text-sm"
                                            } else {
                                                "flex items-center px-3 py-1 rounded cursor-pointer hover:bg-gray-100 text-sm"
                                            }
                                        }
                                        on:click=move |_| {
                                            set_expanded_tables.update(|s| {
                                                if s.contains(&key_clone) {
                                                    s.remove(&key_clone);
                                                } else {
                                                    s.insert(key_clone.clone());
                                                }
                                            });
                                        }
                                    >
                                        <span class="mr-1 text-xs text-gray-400">
                                            {if is_expanded { "▼" } else { "▶" }}
                                        </span>
                                        <span class=move || if dark_mode.get() { "text-gray-300" } else { "text-gray-700" }>"⊞"</span>
                                        <span
                                            class=move || {
                                                if dark_mode.get() {
                                                    "ml-1 truncate text-gray-300 hover:text-blue-400 cursor-pointer"
                                                } else {
                                                    "ml-1 truncate text-gray-700 hover:text-blue-600 cursor-pointer"
                                                }
                                            }
                                            on:click={
                                                let tm = tm.clone();
                                                let app_state_sidebar = app_state_sidebar.clone();
                                                move |ev: leptos::ev::MouseEvent| {
                                                    ev.stop_propagation();
                                                    if let Some(tm) = tm {
                                                        let route = format!("/table/{}/{}", conn_id_for_tab, table_name_for_tab);
                                                        tm.update(|tm| {
                                                            let tab = Tab {
                                                                id: make_table_structure_tab_id(
                                                                    conn_id_for_tab.clone(),
                                                                    &table_name_for_tab,
                                                                ),
                                                                kind: TabKind::TableStructure {
                                                                    connection_id: conn_id_for_tab.clone(),
                                                                    table_name: table_name_for_tab.clone(),
                                                                },
                                                                title: table_name_for_tab.clone(),
                                                            };
                                                            tm.ensure_tab(tab);
                                                        });
                                                        if let Some(ref s) = app_state_sidebar {
                                                            s.pending_navigation.set(Some(route));
                                                        }
                                                    }
                                                }
                                            }
                                        >{table_name}</span>
                                        {row_count.map(|c| view! {
                                            <span class="ml-auto text-xs text-gray-400">"(" {c} ")"</span>
                                        })}
                                    </div>
                                    {if is_expanded {
                                        view! {
                                            <div class="ml-8 space-y-0.5">
                                                {columns.into_iter().map(|col| {
                                                    let type_info = col.data_type.clone();
                                                    let key_icon = if col.is_primary_key { "🔑" } else { "" };
                                                    let null_icon = if col.not_null { "" } else { "○" };
                                                    view! {
                                                        <div class=move || {
                                                            if dark_mode.get() {
                                                                "flex items-center px-2 py-0.5 text-xs text-gray-400"
                                                            } else {
                                                                "flex items-center px-2 py-0.5 text-xs text-gray-600"
                                                            }
                                                        }>
                                                            <span class="mr-1">{key_icon}</span>
                                                            <span class="truncate">{col.name}</span>
                                                            <span class="ml-auto text-gray-400">{type_info}</span>
                                                            <span class="ml-1 text-gray-300">{null_icon}</span>
                                                        </div>
                                                    }
                                                }).collect_view()}
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {}.into_any()
                                    }}
                                </div>
                            }
                        }).collect_view()}
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }}

            {if !schema.views.is_empty() {
                view! {
                    <div>
                        <div class=move || {
                            if dark_mode.get() {
                                "flex items-center px-2 py-1 text-xs font-medium text-gray-300 uppercase tracking-wider"
                            } else {
                                "flex items-center px-2 py-1 text-xs font-medium text-gray-600 uppercase tracking-wider"
                            }
                        }>
                            "Views"
                            <span class="ml-1 text-gray-400">"(" {schema.views.len()} ")"</span>
                        </div>
                        {schema.views.into_iter().map(|view_name| {
                            view! {
                                <div class=move || {
                                    if dark_mode.get() {
                                        "flex items-center px-3 py-1 text-sm text-gray-300"
                                    } else {
                                        "flex items-center px-3 py-1 text-sm text-gray-700"
                                    }
                                }>
                                    <span class="mr-1">"◉"</span>
                                    <span class="truncate">{view_name}</span>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }}

            {if !schema.indexes.is_empty() {
                view! {
                    <div>
                        <div class=move || {
                            if dark_mode.get() {
                                "flex items-center px-2 py-1 text-xs font-medium text-gray-300 uppercase tracking-wider"
                            } else {
                                "flex items-center px-2 py-1 text-xs font-medium text-gray-600 uppercase tracking-wider"
                            }
                        }>
                            "Indexes"
                            <span class="ml-1 text-gray-400">"(" {schema.indexes.len()} ")"</span>
                        </div>
                        {schema.indexes.into_iter().map(|idx| {
                            view! {
                                <div class=move || {
                                    if dark_mode.get() {
                                        "flex items-center px-3 py-1 text-sm text-gray-300"
                                    } else {
                                        "flex items-center px-3 py-1 text-sm text-gray-700"
                                    }
                                }>
                                    <span class="mr-1">"⚡"</span>
                                    <span class="truncate">{idx.name}</span>
                                    {if idx.is_unique {
                                        view! { <span class="ml-1 text-xs text-blue-500">"UQ"</span> }.into_any()
                                    } else {
                                        view! {}.into_any()
                                    }}
                                </div>
                            }
                        }).collect_view()}
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }}

            {if !schema.triggers.is_empty() {
                view! {
                    <div>
                        <div class=move || {
                            if dark_mode.get() {
                                "flex items-center px-2 py-1 text-xs font-medium text-gray-300 uppercase tracking-wider"
                            } else {
                                "flex items-center px-2 py-1 text-xs font-medium text-gray-600 uppercase tracking-wider"
                            }
                        }>
                            "Triggers"
                            <span class="ml-1 text-gray-400">"(" {schema.triggers.len()} ")"</span>
                        </div>
                        {schema.triggers.into_iter().map(|trigger_name| {
                            view! {
                                <div class=move || {
                                    if dark_mode.get() {
                                        "flex items-center px-3 py-1 text-sm text-gray-300"
                                    } else {
                                        "flex items-center px-3 py-1 text-sm text-gray-700"
                                    }
                                }>
                                    <span class="mr-1">"↯"</span>
                                    <span class="truncate">{trigger_name}</span>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }}

            {if !schema.schemas.is_empty() {
                view! {
                    <div>
                        <div class=move || {
                            if dark_mode.get() {
                                "flex items-center px-2 py-1 text-xs font-medium text-gray-300 uppercase tracking-wider"
                            } else {
                                "flex items-center px-2 py-1 text-xs font-medium text-gray-600 uppercase tracking-wider"
                            }
                        }>
                            "Schemas"
                            <span class="ml-1 text-gray-400">"(" {schema.schemas.len()} ")"</span>
                        </div>
                        {schema.schemas.into_iter().map(|schema_name| {
                            view! {
                                <div class=move || {
                                    if dark_mode.get() {
                                        "flex items-center px-3 py-1 text-sm text-gray-300"
                                    } else {
                                        "flex items-center px-3 py-1 text-sm text-gray-700"
                                    }
                                }>
                                    <span class="mr-1">"⊡"</span>
                                    <span class="truncate">{schema_name}</span>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }}
        </div>
    }
}
