use crate::state::use_app_state;
use leptos::prelude::*;

#[component]
pub fn KeyboardShortcutsPage() -> impl IntoView {
    let app_state = use_app_state();
    let dark_mode = app_state.dark_mode;

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
                    }>"Keyboard Shortcuts"</h1>
                </div>

                <div class=move || {
                    if dark_mode.get() {
                        "bg-gray-800 rounded-lg p-6"
                    } else {
                        "bg-gray-50 rounded-lg p-6"
                    }
                }>
                    <div class="grid grid-cols-2 gap-3">
                        <div class=move || {
                            if dark_mode.get() {
                                "flex items-center justify-between px-4 py-2 bg-gray-700 rounded"
                            } else {
                                "flex items-center justify-between px-4 py-2 bg-white rounded border"
                            }
                        }>
                            <span class=move || {
                                if dark_mode.get() { "text-gray-300" } else { "text-gray-700" }
                            }>"New Query"</span>
                            <kbd class=move || {
                                if dark_mode.get() {
                                    "px-2 py-1 bg-gray-600 text-gray-300 text-sm rounded"
                                } else {
                                    "px-2 py-1 bg-gray-200 text-gray-600 text-sm rounded"
                                }
                            }>"Ctrl+N"</kbd>
                        </div>
                        <div class=move || {
                            if dark_mode.get() {
                                "flex items-center justify-between px-4 py-2 bg-gray-700 rounded"
                            } else {
                                "flex items-center justify-between px-4 py-2 bg-white rounded border"
                            }
                        }>
                            <span class=move || {
                                if dark_mode.get() { "text-gray-300" } else { "text-gray-700" }
                            }>"Run Query"</span>
                            <kbd class=move || {
                                if dark_mode.get() {
                                    "px-2 py-1 bg-gray-600 text-gray-300 text-sm rounded"
                                } else {
                                    "px-2 py-1 bg-gray-200 text-gray-600 text-sm rounded"
                                }
                            }>"Ctrl+Enter"</kbd>
                        </div>
                        <div class=move || {
                            if dark_mode.get() {
                                "flex items-center justify-between px-4 py-2 bg-gray-700 rounded"
                            } else {
                                "flex items-center justify-between px-4 py-2 bg-white rounded border"
                            }
                        }>
                            <span class=move || {
                                if dark_mode.get() { "text-gray-300" } else { "text-gray-700" }
                            }>"Format SQL"</span>
                            <kbd class=move || {
                                if dark_mode.get() {
                                    "px-2 py-1 bg-gray-600 text-gray-300 text-sm rounded"
                                } else {
                                    "px-2 py-1 bg-gray-200 text-gray-600 text-sm rounded"
                                }
                            }>"Ctrl+Shift+F"</kbd>
                        </div>
                        <div class=move || {
                            if dark_mode.get() {
                                "flex items-center justify-between px-4 py-2 bg-gray-700 rounded"
                            } else {
                                "flex items-center justify-between px-4 py-2 bg-white rounded border"
                            }
                        }>
                            <span class=move || {
                                if dark_mode.get() { "text-gray-300" } else { "text-gray-700" }
                            }>"New Connection"</span>
                            <kbd class=move || {
                                if dark_mode.get() {
                                    "px-2 py-1 bg-gray-600 text-gray-300 text-sm rounded"
                                } else {
                                    "px-2 py-1 bg-gray-200 text-gray-600 text-sm rounded"
                                }
                            }>"Ctrl+Shift+N"</kbd>
                        </div>
                        <div class=move || {
                            if dark_mode.get() {
                                "flex items-center justify-between px-4 py-2 bg-gray-700 rounded"
                            } else {
                                "flex items-center justify-between px-4 py-2 bg-white rounded border"
                            }
                        }>
                            <span class=move || {
                                if dark_mode.get() { "text-gray-300" } else { "text-gray-700" }
                            }>"Close Tab"</span>
                            <kbd class=move || {
                                if dark_mode.get() {
                                    "px-2 py-1 bg-gray-600 text-gray-300 text-sm rounded"
                                } else {
                                    "px-2 py-1 bg-gray-200 text-gray-600 text-sm rounded"
                                }
                            }>"Ctrl+W"</kbd>
                        </div>
                        <div class=move || {
                            if dark_mode.get() {
                                "flex items-center justify-between px-4 py-2 bg-gray-700 rounded"
                            } else {
                                "flex items-center justify-between px-4 py-2 bg-white rounded border"
                            }
                        }>
                            <span class=move || {
                                if dark_mode.get() { "text-gray-300" } else { "text-gray-700" }
                            }>"Toggle Sidebar"</span>
                            <kbd class=move || {
                                if dark_mode.get() {
                                    "px-2 py-1 bg-gray-600 text-gray-300 text-sm rounded"
                                } else {
                                    "px-2 py-1 bg-gray-200 text-gray-600 text-sm rounded"
                                }
                            }>"Ctrl+B"</kbd>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
