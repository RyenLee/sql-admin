use crate::state::use_app_state;
use crate::tab_manager::make_home_tab_id;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn TabBar() -> impl IntoView {
    let app_state = use_app_state();
    let navigate = use_navigate();

    let activate_tab = {
        let app_state = app_state.clone();
        move |tab_id: String| {
            app_state.tab_manager.update(|tm| {
                tm.active_tab_id = Some(tab_id.clone());
            });
            if let Some(active) = app_state.tab_manager.get().active_tab() {
                app_state.pending_navigation.set(Some(active.kind.route()));
            }
        }
    };

    let remove_tab = {
        let app_state = app_state.clone();
        move |tab_id: String| {
            app_state.tab_manager.update(|tm| {
                tm.remove_tab(&tab_id);
            });
            if let Some(active) = app_state.tab_manager.get().active_tab() {
                app_state.pending_navigation.set(Some(active.kind.route()));
            } else {
                app_state.pending_navigation.set(Some("/".to_string()));
            }
        }
    };

    Effect::new(move |_| {
        let nav = app_state.pending_navigation.get();
        if let Some(route) = nav {
            app_state.pending_navigation.set(None);
            navigate(&route, Default::default());
        }
    });

    view! {
        <div class="border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-900">
            <div class="flex items-center">
                {move || {
                    let tabs = app_state.tab_manager.get().tabs.clone();
                    let active_id = app_state.tab_manager.get().active_tab_id.clone();

                    if tabs.is_empty() {
                        view! {
                            <div class="flex-1 flex items-center justify-center px-4 py-2">
                                <span class="text-sm text-gray-500">"No tabs open"</span>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="flex items-center overflow-x-auto">
                                {tabs.into_iter().map(|tab| {
                                    let is_active = active_id == Some(tab.id.clone());
                                    let tab_id_activate = tab.id.clone();
                                    let tab_id_remove = tab.id.clone();
                                    let tab_title = tab.title.clone();
                                    let is_home_tab = tab.id == make_home_tab_id();

                                    view! {
                                        <div
                                            class=move || {
                                                let base = if is_active {
                                                    "bg-white dark:bg-gray-800 border-b-2 border-blue-500 text-gray-900 dark:text-white"
                                                } else {
                                                    "bg-gray-50 dark:bg-gray-900 text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800"
                                                };
                                                format!("flex items-center px-4 py-2 cursor-pointer border-r border-gray-700/50 {}", base)
                                            }
                                            on:click=move |_| activate_tab(tab_id_activate.clone())
                                        >
                                            <span class="text-sm font-medium truncate max-w-[180px]">{tab_title}</span>
                                            {if !is_home_tab {
                                                view! {
                                                    <button
                                                        class="ml-2 p-0.5 rounded hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-500 dark:text-gray-400"
                                                        on:click=move |ev| {
                                                            ev.stop_propagation();
                                                            remove_tab(tab_id_remove.clone());
                                                        }
                                                    >
                                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                                                        </svg>
                                                    </button>
                                                }.into_any()
                                            } else {
                                                let _: () = view! {};
                                                ().into_any()
                                            }}
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
