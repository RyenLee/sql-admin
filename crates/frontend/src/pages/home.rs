use crate::state::AppState;
use crate::tab_manager::{Tab, TabKind, make_connections_tab_id, make_query_tab_id};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    let app_state = use_context::<AppState>();
    let dark_mode = app_state
        .as_ref()
        .map(|s| s.dark_mode)
        .unwrap_or(RwSignal::new(false));
    let tab_manager = app_state.as_ref().map(|s| s.tab_manager.clone());

    let open_connections_tab = {
        let tab_manager = tab_manager.clone();
        let app_state_clone = app_state.clone();
        move || {
            if let (Some(tm), Some(state)) = (tab_manager.clone(), app_state_clone.clone()) {
                let tab = Tab {
                    id: make_connections_tab_id(),
                    kind: TabKind::Connections,
                    title: "Connections".to_string(),
                };
                tm.update(|tm| {
                    tm.ensure_tab(tab.clone());
                });
                state.pending_navigation.set(Some(tab.kind.route()));
            }
        }
    };

    let open_query_tab = {
        let tab_manager = tab_manager.clone();
        let app_state_clone = app_state.clone();
        move || {
            if let (Some(tm), Some(state)) = (tab_manager.clone(), app_state_clone.clone()) {
                let tab = Tab {
                    id: make_query_tab_id(String::new()),
                    kind: TabKind::Query {
                        connection_id: String::new(),
                    },
                    title: "SQL Query".to_string(),
                };
                tm.update(|tm| {
                    tm.ensure_tab(tab.clone());
                });
                state.pending_navigation.set(Some(tab.kind.route()));
            }
        }
    };

    view! {
        <div class="text-center">
            <h1 class=move || {
                if dark_mode.get() { "text-4xl font-bold text-gray-100 mb-4" } else { "text-4xl font-bold text-gray-800 mb-4" }
            }>
                <a 
                    href="https://github.com" 
                    target="_blank" 
                    rel="noopener noreferrer"
                    class="hover:text-blue-500 transition-colors"
                >"Welcome to LiteAdmin"</a>
            </h1>
            <p class=move || {
                if dark_mode.get() { "text-gray-400 mb-8" } else { "text-gray-600 mb-8" }
            }>"A modern web-based SQL management tool built with Rust"</p>

            <div class="grid grid-cols-3 gap-6 max-w-4xl mx-auto">
                <div class=move || {
                    if dark_mode.get() { "bg-gray-800 rounded-lg shadow p-6" } else { "bg-white rounded-lg shadow p-6" }
                }>
                    <div class="text-4xl mb-4">"🔗"</div>
                    <h3 class=move || {
                        if dark_mode.get() { "text-xl font-semibold mb-2 text-gray-100" } else { "text-xl font-semibold mb-2" }
                    }>"Manage Connections"</h3>
                    <p class=move || {
                        if dark_mode.get() { "text-gray-400 text-sm" } else { "text-gray-500 text-sm" }
                    }>"Create and manage database connections"</p>
                    <div
                        class="inline-block mt-4 text-blue-600 hover:text-blue-800 cursor-pointer"
                        on:click={
                            let open_connections_tab = open_connections_tab.clone();
                            move |_| open_connections_tab()
                        }
                    >
                        "Get Started"
                    </div>
                </div>

                <div class=move || {
                    if dark_mode.get() { "bg-gray-800 rounded-lg shadow p-6" } else { "bg-white rounded-lg shadow p-6" }
                }>
                    <div class="text-4xl mb-4">"📝"</div>
                    <h3 class=move || {
                        if dark_mode.get() { "text-xl font-semibold mb-2 text-gray-100" } else { "text-xl font-semibold mb-2" }
                    }>"Execute Queries"</h3>
                    <p class=move || {
                        if dark_mode.get() { "text-gray-400 text-sm" } else { "text-gray-500 text-sm" }
                    }>"Run SQL queries and view results"</p>
                    <div
                        class="inline-block mt-4 text-blue-600 hover:text-blue-800 cursor-pointer"
                        on:click={
                            let open_query_tab = open_query_tab.clone();
                            move |_| open_query_tab()
                        }
                    >
                        "Query Now"
                    </div>
                </div>

                <div class=move || {
                    if dark_mode.get() { "bg-gray-800 rounded-lg shadow p-6" } else { "bg-white rounded-lg shadow p-6" }
                }>
                    <div class="text-4xl mb-4">"⚡"</div>
                    <h3 class=move || {
                        if dark_mode.get() { "text-xl font-semibold mb-2 text-gray-100" } else { "text-xl font-semibold mb-2" }
                    }>"Built with Rust"</h3>
                    <p class=move || {
                        if dark_mode.get() { "text-gray-400 text-sm" } else { "text-gray-500 text-sm" }
                    }>"Fast, safe, and reliable"</p>
                </div>
            </div>
        </div>
    }
}
