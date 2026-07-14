use crate::api::client;
use crate::state::AppState;
use crate::tab_manager::{
    Tab, TabKind, make_connections_tab_id, make_redb_browser_tab_id, make_table_structure_tab_id,
};
use leptos::prelude::*;
use sql_admin_api_types::{Connection, DatabaseType, RedbTableSummary, SchemaInfo};
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn DatabaseSidebar() -> impl IntoView {
    let app_state = use_context::<AppState>();
    let connections = app_state
        .as_ref()
        .map(|s| s.connections)
        .unwrap_or(RwSignal::new(Vec::<Connection>::new()));
    let (expanded_conn, set_expanded_conn) = signal(None::<String>);
    let (schemas, set_schemas) = signal(std::collections::HashMap::<String, SchemaInfo>::new());
    let (redb_tables, set_redb_tables) =
        signal(std::collections::HashMap::<String, Vec<RedbTableSummary>>::new());
    let (expanded_tables, set_expanded_tables) = signal(std::collections::HashSet::<String>::new());
    let (loading_schema, set_loading_schema) = signal(None::<String>);
    let (refreshing, set_refreshing) = signal(false);
    let (refresh_error, set_refresh_error) = signal(None::<String>);
    let (expanded_groups, set_expanded_groups) =
        signal(std::collections::HashSet::<DatabaseType>::new());

    let refresh_trigger = app_state
        .as_ref()
        .map(|s| s.refresh_trigger)
        .unwrap_or(RwSignal::new(0u32));

    Effect::new(move |_| {
        let _ = refresh_trigger.get();
        set_refreshing.set(true);
        set_refresh_error.set(None);

        let current_expanded = expanded_conn.get_untracked();
        if let Some(conn_id) = current_expanded {
            let current_schemas = schemas.get_untracked();
            let current_redb = redb_tables.get_untracked();
            let has_data =
                current_schemas.contains_key(&conn_id) || current_redb.contains_key(&conn_id);
            if has_data {
                set_loading_schema.set(Some(conn_id.clone()));

                let conns = connections.get_untracked();
                let is_redb = conns
                    .iter()
                    .any(|c| c.id == conn_id && c.database_type == DatabaseType::Redb);

                spawn_local(async move {
                    if is_redb {
                        match client::list_redb_tables(conn_id.clone()).await {
                            Ok(tables) => {
                                set_redb_tables.update(|s| {
                                    s.insert(conn_id, tables);
                                });
                            }
                            Err(e) => {
                                leptos::logging::error!("Failed to refresh redb tables: {}", e);
                            }
                        }
                    } else {
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
                    }
                    set_loading_schema.set(None);
                });
            }
        }

        set_refreshing.set(false);
    });

    let toggle_connection = move |id: String| {
        let current = expanded_conn.get();
        if current == Some(id.clone()) {
            set_expanded_conn.set(None);
        } else {
            set_expanded_conn.set(Some(id.clone()));

            let current_schemas = schemas.get();
            let current_redb = redb_tables.get();
            if current_schemas.contains_key(&id) || current_redb.contains_key(&id) {
                return;
            }

            set_loading_schema.set(Some(id.clone()));

            let conns = connections.get();
            let is_redb = conns
                .iter()
                .any(|c| c.id == id && c.database_type == DatabaseType::Redb);

            spawn_local(async move {
                if is_redb {
                    match client::list_redb_tables(id.clone()).await {
                        Ok(tables) => {
                            set_redb_tables.update(|s| {
                                s.insert(id, tables);
                            });
                        }
                        Err(e) => {
                            leptos::logging::error!("Failed to load redb tables: {}", e);
                        }
                    }
                } else {
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

    let refresh_connections = {
        let app_state = app_state.clone();
        let rt = refresh_trigger.clone();
        let cons = connections.clone();
        move || {
            set_refreshing.set(true);
            set_refresh_error.set(None);
            spawn_local(async move {
                match client::list_connections().await {
                    Ok(conns) => {
                        if let Some(ref s) = app_state {
                            s.connections.set(conns);
                        } else {
                            cons.set(conns);
                        }
                        rt.update(|v| *v = v.wrapping_add(1));
                    }
                    Err(e) => {
                        set_refresh_error.set(Some(format!("Failed to refresh connections: {}", e)));
                        leptos::logging::error!("Failed to refresh connections: {}", e);
                    }
                }
                set_refreshing.set(false);
            });
        }
    };

    view! {
        <div class="p-3">
            <div class="flex items-center justify-between mb-3">
                <h2 class="text-sm font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wider">"Explorer"</h2>
                <div class="flex items-center gap-2">
                    <button
                        class="p-1 rounded hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors"
                        title="Refresh connections"
                        on:click=refresh_connections
                    >
                        {move || if refreshing.get() {
                            view! { <span class="text-xs">"⟳"</span> }.into_any()
                        } else {
                            view! { <span class="text-sm">"↻"</span> }.into_any()
                        }}
                    </button>
                    {move || if refreshing.get() {
                        view! {
                            <span class="text-xs text-gray-400 animate-pulse">"Refreshing..."</span>
                        }.into_any()
                    } else {
                        let _: () = view! {};
                        ().into_any()
                    }}
                </div>
            </div>

            {move || {
                if let Some(err) = refresh_error.get() {
                    view! {
                        <div class="mb-2 px-2 py-1 text-xs bg-red-100 text-red-700 rounded">
                            {err}
                        </div>
                    }.into_any()
                } else {
                    let _: () = view! {};
                    ().into_any()
                }
            }}

            <div class="space-y-1">
                {move || {
                    let conns = connections.get();
                    let current_expanded = expanded_conn.get();
                    let current_schemas = schemas.get();
                    let current_redb = redb_tables.get();
                    let current_loading = loading_schema.get();
                    let current_expanded_tables = expanded_tables.get();
                    let current_expanded_groups = expanded_groups.get();

                    // Group connections by database type, preserving insertion order
                    let mut seen_types: Vec<DatabaseType> = Vec::new();
                    let mut groups: std::collections::HashMap<DatabaseType, Vec<&Connection>> = std::collections::HashMap::new();
                    for c in &conns {
                        if !groups.contains_key(&c.database_type) {
                            seen_types.push(c.database_type.clone());
                        }
                        groups.entry(c.database_type.clone()).or_default().push(c);
                    }

                    seen_types.into_iter().map(|db_type| {
                        let group_conns = groups.get(&db_type).cloned().unwrap_or_default();
                        let is_group_expanded = current_expanded_groups.contains(&db_type);
                        let db_type_label = db_type.to_string();
                        let db_type_clone = db_type.clone();
                        let group_count = group_conns.len();

                        let type_icon = match db_type {
                            DatabaseType::Sqlite => "🗄",
                            DatabaseType::Postgres => "🐘",
                            DatabaseType::Mysql => "🐬",
                            DatabaseType::Redb => "🔴",
                        };

                        view! {
                            <div>
                                <div
                                    class="flex items-center px-2 py-1.5 rounded cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700 text-sm font-semibold"
                                    on:click=move |_| {
                                        set_expanded_groups.update(|s| {
                                            if s.contains(&db_type_clone) {
                                                s.remove(&db_type_clone);
                                            } else {
                                                s.insert(db_type_clone.clone());
                                            }
                                        });
                                    }
                                >
                                    <span class="mr-1 text-xs text-gray-400">
                                        {if is_group_expanded { "▼" } else { "▶" }}
                                    </span>
                                    <span class="mr-1">{type_icon}</span>
                                    <span class="truncate text-gray-700 dark:text-gray-200 uppercase tracking-wider text-xs">{db_type_label}</span>
                                    <span class="ml-auto text-xs text-gray-400">"(" {group_count} ")"</span>
                                </div>

                                {if is_group_expanded {
                                    group_conns.into_iter().map(|conn| {
                                        let conn_id = conn.id.clone();
                                        let conn_id_1 = conn_id.clone();
                                        let conn_id_2 = conn_id.clone();
                                        let is_expanded = current_expanded == Some(conn_id.clone());
                                        let is_loading = current_loading == Some(conn_id.clone());
                                        let schema = current_schemas.get(&conn_id).cloned();
                                        let redb_tbls = current_redb.get(&conn_id).cloned();
                                        let conn_name = conn.name.clone();
                                        let db_type_enum = conn.database_type.clone();
                                        let expanded_tables_clone = current_expanded_tables.clone();
                                        let conn_name_for_tree = conn_name.clone();

                                        view! {
                                            <div>
                                                <div
                                                    class="flex items-center px-2 py-1.5 ml-2 rounded cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700 text-sm"
                                                    on:click=move |_| toggle_connection(conn_id_1.clone())
                                                >
                                                    <span class="mr-1 text-xs text-gray-400">
                                                        {if is_expanded { "▼" } else { "▶" }}
                                                    </span>
                                                    <span class="mr-2">"●"</span>
                                                    <span class="truncate font-medium text-gray-800 dark:text-gray-200">{conn_name}</span>
                                                </div>

                                                {if is_expanded {
                                                    if is_loading {
                                                        view! {
                                                            <div class="ml-8 py-2 text-xs text-gray-400">"Loading..."</div>
                                                        }.into_any()
                                                    } else if db_type_enum == DatabaseType::Redb {
                                                        view! {
                                                            <div class="ml-6">
                                                                <RedbSchemaTree
                                                                    conn_id=conn_id_2
                                                                    conn_name=conn_name_for_tree
                                                                    tables=redb_tbls.unwrap_or_default()
                                                                />
                                                            </div>
                                                        }.into_any()
                                                    } else if let Some(schema) = schema {
                                                        view! {
                                                            <div class="ml-6">
                                                                <SchemaTree
                                                                    conn_id=conn_id_2
                                                                    schema=schema
                                                                    expanded_tables=expanded_tables_clone
                                                                    set_expanded_tables=set_expanded_tables
                                                                />
                                                            </div>
                                                        }.into_any()
                                                    } else {
                                                        let _: () = view! {};
                                                        ().into_any()
                                                    }
                                                } else {
                                                    let _: () = view! {};
                                                    ().into_any()
                                                }}
                                            </div>
                                        }
                                    }).collect_view().into_any()
                                } else {
                                    let _: () = view! {};
                                    ().into_any()
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
fn RedbSchemaTree(
    conn_id: String,
    conn_name: String,
    tables: Vec<RedbTableSummary>,
) -> impl IntoView {
    let app_state = use_context::<AppState>();
    let tab_manager = app_state.as_ref().map(|s| s.tab_manager);

    view! {
        <div class="space-y-0.5">
            <div>
                <div class="flex items-center px-2 py-1 text-xs font-medium text-gray-600 dark:text-gray-300 uppercase tracking-wider">
                    <span class="mr-1">"▼"</span>
                    "Tables"
                    <span class="ml-1 text-gray-400">"(" {tables.len()} ")"</span>
                </div>
                {if tables.is_empty() {
                    view! {
                        <div class="px-3 py-2 text-xs text-gray-400 dark:text-gray-500">"No tables found"</div>
                    }.into_any()
                } else {
                    tables.into_iter().map(|table| {
                        let table_name = table.name.clone();
                        let key_count = table.key_count;
                        let tm = tab_manager;
                        let app_state_sidebar = app_state.clone();
                        let conn_id_for_tab = conn_id.clone();
                        let conn_name_for_tab = conn_name.clone();
                        let table_name_for_tab = table_name.clone();

                        view! {
                            <div>
                                <div
                                    class="flex items-center px-3 py-1 rounded cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700 text-sm"
                                    on:click={
                                        let app_state_sidebar = app_state_sidebar.clone();
                                        move |_| {
                                            if let Some(tm) = tm {
                                                let route = format!("/redb/{}?table={}", conn_id_for_tab, table_name_for_tab);
                                                tm.update(|tm| {
                                                    let tab = Tab {
                                                        id: make_redb_browser_tab_id(conn_id_for_tab.clone()),
                                                        kind: TabKind::RedbBrowser {
                                                            connection_id: conn_id_for_tab.clone(),
                                                        },
                                                        title: format!("Redb: {}", conn_name_for_tab),
                                                    };
                                                    tm.ensure_tab(tab);
                                                });
                                                if let Some(ref s) = app_state_sidebar {
                                                    s.pending_navigation.set(Some(route));
                                                }
                                            }
                                        }
                                    }
                                >
                                    <span class="mr-1 text-orange-400">"\u{1f5c2}\u{fe0f}"</span>
                                    <span
                                        class="ml-1 truncate text-gray-700 dark:text-gray-300 hover:text-orange-600 dark:hover:text-orange-400 cursor-pointer"
                                    >{table_name_for_tab.clone()}</span>
                                    <span class="ml-auto text-xs text-gray-400">
                                        {format!("{} keys", key_count)}
                                    </span>
                                </div>
                            </div>
                        }
                    }).collect_view().into_any()
                }}
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
    let tab_manager = app_state.as_ref().map(|s| s.tab_manager);

    view! {
        <div class="space-y-0.5">
            {if !schema.tables.is_empty() {
                view! {
                    <div>
                        <div class="flex items-center px-2 py-1 text-xs font-medium text-gray-600 dark:text-gray-300 uppercase tracking-wider">
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
                            let tm = tab_manager;
                            let app_state_sidebar = app_state.clone();
                            let conn_id_for_tab = conn_id.clone();
                            let table_name_for_tab = table_name.clone();

                            view! {
                                <div>
                                    <div
                                        class="flex items-center px-3 py-1 rounded cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700 text-sm"
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
                                        <span class="text-gray-700 dark:text-gray-300">"⊞"</span>
                                        <span
                                            class="ml-1 truncate text-gray-700 dark:text-gray-300 hover:text-blue-600 dark:hover:text-blue-400 cursor-pointer"
                                            on:click={
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
                                                        <div class="flex items-center px-2 py-0.5 text-xs text-gray-600 dark:text-gray-400">
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
                                        let _: () = view! {};
                                        ().into_any()
                                    }}
                                </div>
                            }
                        }).collect_view()}
                    </div>
                }.into_any()
            } else {
                let _: () = view! {};
                ().into_any()
            }}

            {if !schema.views.is_empty() {
                view! {
                    <div>
                        <div class="flex items-center px-2 py-1 text-xs font-medium text-gray-600 dark:text-gray-300 uppercase tracking-wider">
                            "Views"
                            <span class="ml-1 text-gray-400">"(" {schema.views.len()} ")"</span>
                        </div>
                        {schema.views.into_iter().map(|view_name| {
                            view! {
                                <div class="flex items-center px-3 py-1 text-sm text-gray-700 dark:text-gray-300">
                                    <span class="mr-1">"◉"</span>
                                    <span class="truncate">{view_name}</span>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                }.into_any()
            } else {
                let _: () = view! {};
                ().into_any()
            }}

            {if !schema.indexes.is_empty() {
                view! {
                    <div>
                        <div class="flex items-center px-2 py-1 text-xs font-medium text-gray-600 dark:text-gray-300 uppercase tracking-wider">
                            "Indexes"
                            <span class="ml-1 text-gray-400">"(" {schema.indexes.len()} ")"</span>
                        </div>
                        {schema.indexes.into_iter().map(|idx| {
                            view! {
                                <div class="flex items-center px-3 py-1 text-sm text-gray-700 dark:text-gray-300">
                                    <span class="mr-1">"⚡"</span>
                                    <span class="truncate">{idx.name}</span>
                                    {if idx.is_unique {
                                        view! { <span class="ml-1 text-xs text-blue-500">"UQ"</span> }.into_any()
                                    } else {
                                        let _: () = view! {};
                                        ().into_any()
                                    }}
                                </div>
                            }
                        }).collect_view()}
                    </div>
                }.into_any()
            } else {
                let _: () = view! {};
                ().into_any()
            }}

            {if !schema.triggers.is_empty() {
                view! {
                    <div>
                        <div class="flex items-center px-2 py-1 text-xs font-medium text-gray-600 dark:text-gray-300 uppercase tracking-wider">
                            "Triggers"
                            <span class="ml-1 text-gray-400">"(" {schema.triggers.len()} ")"</span>
                        </div>
                        {schema.triggers.into_iter().map(|trigger_name| {
                            view! {
                                <div class="flex items-center px-3 py-1 text-sm text-gray-700 dark:text-gray-300">
                                    <span class="mr-1">"↯"</span>
                                    <span class="truncate">{trigger_name}</span>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                }.into_any()
            } else {
                let _: () = view! {};
                ().into_any()
            }}

            {if !schema.schemas.is_empty() {
                view! {
                    <div>
                        <div class="flex items-center px-2 py-1 text-xs font-medium text-gray-600 dark:text-gray-300 uppercase tracking-wider">
                            "Schemas"
                            <span class="ml-1 text-gray-400">"(" {schema.schemas.len()} ")"</span>
                        </div>
                        {schema.schemas.into_iter().map(|schema_name| {
                            view! {
                                <div class="flex items-center px-3 py-1 text-sm text-gray-700 dark:text-gray-300">
                                    <span class="mr-1">"⊡"</span>
                                    <span class="truncate">{schema_name}</span>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                }.into_any()
            } else {
                let _: () = view! {};
                ().into_any()
            }}
        </div>
    }
}
