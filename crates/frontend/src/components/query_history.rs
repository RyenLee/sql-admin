use crate::api::client;
use leptos::prelude::*;
use sql_admin_api_types::QueryHistory;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn QueryHistoryPanel(
    #[prop(optional)] on_select: Option<Callback<String, ()>>,
) -> impl IntoView {
    let (history, set_history) = signal(Vec::<QueryHistory>::new());
    let (_show_all, _set_show_all) = signal(true);
    let (loading, set_loading) = signal(false);

    let load_history = move |_| {
        set_loading.set(true);
        spawn_local(async move {
            if let Ok(items) = client::get_query_history().await {
                set_history.set(items);
            }
            set_loading.set(false);
        });
    };

    let clear_history = move |_| {
        spawn_local(async move {
            if client::clear_query_history().await.is_ok() {
                set_history.set(Vec::new());
            }
        });
    };

    Effect::new(move |_| {
        load_history(());
    });

    view! {
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow flex flex-col h-full">
            <div class="p-3 border-b dark:border-gray-700 flex items-center justify-between flex-shrink-0">
                <h3 class="font-semibold text-sm text-gray-700 dark:text-gray-200">"Query History"</h3>
                <div class="flex items-center gap-2">
                    <button
                        class="text-xs px-2 py-1 rounded hover:bg-gray-100 dark:hover:bg-gray-700"
                        on:click=move |_| {
                            load_history(());
                        }
                        disabled=loading
                    >
                        {move || if loading.get() { "..." } else { "↻" }}
                    </button>
                    <button
                        class="text-xs px-2 py-1 rounded text-red-500 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900"
                        on:click=clear_history
                    >
                        "Clear"
                    </button>
                </div>
            </div>

            <div class="overflow-y-auto flex-1 min-h-0">
                {move || {
                    let items = history.get();
                    if items.is_empty() {
                        view! {
                            <div class="p-4 text-center text-sm text-gray-400">
                                "No query history yet"
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="divide-y dark:divide-gray-700">
                                {items.into_iter().map(|item| {
                                    let query_text = item.query_text.clone();
                                    let conn_name = item.connection_name.clone();
                                    let exec_time = item.execution_time_ms.map(|t| format!("{}ms", t)).unwrap_or_default();
                                    let is_success = item.success;
                                    let created = item.created_at.format("%H:%M:%S").to_string();

                                    view! {
                                        <div
                                            class="p-2 hover:bg-gray-50 dark:hover:bg-gray-700 cursor-pointer"
                                            on:click=move |_| {
                                                if let Some(ref cb) = on_select {
                                                    cb.run(query_text.clone());
                                                }
                                            }
                                        >
                                            <div class="flex items-center justify-between">
                                                <span class="text-xs text-gray-500">{conn_name}</span>
                                                <span class="text-xs text-gray-400">{created}</span>
                                            </div>
                                            <div class="text-xs text-gray-700 dark:text-gray-300 truncate mt-0.5">{item.query_text.clone()}</div>
                                            <div class="flex items-center gap-2 mt-0.5">
                                                {if is_success {
                                                    view! {
                                                        <span class="text-xs text-green-600">"OK"</span>
                                                    }.into_any()
                                                } else {
                                                    view! {
                                                        <span class="text-xs text-red-500">"Error"</span>
                                                    }.into_any()
                                                }}
                                                {if !exec_time.is_empty() {
                                                    view! {
                                                        <span class="text-xs text-gray-400">{exec_time}</span>
                                                    }.into_any()
                                                } else {
                                                    let _: () = view! {};
                                                    ().into_any()
                                                }}
                                            </div>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
