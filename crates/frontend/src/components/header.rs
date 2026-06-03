use crate::state::use_app_state;
use crate::tab_manager::{Tab, TabKind, make_connections_tab_id, make_query_tab_id};
use leptos::prelude::*;

fn open_tab(app_state: &crate::state::AppState, id: &str, kind: TabKind, title: &str) {
    let route = kind.route();
    app_state.tab_manager.update(|tm| {
        let tab = Tab {
            id: id.to_string(),
            kind,
            title: title.to_string(),
        };
        tm.ensure_tab(tab);
    });
    app_state.pending_navigation.set(Some(route));
}

#[component]
pub fn HeaderBar(
    sidebar_open: ReadSignal<bool>,
    set_sidebar_open: WriteSignal<bool>,
) -> impl IntoView {
    let _ = sidebar_open;
    let (active_menu, set_active_menu) = signal(None::<String>);
    let app_state = use_app_state();
    let dark_mode = app_state.dark_mode;

    let open_query_tab = {
        let app_state = app_state.clone();
        move || {
            open_tab(
                &app_state,
                &make_query_tab_id(String::new()),
                TabKind::Query {
                    connection_id: String::new(),
                },
                "SQL Query",
            );
        }
    };

    let open_connections_tab = {
        let app_state = app_state.clone();
        move || {
            open_tab(
                &app_state,
                &make_connections_tab_id(),
                TabKind::Connections,
                "Connections",
            );
        }
    };

    let open_query_history_tab = {
        let app_state = app_state.clone();
        move || {
            open_tab(
                &app_state,
                "query_history",
                TabKind::QueryHistory,
                "Query History",
            );
        }
    };

    let open_bookmarks_tab = {
        let app_state = app_state.clone();
        move || {
            open_tab(&app_state, "bookmarks", TabKind::Bookmarks, "Bookmarks");
        }
    };

    let open_appearance_tab = {
        let app_state = app_state.clone();
        move || {
            open_tab(&app_state, "appearance", TabKind::Appearance, "Appearance");
        }
    };

    let open_layout_tab = {
        let app_state = app_state.clone();
        move || {
            open_tab(&app_state, "layout", TabKind::Layout, "Layout");
        }
    };

    let open_database_tools_tab = {
        let app_state = app_state.clone();
        move || {
            open_tab(
                &app_state,
                "database_tools",
                TabKind::DatabaseTools,
                "Database Tools",
            );
        }
    };

    let open_sql_utilities_tab = {
        let app_state = app_state.clone();
        move || {
            open_tab(
                &app_state,
                "sql_utilities",
                TabKind::SqlUtilities,
                "SQL Utilities",
            );
        }
    };

    let open_quick_start_tab = {
        let app_state = app_state.clone();
        move || {
            open_tab(
                &app_state,
                "quick_start",
                TabKind::QuickStart,
                "Quick Start",
            );
        }
    };

    let open_keyboard_shortcuts_tab = {
        let app_state = app_state.clone();
        move || {
            open_tab(
                &app_state,
                "keyboard_shortcuts",
                TabKind::KeyboardShortcuts,
                "Keyboard Shortcuts",
            );
        }
    };

    let open_about_tab = {
        let app_state = app_state.clone();
        move || {
            open_tab(&app_state, "about", TabKind::About, "About LiteAdmin");
        }
    };

    let open_query_tab_btn = open_query_tab.clone();

    view! {
        <header class="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
            <div class="flex items-center h-12 px-4">
                <button
                    class="p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-700 mr-3"
                    on:click=move |_| set_sidebar_open.update(|v| *v = !*v)
                >
                    <svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
                    </svg>
                </button>

                <button
                    class="text-lg font-bold text-blue-600 mr-6 px-2 py-1 rounded hover:bg-gray-100 dark:hover:bg-gray-700 active:bg-gray-200 dark:active:bg-gray-600 cursor-pointer transition-colors"
                    on:click={
                        let app_state = app_state.clone();
                        move |_| {
                            app_state.tab_manager.update(|tm| {
                                tm.close_all_tabs_except_home();
                            });
                            app_state.pending_navigation.set(Some("/".to_string()));
                        }
                    }
                >
                    "LiteAdmin"
                </button>

                <nav class="flex items-center space-x-1 text-sm">
                    // File Menu
                    <div class="relative"
                        on:mouseenter=move |_| {
                            if active_menu.get().is_some() {
                                set_active_menu.set(Some("file".to_string()));
                            }
                        }
                        on:mouseleave=move |_| set_active_menu.set(None)
                    >
                        <button class="px-3 py-1 rounded hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300"
                        on:click=move |_| {
                            if active_menu.get() == Some("file".to_string()) {
                                set_active_menu.set(None);
                            } else {
                                set_active_menu.set(Some("file".to_string()));
                            }
                        }
                        >"File"</button>
                        {move || if active_menu.get() == Some("file".to_string()) {
                            view! {
                                <div class="absolute top-full left-0 bg-white dark:bg-gray-800 border dark:border-gray-700 rounded shadow-lg py-1 min-w-40 z-50">
                                    <div
                                        class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300 cursor-pointer"
                                        on:click={
                                            let open_connections_tab = open_connections_tab.clone();
                                            move |_| {
                                                set_active_menu.set(None);
                                                open_connections_tab();
                                            }
                                        }
                                    >
                                        "New Connection"
                                    </div>
                                    <div
                                        class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300 cursor-pointer"
                                        on:click={
                                            let open_query_tab = open_query_tab.clone();
                                            move |_| {
                                                set_active_menu.set(None);
                                                open_query_tab();
                                            }
                                        }
                                    >
                                        "New Query"
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            let _: () = view! {};
                            ().into_any()
                        }}
                    </div>

                    // Edit Menu
                    <div class="relative"
                        on:mouseenter=move |_| {
                            if active_menu.get().is_some() {
                                set_active_menu.set(Some("edit".to_string()));
                            }
                        }
                        on:mouseleave=move |_| set_active_menu.set(None)
                    >
                        <button class="px-3 py-1 rounded hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300"
                        on:click=move |_| {
                            if active_menu.get() == Some("edit".to_string()) {
                                set_active_menu.set(None);
                            } else {
                                set_active_menu.set(Some("edit".to_string()));
                            }
                        }
                        >"Edit"</button>
                        {move || if active_menu.get() == Some("edit".to_string()) {
                            view! {
                                <div class="absolute top-full left-0 bg-white dark:bg-gray-800 border dark:border-gray-700 rounded shadow-lg py-1 min-w-40 z-50">
                                    <div
                                        class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300 cursor-pointer"
                                        on:click={
                                            let open_query_history_tab = open_query_history_tab.clone();
                                            move |_| {
                                                set_active_menu.set(None);
                                                open_query_history_tab();
                                            }
                                        }
                                    >
                                        "Query History"
                                    </div>
                                    <div
                                        class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300 cursor-pointer"
                                        on:click={
                                            let open_bookmarks_tab = open_bookmarks_tab.clone();
                                            move |_| {
                                                set_active_menu.set(None);
                                                open_bookmarks_tab();
                                            }
                                        }
                                    >
                                        "Bookmarks"
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            let _: () = view! {};
                            ().into_any()
                        }}
                    </div>

                    // View Menu
                    <div class="relative"
                        on:mouseenter=move |_| {
                            if active_menu.get().is_some() {
                                set_active_menu.set(Some("view".to_string()));
                            }
                        }
                        on:mouseleave=move |_| set_active_menu.set(None)
                    >
                        <button class="px-3 py-1 rounded hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300"
                        on:click=move |_| {
                            if active_menu.get() == Some("view".to_string()) {
                                set_active_menu.set(None);
                            } else {
                                set_active_menu.set(Some("view".to_string()));
                            }
                        }
                        >"View"</button>
                        {move || if active_menu.get() == Some("view".to_string()) {
                            view! {
                                <div class="absolute top-full left-0 bg-white dark:bg-gray-800 border dark:border-gray-700 rounded shadow-lg py-1 min-w-40 z-50">
                                    <div
                                        class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300 cursor-pointer"
                                        on:click={
                                            let open_appearance_tab = open_appearance_tab.clone();
                                            move |_| {
                                                set_active_menu.set(None);
                                                open_appearance_tab();
                                            }
                                        }
                                    >
                                        "Appearance"
                                    </div>
                                    <div
                                        class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300 cursor-pointer"
                                        on:click={
                                            let open_layout_tab = open_layout_tab.clone();
                                            move |_| {
                                                set_active_menu.set(None);
                                                open_layout_tab();
                                            }
                                        }
                                    >
                                        "Layout"
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            let _: () = view! {};
                            ().into_any()
                        }}
                    </div>

                    // Tools Menu
                    <div class="relative"
                        on:mouseenter=move |_| {
                            if active_menu.get().is_some() {
                                set_active_menu.set(Some("tools".to_string()));
                            }
                        }
                        on:mouseleave=move |_| set_active_menu.set(None)
                    >
                        <button class="px-3 py-1 rounded hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300"
                        on:click=move |_| {
                            if active_menu.get() == Some("tools".to_string()) {
                                set_active_menu.set(None);
                            } else {
                                set_active_menu.set(Some("tools".to_string()));
                            }
                        }
                        >"Tools"</button>
                        {move || if active_menu.get() == Some("tools".to_string()) {
                            view! {
                                <div class="absolute top-full left-0 bg-white dark:bg-gray-800 border dark:border-gray-700 rounded shadow-lg py-1 min-w-40 z-50">
                                    <div
                                        class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300 cursor-pointer"
                                        on:click={
                                            let open_database_tools_tab = open_database_tools_tab.clone();
                                            move |_| {
                                                set_active_menu.set(None);
                                                open_database_tools_tab();
                                            }
                                        }
                                    >
                                        "Database Tools"
                                    </div>
                                    <div
                                        class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300 cursor-pointer"
                                        on:click={
                                            let open_sql_utilities_tab = open_sql_utilities_tab.clone();
                                            move |_| {
                                                set_active_menu.set(None);
                                                open_sql_utilities_tab();
                                            }
                                        }
                                    >
                                        "SQL Utilities"
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            let _: () = view! {};
                            ().into_any()
                        }}
                    </div>

                    // Help Menu
                    <div class="relative"
                        on:mouseenter=move |_| {
                            if active_menu.get().is_some() {
                                set_active_menu.set(Some("help".to_string()));
                            }
                        }
                        on:mouseleave=move |_| set_active_menu.set(None)
                    >
                        <button class="px-3 py-1 rounded hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300"
                        on:click=move |_| {
                            if active_menu.get() == Some("help".to_string()) {
                                set_active_menu.set(None);
                            } else {
                                set_active_menu.set(Some("help".to_string()));
                            }
                        }
                        >"Help"</button>
                        {move || if active_menu.get() == Some("help".to_string()) {
                            view! {
                                <div class="absolute top-full left-0 bg-white dark:bg-gray-800 border dark:border-gray-700 rounded shadow-lg py-1 min-w-40 z-50">
                                    <div
                                        class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300 cursor-pointer"
                                        on:click={
                                            let open_quick_start_tab = open_quick_start_tab.clone();
                                            move |_| {
                                                set_active_menu.set(None);
                                                open_quick_start_tab();
                                            }
                                        }
                                    >
                                        "Quick Start"
                                    </div>
                                    <div
                                        class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300 cursor-pointer"
                                        on:click={
                                            let open_keyboard_shortcuts_tab = open_keyboard_shortcuts_tab.clone();
                                            move |_| {
                                                set_active_menu.set(None);
                                                open_keyboard_shortcuts_tab();
                                            }
                                        }
                                    >
                                        "Keyboard Shortcuts"
                                    </div>
                                    <div
                                        class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300 cursor-pointer"
                                        on:click={
                                            let open_about_tab = open_about_tab.clone();
                                            move |_| {
                                                set_active_menu.set(None);
                                                open_about_tab();
                                            }
                                        }
                                    >
                                        "About LiteAdmin"
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            let _: () = view! {};
                            ().into_any()
                        }}
                    </div>
                </nav>

                <div class="flex-1"></div>

                <div class="flex items-center space-x-2">
                    <button
                        class="p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-600 dark:text-gray-300"
                        title="Refresh"
                        on:click=move |_| {
                            app_state.refresh_trigger.update(|v| *v += 1);
                        }
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
                        </svg>
                    </button>
                    <button
                        class="p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-600 dark:text-gray-300"
                        on:click=move |_| dark_mode.update(|v| *v = !*v)
                        title=move || if dark_mode.get() { "Switch to Light Mode" } else { "Switch to Dark Mode" }
                    >
                        {move || if dark_mode.get() {
                            view! {
                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
                                </svg>
                            }.into_any()
                        } else {
                            view! {
                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
                                </svg>
                            }.into_any()
                        }}
                    </button>
                    <button
                        class="px-3 py-1.5 bg-blue-600 text-white rounded text-sm hover:bg-blue-700"
                        on:click={
                            let open_query_tab_btn = open_query_tab_btn.clone();
                            move |_| open_query_tab_btn()
                        }
                    >
                        "New Query"
                    </button>
                </div>
            </div>
        </header>
    }
}
