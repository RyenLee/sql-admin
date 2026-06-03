use crate::api::client;
use crate::components::connection_form::ConnectionForm;
use crate::state::AppState;
use crate::tab_manager::{Tab, TabKind, make_connections_tab_id};
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use sql_admin_api_types::{Connection, CreateConnectionRequest};
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn Connections() -> impl IntoView {
    let app_state = use_context::<AppState>();
    let tab_manager = app_state.as_ref().map(|s| s.tab_manager);
    let connections = app_state
        .as_ref()
        .map(|s| s.connections)
        .unwrap_or(RwSignal::new(Vec::<Connection>::new()));
    let (show_form, set_show_form) = signal(false);
    let (editing_connection, set_editing_connection) = signal(None::<Connection>);
    let (testing_connection, set_testing_connection) = signal(None::<String>);
    let (test_result, set_test_result) = signal(None::<(String, bool, String)>);
    let (error_message, set_error_message) = signal(None::<String>);

    let refresh_trigger = app_state
        .as_ref()
        .map(|s| s.refresh_trigger)
        .unwrap_or(RwSignal::new(0u32));

    let mounted = StoredValue::new(false);
    Effect::new(move |_| {
        if !mounted.get_value() {
            mounted.set_value(true);
            
            if let Some(tm) = tab_manager {
                tm.update(|tm| {
                    let tab = Tab {
                        id: make_connections_tab_id(),
                        kind: TabKind::Connections,
                        title: "Connections".to_string(),
                    };
                    tm.ensure_tab(tab);
                });
            }
        }
    });

    Effect::new(move |_| {
        let _ = refresh_trigger.get();
    });

    let app_state_for_submit = app_state.clone();
    let on_submit_handler = move |conn: CreateConnectionRequest| {
        let rt = refresh_trigger;
        let editing = editing_connection.get();
        let app_state = app_state_for_submit.clone();
        let set_error = set_error_message;
        spawn_local(async move {
            let result = if let Some(existing) = editing {
                client::update_connection(existing.id.clone(), conn).await
            } else {
                client::create_connection(conn).await
            };
            if result.is_ok() {
                set_error.set(None);
                if let Some(ref s) = app_state {
                    match client::list_connections().await {
                        Ok(conns) => s.connections.set(conns),
                        Err(e) => leptos::logging::error!("Failed to reload connections: {}", e),
                    }
                }
                set_editing_connection.set(None);
                set_show_form.set(false);
                rt.update(|v| *v = v.wrapping_add(1));
            } else if let Err(e) = result {
                set_error.set(Some(e));
            }
        });
    };

    let handle_delete = move |id: String| {
        let rt = refresh_trigger;
        let app_state = app_state.clone();
        spawn_local(async move {
            if client::delete_connection(id).await.is_ok() {
                if let Some(ref s) = app_state {
                    match client::list_connections().await {
                        Ok(conns) => s.connections.set(conns),
                        Err(e) => leptos::logging::error!("Failed to reload connections: {}", e),
                    }
                }
                rt.update(|v| *v = v.wrapping_add(1));
            }
        });
    };

    let handle_test = move |id: String| {
        set_testing_connection.set(Some(id.clone()));
        spawn_local(async move {
            match client::test_connection(id.clone()).await {
                Ok(msg) => {
                    set_test_result.set(Some((id, true, msg)));
                }
                Err(e) => {
                    set_test_result.set(Some((id, false, e)));
                }
            }
            set_testing_connection.set(None);

            TimeoutFuture::new(3000).await;
            set_test_result.set(None);
        });
    };

    view! {
        <div>
            <div class="flex justify-between items-center mb-6">
                <h1 class="text-2xl font-bold text-gray-800 dark:text-gray-100">"Database Connections"</h1>
                <button
                    class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
                    on:click=move |_| {
                        set_show_form.set(true);
                        set_editing_connection.set(None);
                    }
                >
                    "Add Connection"
                </button>
            </div>

            {move || if show_form.get() {
                view! {
                    <div>
                        {move || {
                            if let Some(ref err) = error_message.get() {
                                view! {
                                    <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
                                        {err.clone()}
                                    </div>
                                }.into_any()
                            } else {
                                ().into_any()
                            }
                        }}
                        <ConnectionForm
                            connection=editing_connection.get()
                            on_submit=on_submit_handler.clone()
                            on_cancel=move || {
                                set_show_form.set(false);
                                set_error_message.set(None);
                            }
                        />
                    </div>
                }.into_any()
            } else {
                let handle_delete = handle_delete.clone();
                view! {
                    <div class="grid gap-4">
                        {move || {
                            let conns = connections.get();
                            if conns.is_empty() {
                                view! {
                                    <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-8 text-center">
                                        <p class="text-gray-500 dark:text-gray-400">"No connections yet. Create one to get started!"</p>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    {conns.into_iter().map(|conn| {
                                        let conn_clone = conn.clone();
                                        let conn_id = conn.id.clone();
                                        let conn_id_1 = conn_id.clone();
                                        let conn_id_2 = conn_id.clone();
                                        let conn_id_3 = conn_id.clone();
                                        let conn_id_4 = conn_id.clone();
                                        let conn_id_5 = conn_id.clone();
                                        let conn_id_6 = conn_id.clone();
                                        let db_type = conn.database_type.clone();
                                        let handle_delete = handle_delete.clone();
                                        let testing = testing_connection;
                                        let testing_1 = testing;
                                        let testing_2 = testing;
                                        let testing_3 = testing;
                                        let test_res = test_result;

                                        view! {
                                            <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
                                                <div class="flex justify-between items-start">
                                                    <div>
                                                        <div class="flex items-center gap-2">
                                                            <h3 class="font-semibold text-lg dark:text-gray-100">{conn.name.clone()}</h3>
                                                            <span class=format!("px-2 py-0.5 text-xs rounded-full font-medium {}",
                                                                match db_type {
                                                                    sql_admin_api_types::DatabaseType::Sqlite => "bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-300",
                                                                    sql_admin_api_types::DatabaseType::Postgres => "bg-indigo-100 dark:bg-indigo-900 text-indigo-700 dark:text-indigo-300",
                                                                    sql_admin_api_types::DatabaseType::Mysql => "bg-orange-100 dark:bg-orange-900 text-orange-700 dark:text-orange-300",
                                                                    sql_admin_api_types::DatabaseType::Redb => "bg-purple-100 dark:bg-purple-900 text-purple-700 dark:text-purple-300",
                                                                }
                                                            )>{db_type.to_string()}</span>
                                                        </div>
                                                        <p class="text-sm text-gray-500">
                                                            {format!("{}://{}:{}", conn.database_type, conn.host, conn.port)}
                                                        </p>
                                                        <p class="text-sm text-gray-500">{conn.database.clone()}</p>
                                                        {move || {
                                                            let res = test_res.get().filter(|(id, _, _)| id == &conn_id_1);
                                                            if let Some((_, success, msg)) = res {
                                                                view! {
                                                                    <p class=if success { "text-sm text-green-600" } else { "text-sm text-red-600" }>
                                                                        {msg}
                                                                    </p>
                                                                }.into_any()
                                                            } else {
                                                                let _: () = view! {};
                                                                ().into_any()
                                                            }
                                                        }}
                                                    </div>
                                                    <div class="flex space-x-2">
                                                        <button
                                                            disabled=move || testing_1.get() == Some(conn_id_2.clone())
                                                            class=move || format!(
                                                                "px-3 py-1 text-sm rounded {}",
                                                                if testing_2.get() == Some(conn_id_3.clone()) {
                                                                    "bg-gray-100 dark:bg-gray-700 text-gray-400 dark:text-gray-400 cursor-not-allowed"
                                                                } else {
                                                                    "bg-green-100 dark:bg-green-900 text-green-600 dark:text-green-300 hover:bg-green-200 dark:hover:bg-green-800"
                                                                }
                                                            )
                                                            on:click=move |_| handle_test(conn_id_4.clone())
                                                        >
                                                            {move || if testing_3.get() == Some(conn_id_5.clone()) { "Testing..." } else { "Test" }}
                                                        </button>
                                                        <button
                                                            class="px-3 py-1 text-sm bg-gray-100 dark:bg-gray-700 text-gray-300 dark:text-gray-300 rounded hover:bg-gray-200 dark:hover:bg-gray-600"
                                                            on:click=move |_| {
                                                                set_editing_connection.set(Some(conn_clone.clone()));
                                                                set_show_form.set(true);
                                                            }
                                                        >
                                                            "Edit"
                                                        </button>
                                                        <button
                                                            class="px-3 py-1 text-sm bg-red-100 text-red-600 rounded hover:bg-red-200"
                                                            on:click=move |_| handle_delete(conn_id_6.clone())
                                                        >
                                                            "Delete"
                                                        </button>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()}
                                }.into_any()
                            }
                        }}
                    </div>
                }.into_any()
            }}
        </div>
    }
}
