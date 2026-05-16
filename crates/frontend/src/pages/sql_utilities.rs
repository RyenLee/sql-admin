use crate::state::use_app_state;
use crate::tab_manager::{Tab, TabKind, make_query_tab_id};
use leptos::prelude::*;

#[component]
pub fn SqlUtilitiesPage() -> impl IntoView {
    let app_state = use_app_state();
    let dark_mode = app_state.dark_mode;
    let (status_msg, set_status_msg) = signal(None::<String>);

    view! {
        <div class=move || {
            if dark_mode.get() {
                "min-h-full bg-gray-900 p-6"
            } else {
                "min-h-full bg-white p-6"
            }
        }>
            <div class="max-w-4xl mx-auto">
                <div class="flex items-center mb-6">
                    <h1 class=move || {
                        if dark_mode.get() {
                            "text-2xl font-bold text-white"
                        } else {
                            "text-2xl font-bold text-gray-900"
                        }
                    }>"SQL Utilities"</h1>
                </div>

                {move || {
                    let msg = status_msg.get();
                    msg.map(|m| view! {
                        <div class=move || {
                            if dark_mode.get() {
                                "mb-4 px-4 py-3 bg-green-900 border border-green-700 text-green-300 rounded-lg"
                            } else {
                                "mb-4 px-4 py-3 bg-green-50 border border-green-200 text-green-700 rounded-lg"
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
                    <div class="space-y-3">
                        <button
                            class=move || {
                                if dark_mode.get() {
                                    "w-full flex items-center justify-between px-4 py-3 bg-gray-700 rounded hover:bg-gray-600 cursor-pointer transition-colors"
                                } else {
                                    "w-full flex items-center justify-between px-4 py-3 bg-white rounded border hover:bg-gray-50 cursor-pointer transition-colors"
                                }
                            }
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
                                    set_status_msg.set(Some("Opening query editor for SQL formatting...".to_string()));
                                }
                            }
                        >
                            <span class=move || {
                                if dark_mode.get() { "text-gray-300" } else { "text-gray-700" }
                            }>"Format SQL"</span>
                            <span class=move || {
                                if dark_mode.get() { "text-gray-500 text-sm" } else { "text-gray-400 text-sm" }
                            }>"Ctrl+Shift+F"</span>
                        </button>
                        <button
                            class=move || {
                                if dark_mode.get() {
                                    "w-full flex items-center justify-between px-4 py-3 bg-gray-700 rounded hover:bg-gray-600 cursor-pointer transition-colors"
                                } else {
                                    "w-full flex items-center justify-between px-4 py-3 bg-white rounded border hover:bg-gray-50 cursor-pointer transition-colors"
                                }
                            }
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
                                    set_status_msg.set(Some("Opening query editor with EXPLAIN...".to_string()));
                                }
                            }
                        >
                            <span class=move || {
                                if dark_mode.get() { "text-gray-300" } else { "text-gray-700" }
                            }>"Explain Query"</span>
                            <span class=move || {
                                if dark_mode.get() { "text-gray-500 text-sm" } else { "text-gray-400 text-sm" }
                            }>"Ctrl+E"</span>
                        </button>
                        <button
                            class=move || {
                                if dark_mode.get() {
                                    "w-full flex items-center justify-between px-4 py-3 bg-gray-700 rounded hover:bg-gray-600 cursor-pointer transition-colors"
                                } else {
                                    "w-full flex items-center justify-between px-4 py-3 bg-white rounded border hover:bg-gray-50 cursor-pointer transition-colors"
                                }
                            }
                            on:click=move |_| {
                                set_status_msg.set(Some("Export results: No active query to export.".to_string()));
                            }
                        >
                            <span class=move || {
                                if dark_mode.get() { "text-gray-300" } else { "text-gray-700" }
                            }>"Export Results"</span>
                            <span class=move || {
                                if dark_mode.get() { "text-gray-500 text-sm" } else { "text-gray-400 text-sm" }
                            }>"Ctrl+S"</span>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
