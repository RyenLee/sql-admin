use crate::state::use_app_state;
use leptos::prelude::*;

#[component]
pub fn AppearancePage() -> impl IntoView {
    let app_state = use_app_state();
    let dark_mode = app_state.dark_mode;

    view! {
        <div class="min-h-full bg-white dark:bg-gray-900 p-6">
            <div class="max-w-4xl mx-auto">
                <div class="flex items-center mb-6">
                    <h1 class="text-2xl font-bold text-gray-900 dark:text-white">"Appearance"</h1>
                </div>

                <div class="grid gap-6">
                    <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-6">
                        <h2 class="text-lg font-semibold text-gray-800 dark:text-gray-200 mb-4">"Theme"</h2>
                        <div class="space-y-4">
                            <div class="flex items-center justify-between px-4 py-3 bg-white dark:bg-gray-700 rounded border dark:border-0">
                                <span class="text-gray-700 dark:text-gray-300">"Dark Mode"</span>
                                <button
                                    class=move || {
                                        if dark_mode.get() {
                                            "relative w-12 h-6 bg-blue-600 rounded-full"
                                        } else {
                                            "relative w-12 h-6 bg-gray-300 rounded-full"
                                        }
                                    }
                                    on:click=move |_| dark_mode.update(|v| *v = !*v)
                                >
                                    <div class=move || {
                                        if dark_mode.get() {
                                            "absolute right-1 top-1 w-4 h-4 bg-white rounded-full"
                                        } else {
                                            "absolute left-1 top-1 w-4 h-4 bg-white rounded-full"
                                        }
                                    }></div>
                                </button>
                            </div>
                        </div>
                    </div>

                    <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-6">
                        <h2 class="text-lg font-semibold text-gray-800 dark:text-gray-200 mb-4">"Font Size"</h2>
                        <div class="flex items-center space-x-4">
                            <span class="text-gray-500 dark:text-gray-400">"Small"</span>
                            <input type="range" min="12" max="18" value="14" class="flex-1" />
                            <span class="text-gray-500 dark:text-gray-400">"Large"</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
