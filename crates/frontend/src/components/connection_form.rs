use crate::api::client;
use leptos::prelude::*;
use sql_admin_api_types::{Connection, CreateConnectionRequest, DatabaseType};
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn ConnectionForm(
    connection: Option<Connection>,
    on_submit: impl Fn(CreateConnectionRequest) + 'static,
    on_cancel: impl Fn() + 'static,
) -> impl IntoView {
    let (name, set_name) = signal(
        connection
            .as_ref()
            .map(|c| c.name.clone())
            .unwrap_or_default(),
    );
    let (selected_type, set_selected_type) = signal(
        match connection.as_ref().map(|c| &c.database_type) {
            Some(DatabaseType::Postgres) => "postgres",
            Some(DatabaseType::Mysql) => "mysql",
            Some(DatabaseType::Sqlite) => "sqlite",
            Some(DatabaseType::Redb) => "redb",
            None => "postgres",
        }
        .to_string(),
    );
    let (host, set_host) = signal(
        connection
            .as_ref()
            .map(|c| c.host.clone())
            .unwrap_or("localhost".to_string()),
    );
    let (port, set_port) = signal(
        connection
            .as_ref()
            .map(|c| c.port.to_string())
            .unwrap_or("5432".to_string()),
    );
    let (database, set_database) = signal(
        connection
            .as_ref()
            .map(|c| c.database.clone())
            .unwrap_or_default(),
    );
    let (username, set_username) = signal(
        connection
            .as_ref()
            .map(|c| c.username.clone())
            .unwrap_or_default(),
    );
    let (password, set_password) = signal(
        connection
            .as_ref()
            .and_then(|c| c.password.clone())
            .unwrap_or_default(),
    );

    let (testing, set_testing) = signal(false);
    let (test_result, set_test_result) = signal(None::<(bool, String)>);

    let build_request = move || {
        let db_type = match selected_type.get().as_str() {
            "mysql" => DatabaseType::Mysql,
            "sqlite" => DatabaseType::Sqlite,
            "redb" => DatabaseType::Redb,
            _ => DatabaseType::Postgres,
        };

        let port_value = match port.get().parse::<u16>() {
            Ok(p) => p,
            Err(_) => match db_type {
                DatabaseType::Postgres => 5432,
                DatabaseType::Mysql => 3306,
                DatabaseType::Sqlite => 0,
                DatabaseType::Redb => 0,
            },
        };

        CreateConnectionRequest {
            name: name.get(),
            database_type: db_type,
            host: host.get(),
            port: port_value,
            database: database.get(),
            username: username.get(),
            password: if password.get().is_empty() {
                None
            } else {
                Some(password.get())
            },
        }
    };

    let handle_test = move |_| {
        set_testing.set(true);
        set_test_result.set(None);
        let req = build_request();
        spawn_local(async move {
            match client::test_connection_request(req).await {
                Ok(msg) => set_test_result.set(Some((true, msg))),
                Err(e) => set_test_result.set(Some((false, e))),
            }
            set_testing.set(false);
        });
    };

    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let req = build_request();
        on_submit(req);
    };

    view! {
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-6 mb-6">
            <h2 class="text-xl font-semibold mb-4 dark:text-gray-100">
                {move || if connection.is_some() { "Edit Connection" } else { "Add New Connection" }}
            </h2>

            <form on:submit=handle_submit class="space-y-4">
                <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">"Name"</label>
                    <input
                        type="text"
                        class="w-full px-3 py-2 border dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                        placeholder="Connection name"
                        prop:value=name
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                        required
                    />
                </div>

                <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">"Database Type"</label>
                    <select
                        class="w-full px-3 py-2 border dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                        prop:value=selected_type
                        on:change=move |ev| {
                            let val = event_target_value(&ev);
                            set_selected_type.set(val.clone());
                            match val.as_str() {
                                "mysql" => set_port.set("3306".to_string()),
                                "sqlite" => set_port.set("0".to_string()),
                                "redb" => set_port.set("0".to_string()),
                                _ => set_port.set("5432".to_string()),
                            }
                        }
                    >
                        <option value="postgres">"PostgreSQL"</option>
                        <option value="mysql">"MySQL"</option>
                        <option value="sqlite">"SQLite"</option>
                        <option value="redb">"redb (K/V)"</option>
                    </select>
                </div>

                {move || {
                    let db_type = selected_type.get();
                    if db_type == "sqlite" || db_type == "redb" {
                        let is_redb = db_type == "redb";
                        let label = if is_redb { "redb 文件路径" } else { "Database File Path" };
                        let placeholder = if is_redb { "e.g. ./data/cache.redb" } else { "e.g. ./data/mydb.db" };
                        view! {
                            <div>
                                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">{label}</label>
                                <input
                                    type="text"
                                    class="w-full px-3 py-2 border dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    placeholder=placeholder
                                    prop:value=database
                                    on:input=move |ev| set_database.set(event_target_value(&ev))
                                />
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="grid grid-cols-2 gap-4">
                                <div>
                                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">"Host"</label>
                                    <input
                                        type="text"
                                        class="w-full px-3 py-2 border dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                        placeholder="localhost"
                                        prop:value=host
                                        on:input=move |ev| set_host.set(event_target_value(&ev))
                                    />
                                </div>
                                <div>
                                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">"Port"</label>
                                    <input
                                        type="number"
                                        class="w-full px-3 py-2 border dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                        placeholder="5432"
                                        prop:value=port
                                        on:input=move |ev| set_port.set(event_target_value(&ev))
                                    />
                                </div>
                            </div>
                            <div>
                                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">"Database"</label>
                                <input
                                    type="text"
                                    class="w-full px-3 py-2 border dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    placeholder="database_name"
                                    prop:value=database
                                    on:input=move |ev| set_database.set(event_target_value(&ev))
                                />
                            </div>
                            <div>
                                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">"Username"</label>
                                <input
                                    type="text"
                                    class="w-full px-3 py-2 border dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    placeholder="username"
                                    prop:value=username
                                    on:input=move |ev| set_username.set(event_target_value(&ev))
                                />
                            </div>
                            <div>
                                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">"Password"</label>
                                <input
                                    type="password"
                                    class="w-full px-3 py-2 border dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    placeholder="password (optional)"
                                    prop:value=password
                                    on:input=move |ev| set_password.set(event_target_value(&ev))
                                />
                            </div>
                        }.into_any()
                    }
                }}

                {move || {
                    let res = test_result.get();
                    if let Some((success, msg)) = res {
                        view! {
                            <div class=if success {
                                "flex items-center p-3 rounded-md bg-green-50 border border-green-200"
                            } else {
                                "flex items-center p-3 rounded-md bg-red-50 border border-red-200"
                            }>
                                <svg
                                    class=if success { "w-5 h-5 mr-2 text-green-500 flex-shrink-0" } else { "w-5 h-5 mr-2 text-red-500 flex-shrink-0" }
                                    fill="none" stroke="currentColor" viewBox="0 0 24 24"
                                >
                                    {if success {
                                        view! {
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                                        }.into_any()
                                    }}
                                </svg>
                                <span class=if success { "text-sm text-green-700" } else { "text-sm text-red-700" }>
                                    {msg}
                                </span>
                            </div>
                        }.into_any()
                    } else {
                        let _: () = view! {};
                        ().into_any()
                    }
                }}

                <div class="flex space-x-4">
                    <button
                        type="submit"
                        class="flex-1 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
                    >
                        "Save"
                    </button>
                    <button
                        type="button"
                        disabled=move || testing.get()
                        class=move || {
                            let base = if testing.get() {
                                "px-4 py-2 bg-gray-400 text-white rounded-lg cursor-not-allowed"
                            } else {
                                "px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700"
                            };
                            base.to_string()
                        }
                        on:click=handle_test
                    >
                        {move || if testing.get() { "Testing..." } else { "Test Connection" }}
                    </button>
                    <button
                        type="button"
                        class="px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-600"
                        on:click=move |_| on_cancel()
                    >
                        "Cancel"
                    </button>
                </div>
            </form>
        </div>
    }
}
