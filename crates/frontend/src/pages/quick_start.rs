use leptos::prelude::*;

#[component]
pub fn QuickStartPage() -> impl IntoView {
    view! {
        <div class="min-h-full bg-white dark:bg-gray-900 p-6">
            <div class="max-w-4xl mx-auto">
                <div class="flex items-center mb-6">
                    <h1 class="text-2xl font-bold text-gray-900 dark:text-white">"Quick Start"</h1>
                </div>

                <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-6">
                    <div class="space-y-4">
                        <div class="bg-white dark:bg-gray-700 rounded-lg p-4 border dark:border-0">
                            <div class="flex items-start">
                                <span class="text-blue-500 font-bold mr-3">"1."</span>
                                <div>
                                    <p class="text-gray-700 dark:text-gray-300 font-medium mb-1">"Connect to a Database"</p>
                                    <p class="text-gray-500 dark:text-gray-400 text-sm">"Click File > New Connection to add a new database connection."</p>
                                </div>
                            </div>
                        </div>
                        <div class="bg-white dark:bg-gray-700 rounded-lg p-4 border dark:border-0">
                            <div class="flex items-start">
                                <span class="text-blue-500 font-bold mr-3">"2."</span>
                                <div>
                                    <p class="text-gray-700 dark:text-gray-300 font-medium mb-1">"Create a Query"</p>
                                    <p class="text-gray-500 dark:text-gray-400 text-sm">"Click File > New Query to open a new SQL editor tab."</p>
                                </div>
                            </div>
                        </div>
                        <div class="bg-white dark:bg-gray-700 rounded-lg p-4 border dark:border-0">
                            <div class="flex items-start">
                                <span class="text-blue-500 font-bold mr-3">"3."</span>
                                <div>
                                    <p class="text-gray-700 dark:text-gray-300 font-medium mb-1">"Execute SQL"</p>
                                    <p class="text-gray-500 dark:text-gray-400 text-sm">"Write your SQL query and click the Run button or press Ctrl+Enter."</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
