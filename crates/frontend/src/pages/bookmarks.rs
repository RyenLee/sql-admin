use crate::state::use_app_state;
use crate::tab_manager::{Tab, TabKind, make_query_tab_id};
use leptos::prelude::*;

#[derive(Clone, PartialEq)]
struct BookmarkItem {
    id: usize,
    name: String,
    sql: String,
}

#[component]
pub fn BookmarksPage() -> impl IntoView {
    let app_state = use_app_state();
    let (bookmarks, set_bookmarks) = signal(vec![
        BookmarkItem { id: 1, name: "Get Active Users".to_string(), sql: "SELECT * FROM users WHERE active = 1 ORDER BY last_login DESC".to_string() },
        BookmarkItem { id: 2, name: "Daily Stats".to_string(), sql: "SELECT date(created_at) as day, COUNT(*) as count FROM posts GROUP BY day".to_string() },
    ]);
    let (status_msg, set_status_msg) = signal(None::<String>);

    view! {
        <div class="min-h-full bg-white dark:bg-gray-900 p-6">
            <div class="max-w-4xl mx-auto">
                <div class="flex items-center mb-6">
                    <h1 class="text-2xl font-bold text-gray-900 dark:text-white">"Bookmarks"</h1>
                </div>

                {move || {
                    let msg = status_msg.get();
                    msg.map(|m| view! {
                        <div class="mb-4 px-4 py-3 bg-blue-50 dark:bg-blue-900 border border-blue-200 dark:border-blue-700 text-blue-700 dark:text-blue-300 rounded-lg">
                            {m}
                        </div>
                    })
                }}

                <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-6">
                    <div class="space-y-3">
                        {move || {
                            bookmarks.get().into_iter().map(|item| {
                                view! {
                                    <div class="px-4 py-3 bg-white dark:bg-gray-700 rounded border dark:border-0">
                                        <div class="flex items-center justify-between">
                                            <div class="flex-1 min-w-0 mr-4">
                                                <span class="text-gray-800 dark:text-gray-200 font-medium">{item.name.clone()}</span>
                                                <p class="text-gray-500 dark:text-gray-400 text-sm mt-1 truncate">{item.sql.clone()}</p>
                                            </div>
                                            <div class="flex space-x-2 flex-shrink-0">
                                                <button
                                                    class="px-3 py-1 bg-blue-600 text-white text-sm rounded hover:bg-blue-700 cursor-pointer transition-colors"
                                                    on:click={
                                                        let name = item.name.clone();
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
                                                            set_status_msg.set(Some(format!("Loaded bookmark: {}", name)));
                                                        }
                                                    }
                                                >"Load"</button>
                                                <button
                                                    class="px-3 py-1 bg-red-600 text-white text-sm rounded hover:bg-red-700 cursor-pointer transition-colors"
                                                    on:click={
                                                        let item_id = item.id;
                                                        move |_| {
                                                            set_bookmarks.update(|b| b.retain(|i| i.id != item_id));
                                                            set_status_msg.set(Some("Bookmark removed.".to_string()));
                                                        }
                                                    }
                                                >"Remove"</button>
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()
                        }}
                    </div>
                    {move || {
                        let items = bookmarks.get();
                        if items.is_empty() {
                            view! {
                                <div class="text-center py-8 text-gray-400 dark:text-gray-500">"No bookmarks yet."</div>
                            }.into_any()
                        } else {
                            let _: () = view! {};
                            ().into_any()
                        }
                    }}
                    <button
                        class="mt-4 w-full px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 cursor-pointer transition-colors"
                        on:click=move |_| {
                            let next_id = bookmarks.get().iter().map(|b| b.id).max().unwrap_or(0) + 1;
                            set_bookmarks.update(|b| {
                                b.push(BookmarkItem {
                                    id: next_id,
                                    name: format!("New Bookmark {}", next_id),
                                    sql: "SELECT 1".to_string(),
                                });
                            });
                            set_status_msg.set(Some("New bookmark added.".to_string()));
                        }
                    >"Add Bookmark"</button>
                </div>
            </div>
        </div>
    }
}
