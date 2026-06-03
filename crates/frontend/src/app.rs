use crate::api::client;
use crate::components::header::HeaderBar;
use crate::components::sidebar::DatabaseSidebar;
use crate::components::status_bar::StatusBar;
use crate::components::tabs::TabBar;
use crate::pages::{
    AboutPage, AppearancePage, BookmarksPage, Connections, DatabaseToolsPage, Home,
    KeyboardShortcutsPage, LayoutPage, NotFound, QueryEditor, QueryHistoryPage, QuickStartPage,
    RedbBrowserPage, SqlUtilitiesPage, TableStructure,
};
use crate::state::provide_app_state;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn App() -> impl IntoView {
    provide_app_state();
    let app_state = use_context::<crate::state::AppState>().expect("AppState not provided");
    let (sidebar_open, set_sidebar_open) = signal(true);

    let loaded = StoredValue::new(false);
    {
        let app_state = app_state.clone();
        Effect::new(move |_| {
            if !loaded.get_value() {
                loaded.set_value(true);
                spawn_local(async move {
                    if let Ok(conns) = client::list_connections().await {
                        app_state.connections.set(conns);
                    }
                });
            }
        });
    }

    view! {
        <Router>
            <div class=move || if app_state.dark_mode.get() {
                "min-h-screen bg-gray-100 dark:bg-gray-900 text-gray-900 dark:text-gray-100 flex flex-col h-screen overflow-hidden dark"
            } else {
                "min-h-screen bg-gray-100 dark:bg-gray-900 text-gray-900 dark:text-gray-100 flex flex-col h-screen overflow-hidden"
            }>
                <HeaderBar sidebar_open=sidebar_open set_sidebar_open=set_sidebar_open />

                <div class="flex flex-1 overflow-hidden">
                    {move || if sidebar_open.get() {
                        view! {
                            <div class="w-64 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 flex-shrink-0 overflow-y-auto">
                                <DatabaseSidebar />
                            </div>
                        }.into_any()
                    } else {
                        let _: () = view! {};
                        ().into_any()
                    }}

                    <main class="flex-1 flex flex-col overflow-hidden bg-gray-100 dark:bg-gray-900">
                        <TabBar />
                        <div class="flex-1 overflow-y-auto p-6">
                            <Routes fallback=|| view! { <NotFound /> }>
                                <Route path=path!("/") view=Home />
                                <Route path=path!("/connections") view=Connections />
                                <Route path=path!("/query") view=QueryEditor />
                                <Route path=path!("/redb/:conn_id") view=RedbBrowserPage />
                                <Route path=path!("/table/:conn_id/:table") view=TableStructure />
                                <Route path=path!("/query-history") view=QueryHistoryPage />
                                <Route path=path!("/bookmarks") view=BookmarksPage />
                                <Route path=path!("/appearance") view=AppearancePage />
                                <Route path=path!("/layout") view=LayoutPage />
                                <Route path=path!("/database-tools") view=DatabaseToolsPage />
                                <Route path=path!("/sql-utilities") view=SqlUtilitiesPage />
                                <Route path=path!("/quick-start") view=QuickStartPage />
                                <Route path=path!("/keyboard-shortcuts") view=KeyboardShortcutsPage />
                                <Route path=path!("/about") view=AboutPage />
                            </Routes>
                        </div>
                    </main>
                </div>

                <StatusBar />
            </div>
        </Router>
    }
}
