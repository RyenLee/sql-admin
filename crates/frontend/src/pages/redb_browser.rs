use crate::components::{redb_key_viewer::RedbKeyViewer, redb_table_list::RedbTableList};
use crate::state::use_app_state;
use leptos::prelude::*;
use leptos_router::hooks::{use_params_map, use_query_map};
use sql_admin_api_types::DatabaseType;

#[component]
pub fn RedbBrowserPage() -> impl IntoView {
    let app_state = use_app_state();
    let params = use_params_map();
    let query_map = use_query_map();

    app_state.active_db_type.set(Some(DatabaseType::Redb));

    let connection_id = move || {
        params
            .get()
            .get("conn_id")
            .unwrap_or_default()
    };

    let connection_name = move || {
        let conn_id = connection_id();
        app_state.connections.get()
            .iter()
            .find(|c| c.id == conn_id)
            .map(|c| c.name.clone())
            .unwrap_or_else(|| conn_id)
    };

    let (selected_table, set_selected_table) = signal(None::<String>);

    // Auto-select table from query parameter
    {
        Effect::new(move |_| {
            let table_name = query_map
                .get()
                .get("table");
            if let Some(table) = table_name
                && !table.is_empty() {
                    set_selected_table.set(Some(table));
                }
        });
    }

    view! {
        <div class="flex h-full">
            <div class="w-56 flex-shrink-0 border-r overflow-y-auto border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800">
                {move || {
                    let conn_id = connection_id();
                    if conn_id.is_empty() {
                        view! {
                            <div class="px-3 py-4 text-center">
                                <div class="text-gray-500 dark:text-gray-400 text-sm">"No connection selected"</div>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <RedbTableList
                                connection_id=conn_id
                                on_select=Callback::new(move |table: String| {
                                    set_selected_table.set(Some(table));
                                })
                            />
                        }.into_any()
                    }
                }}
            </div>

            <div class="flex-1 flex flex-col overflow-hidden">
                {move || {
                    match selected_table.get() {
                        Some(table) => {
                            let conn_id = connection_id();
                            view! {
                                <div class="px-4 py-2 border-b font-medium border-gray-100 dark:border-gray-700 text-gray-800 dark:text-gray-200 bg-white dark:bg-gray-800">
                                    {connection_name()} " / " {table.clone()}
                                </div>
                                <div class="flex-1 overflow-hidden">
                                    <RedbKeyViewer connection_id=conn_id table_name=table />
                                </div>
                            }.into_any()
                        }
                        None => {
                            view! {
                                <div class="flex items-center justify-center h-full">
                                    <div class="text-center text-gray-500 dark:text-gray-400">
                                        <div class="text-4xl mb-2">"\u{1f511}"</div>
                                        <div class="text-sm">"Select a table to browse keys"</div>
                                    </div>
                                </div>
                            }.into_any()
                        }
                    }
                }}
            </div>
        </div>
    }
}
