use crate::components::header::HeaderBar;
use crate::components::sidebar::DatabaseSidebar;
use crate::components::status_bar::StatusBar;
use crate::components::tabs::TabBar;
use crate::pages::{
    AboutPage, AppearancePage, BookmarksPage, Connections, DatabaseToolsPage, Home,
    KeyboardShortcutsPage, LayoutPage, NotFound, QueryEditor, QueryHistoryPage, QuickStartPage,
    SqlUtilitiesPage, TableStructure,
};
use crate::state::provide_app_state;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

#[component]
pub fn App() -> impl IntoView {
    provide_app_state();
    let app_state = use_context::<crate::state::AppState>().expect("AppState not provided");
    let (sidebar_open, set_sidebar_open) = signal(true);

    view! {
        <Router>
            <div class=move || {
                if app_state.dark_mode.get() {
                    "min-h-screen bg-gray-900 text-gray-100 flex flex-col h-screen overflow-hidden dark"
                } else {
                    "min-h-screen bg-gray-100 text-gray-900 flex flex-col h-screen overflow-hidden"
                }
            }>
                <HeaderBar sidebar_open=sidebar_open set_sidebar_open=set_sidebar_open dark_mode=app_state.dark_mode />

                <div class="flex flex-1 overflow-hidden">
                    {move || if sidebar_open.get() {
                        view! {
                            <div class=move || {
                                if app_state.dark_mode.get() {
                                    "w-64 bg-gray-800 border-r border-gray-700 flex-shrink-0 overflow-y-auto"
                                } else {
                                    "w-64 bg-white border-r border-gray-200 flex-shrink-0 overflow-y-auto"
                                }
                            }>
                                <DatabaseSidebar />
                            </div>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }}

                    <main class=move || {
                        if app_state.dark_mode.get() {
                            "flex-1 flex flex-col overflow-hidden bg-gray-900"
                        } else {
                            "flex-1 flex flex-col overflow-hidden bg-gray-100"
                        }
                    }>
                        <TabBar />
                        <div class="flex-1 overflow-y-auto p-6">
                            <Routes fallback=|| view! { <NotFound /> }>
                                <Route path=path!("/") view=Home />
                                <Route path=path!("/connections") view=Connections />
                                <Route path=path!("/query") view=QueryEditor />
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
