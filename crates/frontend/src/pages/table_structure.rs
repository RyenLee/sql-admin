use crate::api::client;
use crate::state::use_app_state;
use crate::tab_manager::{Tab, TabKind, make_table_structure_tab_id};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use sql_admin_shared::TableDef;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn TableStructure() -> impl IntoView {
    let params = use_params_map();
    let app_state = use_app_state();
    let dm = move || app_state.dark_mode.get();
    let (table_def, set_table_def) = signal(None::<TableDef>);
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);

    Effect::new(move |_| {
        let conn_id_str = params
            .get()
            .get("conn_id")
            .map(|s| s.clone())
            .unwrap_or_default();
        let table_name = params
            .get()
            .get("table")
            .map(|s| s.clone())
            .unwrap_or_default();

        if conn_id_str.is_empty() || table_name.is_empty() {
            set_error.set(Some("Missing connection ID or table name".to_string()));
            set_loading.set(false);
            return;
        }

        let tab_id = make_table_structure_tab_id(conn_id_str.clone(), &table_name);
        let tab_title = table_name.clone();
        let conn_id_for_tab = conn_id_str.clone();
        let table_for_tab = table_name.clone();

        app_state.tab_manager.update(|tm| {
            let tab = Tab {
                id: tab_id,
                kind: TabKind::TableStructure {
                    connection_id: conn_id_for_tab,
                    table_name: table_for_tab,
                },
                title: tab_title,
            };
            tm.ensure_tab(tab);
        });

        let conn_id_for_query = conn_id_str.clone();
        let table_for_query = table_name.clone();

        spawn_local(async move {
            match client::get_table_def(conn_id_for_query.clone(), &table_for_query).await {
                Ok(def) => {
                    app_state.status.update(|s| {
                        s.connected_db = Some(table_for_query.clone());
                    });
                    set_table_def.set(Some(def));
                }
                Err(e) => {
                    set_error.set(Some(e));
                }
            }
            set_loading.set(false);
        });
    });

    view! {
        <div class="h-full">
            {move || if loading.get() {
                view! {
                    <div class="flex items-center justify-center h-64 text-gray-400">
                        "Loading table structure..."
                    </div>
                }.into_any()
            } else if let Some(err) = error.get() {
                view! {
                    <div class="bg-red-50 border border-red-200 rounded-lg p-4">
                        <p class="text-red-700">{err}</p>
                    </div>
                }.into_any()
            } else if let Some(def) = table_def.get() {
                view! {
                    <div class="space-y-4">
                        <div class="flex items-center gap-3">
                            <h1 class=move || {
                                if dm() { "text-xl font-bold text-gray-100" } else { "text-xl font-bold text-gray-800" }
                            }>{def.name.clone()}</h1>
                            {if let Some(count) = def.row_count {
                                view! {
                                    <span class="text-sm text-gray-500">
                                        "(" {count} " rows)"
                                    </span>
                                }.into_any()
                            } else {
                                view! {}.into_any()
                            }}
                        </div>

                        <div class=move || {
                            if dm() { "bg-gray-800 rounded-lg shadow" } else { "bg-white rounded-lg shadow" }
                        }>
                            <div class=move || {
                                if dm() { "p-3 border-b border-gray-700" } else { "p-3 border-b" }
                            }>
                                <h2 class=move || {
                                    if dm() { "font-semibold text-sm text-gray-200" } else { "font-semibold text-sm text-gray-700" }
                                }>"Columns"</h2>
                            </div>
                            <div class="overflow-auto">
                                <table class="w-full text-sm">
                                    <thead class=move || {
                                        if dm() { "bg-gray-700" } else { "bg-gray-50" }
                                    }>
                                        <tr>
                                            <th class=move || {
                                                if dm() { "px-3 py-2 text-left font-medium text-gray-300" } else { "px-3 py-2 text-left font-medium text-gray-600" }
                                            }>"#"</th>
                                            <th class=move || {
                                                if dm() { "px-3 py-2 text-left font-medium text-gray-300" } else { "px-3 py-2 text-left font-medium text-gray-600" }
                                            }>"Name"</th>
                                            <th class=move || {
                                                if dm() { "px-3 py-2 text-left font-medium text-gray-300" } else { "px-3 py-2 text-left font-medium text-gray-600" }
                                            }>"Type"</th>
                                            <th class=move || {
                                                if dm() { "px-3 py-2 text-left font-medium text-gray-300" } else { "px-3 py-2 text-left font-medium text-gray-600" }
                                            }>"Nullable"</th>
                                            <th class=move || {
                                                if dm() { "px-3 py-2 text-left font-medium text-gray-300" } else { "px-3 py-2 text-left font-medium text-gray-600" }
                                            }>"Default"</th>
                                            <th class=move || {
                                                if dm() { "px-3 py-2 text-left font-medium text-gray-300" } else { "px-3 py-2 text-left font-medium text-gray-600" }
                                            }>"Key"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {def.columns.iter().enumerate().map(|(idx, col)| {
                                            view! {
                                                <tr class=move || {
                                                    if dm() { "border-t border-gray-700 hover:bg-gray-700" } else { "border-t hover:bg-blue-50" }
                                                }>
                                                    <td class="px-3 py-1.5 text-gray-500">{idx + 1}</td>
                                                    <td class=move || {
                                                        if dm() { "px-3 py-1.5 font-mono text-gray-200" } else { "px-3 py-1.5 font-mono text-gray-800" }
                                                    }>{col.name.clone()}</td>
                                                    <td class=move || {
                                                        if dm() { "px-3 py-1.5 text-gray-400" } else { "px-3 py-1.5 text-gray-600" }
                                                    }>{col.data_type.clone()}</td>
                                                    <td class="px-3 py-1.5">
                                                        {if col.not_null {
                                                            view! { <span class="text-red-500 font-semibold">"NO"</span> }.into_any()
                                                        } else {
                                                            view! { <span class="text-gray-400">"YES"</span> }.into_any()
                                                        }}
                                                    </td>
                                                    <td class="px-3 py-1.5 text-gray-500 text-xs">
                                                        {col.default_value.clone().unwrap_or_default()}
                                                    </td>
                                                    <td class="px-3 py-1.5">
                                                        {if col.is_primary_key {
                                                            view! { <span class="text-yellow-600 font-semibold">"PK"</span> }.into_any()
                                                        } else {
                                                            view! {}.into_any()
                                                        }}
                                                    </td>
                                                </tr>
                                            }
                                        }).collect_view()}
                                    </tbody>
                                </table>
                            </div>
                        </div>

                        {if !def.indexes.is_empty() {
                            view! {
                                <div class=move || {
                                    if dm() { "bg-gray-800 rounded-lg shadow" } else { "bg-white rounded-lg shadow" }
                                }>
                                    <div class=move || {
                                        if dm() { "p-3 border-b border-gray-700" } else { "p-3 border-b" }
                                    }>
                                        <h2 class=move || {
                                            if dm() { "font-semibold text-sm text-gray-200" } else { "font-semibold text-sm text-gray-700" }
                                        }>"Indexes"</h2>
                                    </div>
                                    <div class="p-3">
                                        {def.indexes.iter().map(|idx| {
                                            view! {
                                                <div class="flex items-center gap-2 py-1 text-sm">
                                                    <span class=move || {
                                                        if dm() { "text-gray-200 font-mono" } else { "text-gray-700 font-mono" }
                                                    }>{idx.name.clone()}</span>
                                                    {if idx.is_unique {
                                                        view! { <span class="text-xs text-blue-500 bg-blue-50 px-1 rounded">"UNIQUE"</span> }.into_any()
                                                    } else {
                                                        view! {}.into_any()
                                                    }}
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            view! {}.into_any()
                        }}

                        {if !def.ddl.is_empty() {
                            view! {
                                <div class=move || {
                                    if dm() { "bg-gray-800 rounded-lg shadow" } else { "bg-white rounded-lg shadow" }
                                }>
                                    <div class=move || {
                                        if dm() { "p-3 border-b border-gray-700" } else { "p-3 border-b" }
                                    }>
                                        <h2 class=move || {
                                            if dm() { "font-semibold text-sm text-gray-200" } else { "font-semibold text-sm text-gray-700" }
                                        }>"DDL"</h2>
                                    </div>
                                    <div class="p-3">
                                        <pre class=move || {
                                            if dm() {
                                                "text-xs font-mono text-gray-300 bg-gray-700 p-3 rounded overflow-x-auto whitespace-pre-wrap"
                                            } else {
                                                "text-xs font-mono text-gray-700 bg-gray-50 p-3 rounded overflow-x-auto whitespace-pre-wrap"
                                            }
                                        }>
                                            {def.ddl.clone()}
                                        </pre>
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            view! {}.into_any()
                        }}
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }}
        </div>
    }
}
