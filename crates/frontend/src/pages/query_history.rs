use crate::api::client;
use crate::state::use_app_state;
use crate::tab_manager::{Tab, TabKind, make_query_tab_id};
use leptos::prelude::*;
use sql_admin_shared::QueryHistory;
use wasm_bindgen_futures::spawn_local;

fn format_time_ago(created_at: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let diff = now.signed_duration_since(created_at);
    if diff.num_seconds() < 60 {
        "just now".to_string()
    } else if diff.num_minutes() < 60 {
        format!("{} min ago", diff.num_minutes())
    } else if diff.num_hours() < 24 {
        format!("{} hr ago", diff.num_hours())
    } else if diff.num_days() < 30 {
        format!("{} days ago", diff.num_days())
    } else {
        created_at.format("%Y-%m-%d %H:%M").to_string()
    }
}

#[component]
pub fn QueryHistoryPage() -> impl IntoView {
    let app_state = use_app_state();
    let dark_mode = app_state.dark_mode;
    let (history, set_history) = signal(Vec::<QueryHistory>::new());
    let (loading, set_loading) = signal(false);
    let (error_msg, set_error_msg) = signal(None::<String>);
    let (status_msg, set_status_msg) = signal(None::<String>);

    let load_history = {
        move || {
            set_loading.set(true);
            set_error_msg.set(None);
            spawn_local(async move {
                match client::get_query_history().await {
                    Ok(items) => {
                        set_history.set(items);
                    }
                    Err(e) => {
                        set_error_msg.set(Some(e));
                    }
                }
                set_loading.set(false);
            });
        }
    };

    Effect::new(move |_| {
        load_history();
    });

    view! {
        <div class=move || {
            if dark_mode.get() {
                "min-h-full bg-gray-900 p-6"
            } else {
                "min-h-full bg-white p-6"
            }
        }>
            <div class="max-w-4xl mx-auto">
                <div class="flex items-center justify-between mb-6">
                    <h1 class=move || {
                        if dark_mode.get() {
                            "text-2xl font-bold text-white"
                        } else {
                            "text-2xl font-bold text-gray-900"
                        }
                    }>"Query History"</h1>
                    <button
                        class=move || {
                            if dark_mode.get() {
                                "px-3 py-1.5 bg-gray-700 text-gray-300 rounded hover:bg-gray-600 cursor-pointer transition-colors text-sm"
                            } else {
                                "px-3 py-1.5 bg-gray-200 text-gray-700 rounded hover:bg-gray-300 cursor-pointer transition-colors text-sm"
                            }
                        }
                        on:click=move |_| load_history()
                        disabled=move || loading.get()
                    >
                        {move || if loading.get() { "Loading..." } else { "Refresh" }}
                    </button>
                </div>

                {move || {
                    let err = error_msg.get();
                    err.map(|e| view! {
                        <div class="mb-4 px-4 py-3 bg-red-50 border border-red-200 text-red-700 rounded-lg">
                            {format!("Error loading history: {}", e)}
                        </div>
                    })
                }}

                {move || {
                    let msg = status_msg.get();
                    msg.map(|m| view! {
                        <div class=move || {
                            if dark_mode.get() {
                                "mb-4 px-4 py-3 bg-blue-900 border border-blue-700 text-blue-300 rounded-lg"
                            } else {
                                "mb-4 px-4 py-3 bg-blue-50 border border-blue-200 text-blue-700 rounded-lg"
                            }
                        }>
                            {m}
                        </div>
                    })
                }}

                <div class=move || {
                    if dark_mode.get() {
                        "bg-gray-800 rounded-lg p-6"
                    } else {
                        "bg-gray-50 rounded-lg p-6"
                    }
                }>
                    {move || {
                        if loading.get() && history.get().is_empty() {
                            view! {
                                <div class=move || {
                                    if dark_mode.get() {
                                        "text-center py-8 text-gray-500"
                                    } else {
                                        "text-center py-8 text-gray-400"
                                    }
                                }>"Loading query history..."</div>
                            }.into_any()
                        } else {
                            let items = history.get();
                            if items.is_empty() {
                                view! {
                                    <div class=move || {
                                        if dark_mode.get() {
                                            "text-center py-8 text-gray-500"
                                        } else {
                                            "text-center py-8 text-gray-400"
                                        }
                                    }>"No query history yet. Execute a query to see it here."</div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-3">
                                        {items.into_iter().map(|item| {
                                            let item_id = item.id.clone();
                                            let query_text = item.query_text.clone();
                                            let connection_name = item.connection_name.clone();
                                            let exec_time = item.execution_time_ms.map(|t| format!("{}ms", t)).unwrap_or_default();
                                            let rows_info = item.rows_count.map(|r| format!("{} rows", r)).unwrap_or_default();
                                            let is_success = item.success;
                                            let time_ago = format_time_ago(item.created_at);
                                            let error_message = item.error_message.clone().unwrap_or_default();

                                            view! {
                                                <div class=move || {
                                                    if dark_mode.get() {
                                                        "flex items-center justify-between px-4 py-3 bg-gray-700 rounded"
                                                    } else {
                                                        "flex items-center justify-between px-4 py-3 bg-white rounded border"
                                                    }
                                                }>
                                                    <div class="flex-1 min-w-0 mr-4">
                                                        <div class="flex items-center gap-2 mb-1">
                                                            {if is_success {
                                                                view! {
                                                                    <span class="text-xs px-1.5 py-0.5 bg-green-100 text-green-700 rounded">"OK"</span>
                                                                }.into_any()
                                                            } else {
                                                                view! {
                                                                    <span class="text-xs px-1.5 py-0.5 bg-red-100 text-red-700 rounded">"Error"</span>
                                                                }.into_any()
                                                            }}
                                                            <span class=move || {
                                                                if dark_mode.get() {
                                                                    "text-xs text-gray-500"
                                                                } else {
                                                                    "text-xs text-gray-400"
                                                                }
                                                            }>{connection_name}</span>
                                                            <span class=move || {
                                                                if dark_mode.get() {
                                                                    "text-xs text-gray-500"
                                                                } else {
                                                                    "text-xs text-gray-400"
                                                                }
                                                            }>{time_ago}</span>
                                                        </div>
                                                        <span class=move || {
                                                            if dark_mode.get() {
                                                                "text-gray-300 text-sm"
                                                            } else {
                                                                "text-gray-700 text-sm"
                                                            }
                                                        }>{query_text.clone()}</span>
                                                        <div class="flex items-center gap-3 mt-1">
                                                            {if !exec_time.is_empty() {
                                                                view! {
                                                                    <span class=move || {
                                                                        if dark_mode.get() {
                                                                            "text-xs text-gray-500"
                                                                        } else {
                                                                            "text-xs text-gray-400"
                                                                        }
                                                                    }>{exec_time}</span>
                                                                }.into_any()
                                                            } else {
                                                                view! {}.into_any()
                                                            }}
                                                            {if !rows_info.is_empty() {
                                                                view! {
                                                                    <span class=move || {
                                                                        if dark_mode.get() {
                                                                            "text-xs text-gray-500"
                                                                        } else {
                                                                            "text-xs text-gray-400"
                                                                        }
                                                                    }>{rows_info}</span>
                                                                }.into_any()
                                                            } else {
                                                                view! {}.into_any()
                                                            }}
                                                            {if !error_message.is_empty() {
                                                                view! {
                                                                    <span class="text-xs text-red-500 truncate">{error_message}</span>
                                                                }.into_any()
                                                            } else {
                                                                view! {}.into_any()
                                                            }}
                                                        </div>
                                                    </div>
                                                    <div class="flex space-x-2 flex-shrink-0">
                                                        <button
                                                            class="px-3 py-1 bg-blue-600 text-white text-sm rounded hover:bg-blue-700 cursor-pointer transition-colors"
                                                            on:click={
                                                                let app_state = app_state.clone();
                                                                move |_| {
                                                                    let id = make_query_tab_id(String::new());
                                                                    app_state.tab_manager.update(|tm| {
                                                                        let tab = Tab {
                                                                            id: id.clone(),
                                                                            kind: TabKind::Query { connection_id: String::new() },
                                                                            title: "SQL Query".to_string(),
                                                                        };
                                                                        tm.ensure_tab(tab);
                                                                    });
                                                                    app_state.pending_navigation.set(Some("/query".to_string()));
                                                                }
                                                            }
                                                        >"Load"</button>
                                                        <button
                                                            class="px-3 py-1 bg-red-600 text-white text-sm rounded hover:bg-red-700 cursor-pointer transition-colors"
                                                            on:click={
                                                                let item_id = item_id.clone();
                                                                move |_| {
                                                                    let item_id_for_api = item_id.clone();
                                                                    let item_id_for_retain = item_id.clone();
                                                                    spawn_local(async move {
                                                                        match client::delete_query_history_item(item_id_for_api).await {
                                                                            Ok(_) => {
                                                                                set_history.update(|h| h.retain(|i| i.id != item_id_for_retain));
                                                                                set_status_msg.set(Some("History item deleted.".to_string()));
                                                                            }
                                                                            Err(e) => {
                                                                                set_status_msg.set(Some(format!("Failed to delete: {}", e)));
                                                                            }
                                                                        }
                                                                    });
                                                                }
                                                            }
                                                        >"Delete"</button>
                                                    </div>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            }
                        }
                    }}

                    {move || {
                        let items = history.get();
                        if !items.is_empty() {
                            view! {
                                <button
                                    class=move || {
                                        if dark_mode.get() {
                                            "mt-4 w-full px-4 py-2 bg-gray-700 text-gray-300 rounded hover:bg-gray-600 cursor-pointer transition-colors"
                                        } else {
                                            "mt-4 w-full px-4 py-2 bg-gray-200 text-gray-700 rounded hover:bg-gray-300 cursor-pointer transition-colors"
                                        }
                                    }
                                    on:click=move |_| {
                                        spawn_local(async move {
                                            match client::clear_query_history().await {
                                                Ok(_) => {
                                                    set_history.set(Vec::new());
                                                    set_status_msg.set(Some("All history cleared.".to_string()));
                                                }
                                                Err(e) => {
                                                    set_status_msg.set(Some(format!("Failed to clear: {}", e)));
                                                }
                                            }
                                        });
                                    }
                                >"Clear All History"</button>
                            }.into_any()
                        } else {
                            view! {}.into_any()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}
