use leptos::prelude::*;

#[component]
pub fn KeyboardShortcutsPage() -> impl IntoView {
    view! {
        <div class="min-h-full bg-white dark:bg-gray-900 p-6">
            <div class="max-w-4xl mx-auto">
                <div class="flex items-center mb-6">
                    <h1 class="text-2xl font-bold text-gray-900 dark:text-white">"Keyboard Shortcuts"</h1>
                </div>

                <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-6">
                    <div class="grid grid-cols-2 gap-3">
                        <div class="flex items-center justify-between px-4 py-2 bg-white dark:bg-gray-700 rounded border dark:border-0">
                            <span class="text-gray-700 dark:text-gray-300">"New Query"</span>
                            <kbd class="px-2 py-1 bg-gray-200 dark:bg-gray-600 text-gray-600 dark:text-gray-300 text-sm rounded">"Ctrl+N"</kbd>
                        </div>
                        <div class="flex items-center justify-between px-4 py-2 bg-white dark:bg-gray-700 rounded border dark:border-0">
                            <span class="text-gray-700 dark:text-gray-300">"Run Query"</span>
                            <kbd class="px-2 py-1 bg-gray-200 dark:bg-gray-600 text-gray-600 dark:text-gray-300 text-sm rounded">"Ctrl+Enter"</kbd>
                        </div>
                        <div class="flex items-center justify-between px-4 py-2 bg-white dark:bg-gray-700 rounded border dark:border-0">
                            <span class="text-gray-700 dark:text-gray-300">"Format SQL"</span>
                            <kbd class="px-2 py-1 bg-gray-200 dark:bg-gray-600 text-gray-600 dark:text-gray-300 text-sm rounded">"Ctrl+Shift+F"</kbd>
                        </div>
                        <div class="flex items-center justify-between px-4 py-2 bg-white dark:bg-gray-700 rounded border dark:border-0">
                            <span class="text-gray-700 dark:text-gray-300">"New Connection"</span>
                            <kbd class="px-2 py-1 bg-gray-200 dark:bg-gray-600 text-gray-600 dark:text-gray-300 text-sm rounded">"Ctrl+Shift+N"</kbd>
                        </div>
                        <div class="flex items-center justify-between px-4 py-2 bg-white dark:bg-gray-700 rounded border dark:border-0">
                            <span class="text-gray-700 dark:text-gray-300">"Close Tab"</span>
                            <kbd class="px-2 py-1 bg-gray-200 dark:bg-gray-600 text-gray-600 dark:text-gray-300 text-sm rounded">"Ctrl+W"</kbd>
                        </div>
                        <div class="flex items-center justify-between px-4 py-2 bg-white dark:bg-gray-700 rounded border dark:border-0">
                            <span class="text-gray-700 dark:text-gray-300">"Toggle Sidebar"</span>
                            <kbd class="px-2 py-1 bg-gray-200 dark:bg-gray-600 text-gray-600 dark:text-gray-300 text-sm rounded">"Ctrl+B"</kbd>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
