use crate::state::use_app_state;
use leptos::prelude::*;

#[component]
pub fn StatusBar() -> impl IntoView {
    let app_state = use_app_state();

    view! {
        <footer class=move || {
            if app_state.dark_mode.get() {
                "bg-gray-800 border-t border-gray-700 py-1 px-4 flex-shrink-0"
            } else {
                "bg-white border-t border-gray-200 py-1 px-4 flex-shrink-0"
            }
        }>
            <div class="flex justify-between items-center text-xs text-gray-500">
                <div class="flex items-center space-x-4">
                    <span>"LiteAdmin v0.2.0"</span>
                    <span class="text-gray-300">"|"</span>
                    <span>"Rust + Axum + Leptos + SQLx"</span>
                </div>
                <div class="flex items-center space-x-4">
                    {move || {
                        let status = app_state.status.get();
                        let mut parts = Vec::new();

                        if let Some(ref db) = status.connected_db {
                            let db_display = if let Some(ref db_type) = status.db_type {
                                format!("{} ({})", db, db_type)
                            } else {
                                db.clone()
                            };
                            parts.push(view! {
                                <span class="text-blue-600">{db_display}</span>
                            }.into_any());
                        }

                        if let Some(count) = status.row_count {
                            parts.push(view! {
                                <span>{format!("{} rows", count)}</span>
                            }.into_any());
                        }

                        if let Some(ref time) = status.exec_time {
                            parts.push(view! {
                                <span class="text-green-600">{time.clone()}</span>
                            }.into_any());
                        }

                        if let Some(ref size) = status.db_size {
                            parts.push(view! {
                                <span class="text-purple-600">{size.clone()}</span>
                            }.into_any());
                        }

                        parts.push(view! {
                            <span class="text-gray-300">"|"</span>
                        }.into_any());
                        parts.push(view! {
                            <span class="text-green-600">{status.status_text}</span>
                        }.into_any());

                        parts.into_iter().collect::<Vec<_>>()
                    }}
                </div>
            </div>
        </footer>
    }
}
