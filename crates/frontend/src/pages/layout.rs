use crate::state::use_app_state;
use leptos::prelude::*;

#[component]
pub fn LayoutPage() -> impl IntoView {
    let app_state = use_app_state();
    let dark_mode = app_state.dark_mode;
    let (sidebar_pos, set_sidebar_pos) = signal("left".to_string());
    let (show_status_bar, set_show_status_bar) = signal(true);
    let (show_tab_bar, set_show_tab_bar) = signal(true);

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
                    }>"Layout"</h1>
                </div>

                <div class="grid gap-6">
                    <div class=move || {
                        if dark_mode.get() {
                            "bg-gray-800 rounded-lg p-6"
                        } else {
                            "bg-gray-50 rounded-lg p-6"
                        }
                    }>
                        <h2 class=move || {
                            if dark_mode.get() {
                                "text-lg font-semibold text-gray-200 mb-4"
                            } else {
                                "text-lg font-semibold text-gray-800 mb-4"
                            }
                        }>"Sidebar Position"</h2>
                        <div class="grid grid-cols-2 gap-4">
                            <button
                                class=move || {
                                    let active = sidebar_pos.get() == "left";
                                    if dark_mode.get() {
                                        if active {
                                            "px-4 py-3 bg-blue-600 text-white rounded text-center font-medium cursor-pointer transition-colors"
                                        } else {
                                            "px-4 py-3 bg-gray-700 text-gray-300 rounded text-center font-medium cursor-pointer hover:bg-gray-600 transition-colors"
                                        }
                                    } else {
                                        if active {
                                            "px-4 py-3 bg-blue-600 text-white rounded text-center font-medium cursor-pointer transition-colors"
                                        } else {
                                            "px-4 py-3 bg-gray-200 text-gray-700 rounded text-center font-medium cursor-pointer hover:bg-gray-300 transition-colors"
                                        }
                                    }
                                }
                                on:click=move |_| set_sidebar_pos.set("left".to_string())
                            >"Sidebar Left"</button>
                            <button
                                class=move || {
                                    let active = sidebar_pos.get() == "right";
                                    if dark_mode.get() {
                                        if active {
                                            "px-4 py-3 bg-blue-600 text-white rounded text-center font-medium cursor-pointer transition-colors"
                                        } else {
                                            "px-4 py-3 bg-gray-700 text-gray-300 rounded text-center font-medium cursor-pointer hover:bg-gray-600 transition-colors"
                                        }
                                    } else {
                                        if active {
                                            "px-4 py-3 bg-blue-600 text-white rounded text-center font-medium cursor-pointer transition-colors"
                                        } else {
                                            "px-4 py-3 bg-gray-200 text-gray-700 rounded text-center font-medium cursor-pointer hover:bg-gray-300 transition-colors"
                                        }
                                    }
                                }
                                on:click=move |_| set_sidebar_pos.set("right".to_string())
                            >"Sidebar Right"</button>
                        </div>
                    </div>

                    <div class=move || {
                        if dark_mode.get() {
                            "bg-gray-800 rounded-lg p-6"
                        } else {
                            "bg-gray-50 rounded-lg p-6"
                        }
                    }>
                        <h2 class=move || {
                            if dark_mode.get() {
                                "text-lg font-semibold text-gray-200 mb-4"
                            } else {
                                "text-lg font-semibold text-gray-800 mb-4"
                            }
                        }>"Visibility"</h2>
                        <div class="space-y-3">
                            <div class=move || {
                                if dark_mode.get() {
                                    "flex items-center justify-between px-4 py-3 bg-gray-700 rounded"
                                } else {
                                    "flex items-center justify-between px-4 py-3 bg-white rounded border"
                                }
                            }>
                                <span class=move || {
                                    if dark_mode.get() { "text-gray-300" } else { "text-gray-700" }
                                }>"Show Status Bar"</span>
                                <button
                                    class=move || {
                                        if show_status_bar.get() {
                                            "relative w-12 h-6 bg-blue-600 rounded-full cursor-pointer transition-colors"
                                        } else {
                                            if dark_mode.get() {
                                                "relative w-12 h-6 bg-gray-600 rounded-full cursor-pointer transition-colors"
                                            } else {
                                                "relative w-12 h-6 bg-gray-300 rounded-full cursor-pointer transition-colors"
                                            }
                                        }
                                    }
                                    on:click=move |_| set_show_status_bar.update(|v| *v = !*v)
                                >
                                    <div class=move || {
                                        if show_status_bar.get() {
                                            "absolute right-1 top-1 w-4 h-4 bg-white rounded-full transition-all"
                                        } else {
                                            "absolute left-1 top-1 w-4 h-4 bg-white rounded-full transition-all"
                                        }
                                    }></div>
                                </button>
                            </div>
                            <div class=move || {
                                if dark_mode.get() {
                                    "flex items-center justify-between px-4 py-3 bg-gray-700 rounded"
                                } else {
                                    "flex items-center justify-between px-4 py-3 bg-white rounded border"
                                }
                            }>
                                <span class=move || {
                                    if dark_mode.get() { "text-gray-300" } else { "text-gray-700" }
                                }>"Show Tab Bar"</span>
                                <button
                                    class=move || {
                                        if show_tab_bar.get() {
                                            "relative w-12 h-6 bg-blue-600 rounded-full cursor-pointer transition-colors"
                                        } else {
                                            if dark_mode.get() {
                                                "relative w-12 h-6 bg-gray-600 rounded-full cursor-pointer transition-colors"
                                            } else {
                                                "relative w-12 h-6 bg-gray-300 rounded-full cursor-pointer transition-colors"
                                            }
                                        }
                                    }
                                    on:click=move |_| set_show_tab_bar.update(|v| *v = !*v)
                                >
                                    <div class=move || {
                                        if show_tab_bar.get() {
                                            "absolute right-1 top-1 w-4 h-4 bg-white rounded-full transition-all"
                                        } else {
                                            "absolute left-1 top-1 w-4 h-4 bg-white rounded-full transition-all"
                                        }
                                    }></div>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
