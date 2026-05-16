use crate::api::client;
use crate::components::connection_form::ConnectionForm;
use crate::state::AppState;
use crate::tab_manager::{Tab, TabKind, make_connections_tab_id};
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use sql_admin_shared::{Connection, CreateConnectionRequest};
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn Connections() -> impl IntoView {
    let app_state = use_context::<AppState>();
    let dark_mode = app_state
        .as_ref()
        .map(|s| s.dark_mode)
        .unwrap_or(RwSignal::new(false));
    let tab_manager = app_state.as_ref().map(|s| s.tab_manager);
    let (connections, set_connections) = signal(Vec::<Connection>::new());
    let (show_form, set_show_form) = signal(false);
    let (editing_connection, set_editing_connection) = signal(None::<Connection>);
    let (testing_connection, set_testing_connection) = signal(None::<String>);
    let (test_result, set_test_result) = signal(None::<(String, bool, String)>);

    let refresh_trigger = app_state
        .as_ref()
        .map(|s| s.refresh_trigger)
        .unwrap_or(RwSignal::new(0u32));

    let load_connections = move || {
        spawn_local(async move {
            match client::list_connections().await {
                Ok(conns) => set_connections.set(conns),
                Err(e) => leptos::logging::error!("Failed to load connections: {}", e),
            }
        });
    };

    let mounted = RwSignal::new(false);
    Effect::new(move |_| {
        if !mounted.get() {
            mounted.set(true);
            load_connections();
            
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
        load_connections();
    });

    let on_submit_handler = move |conn: CreateConnectionRequest| {
        let rt = refresh_trigger;
        let editing = editing_connection.get();
        spawn_local(async move {
            let result = if let Some(existing) = editing {
                client::update_connection(existing.id.clone(), conn).await
            } else {
                client::create_connection(conn).await
            };
            if let Ok(_) = result {
                match client::list_connections().await {
                    Ok(conns) => set_connections.set(conns),
                    Err(e) => leptos::logging::error!("Failed to reload connections: {}", e),
                }
                set_editing_connection.set(None);
                set_show_form.set(false);
                rt.update(|v| *v = v.wrapping_add(1));
            }
        });
    };

    let handle_delete = move |id: String| {
        let rt = refresh_trigger;
        spawn_local(async move {
            if let Ok(_) = client::delete_connection(id).await {
                match client::list_connections().await {
                    Ok(conns) => set_connections.set(conns),
                    Err(e) => leptos::logging::error!("Failed to reload connections: {}", e),
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
                <h1 class=move || {
                    if dark_mode.get() { "text-2xl font-bold text-gray-100" } else { "text-2xl font-bold text-gray-800" }
                }>"Database Connections"</h1>
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
                    <ConnectionForm
                        connection=editing_connection.get()
                        on_submit=on_submit_handler
                        on_cancel=move || set_show_form.set(false)
                    />
                }.into_any()
            } else {
                view! {
                    <div class="grid gap-4">
                        {move || {
                            let conns = connections.get();
                            let dm = dark_mode;
                            if conns.is_empty() {
                                view! {
                                    <div class=move || {
                                        if dm.get() {
                                            "bg-gray-800 rounded-lg shadow p-8 text-center"
                                        } else {
                                            "bg-white rounded-lg shadow p-8 text-center"
                                        }
                                    }>
                                        <p class=move || {
                                            if dm.get() { "text-gray-400" } else { "text-gray-500" }
                                        }>"No connections yet. Create one to get started!"</p>
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
                                        let testing = testing_connection.clone();
                                        let testing_1 = testing.clone();
                                        let testing_2 = testing.clone();
                                        let testing_3 = testing.clone();
                                        let test_res = test_result.clone();
                                        let dm = dark_mode;

                                        view! {
                                            <div class=move || {
                                                if dm.get() {
                                                    "bg-gray-800 rounded-lg shadow p-4"
                                                } else {
                                                    "bg-white rounded-lg shadow p-4"
                                                }
                                            }>
                                                <div class="flex justify-between items-start">
                                                    <div>
                                                        <h3 class=move || {
                                                            if dm.get() { "font-semibold text-lg text-gray-100" } else { "font-semibold text-lg" }
                                                        }>{conn.name.clone()}</h3>
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
                                                                view! {}.into_any()
                                                            }
                                                        }}
                                                    </div>
                                                    <div class="flex space-x-2">
                                                        <button
                                                            disabled=move || testing_1.get() == Some(conn_id_2.clone())
                                                            class=move || format!(
                                                                "px-3 py-1 text-sm rounded {}",
                                                                if testing_2.get() == Some(conn_id_3.clone()) {
                                                                    if dm.get() { "bg-gray-700 text-gray-400 cursor-not-allowed" }
                                                                    else { "bg-gray-100 cursor-not-allowed" }
                                                                } else {
                                                                    if dm.get() { "bg-green-900 text-green-300 hover:bg-green-800" }
                                                                    else { "bg-green-100 text-green-600 hover:bg-green-200" }
                                                                }
                                                            )
                                                            on:click=move |_| handle_test(conn_id_4.clone())
                                                        >
                                                            {move || if testing_3.get() == Some(conn_id_5.clone()) { "Testing..." } else { "Test" }}
                                                        </button>
                                                        <button
                                                            class=move || {
                                                                if dm.get() {
                                                                    "px-3 py-1 text-sm bg-gray-700 text-gray-300 rounded hover:bg-gray-600"
                                                                } else {
                                                                    "px-3 py-1 text-sm bg-gray-100 rounded hover:bg-gray-200"
                                                                }
                                                            }
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
