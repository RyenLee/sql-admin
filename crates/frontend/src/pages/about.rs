use leptos::prelude::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="min-h-full bg-white dark:bg-gray-900 p-6">
            <div class="max-w-4xl mx-auto">
                <div class="flex items-center mb-6">
                    <h1 class="text-2xl font-bold text-gray-900 dark:text-white">"About LiteAdmin"</h1>
                </div>

                <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-6">
                    <div class="bg-white dark:bg-gray-700 rounded-lg p-6 border dark:border-0 mb-6">
                        <div class="flex items-center space-x-4">
                            <div class="w-12 h-12 bg-blue-600 rounded-lg flex items-center justify-center text-white text-xl font-bold">"L"</div>
                            <div>
                                <h3 class="text-gray-800 dark:text-gray-200 font-bold text-lg">"LiteAdmin"</h3>
                                <p class="text-gray-500 dark:text-gray-400 text-sm">"Version 1.1.0"</p>
                                <p class="text-gray-500 dark:text-gray-400 text-sm">"A lightweight database administration tool built entirely with Rust."</p>
                            </div>
                        </div>
                    </div>

                    <div class="bg-white dark:bg-gray-700 rounded-lg p-6 border dark:border-0 mb-6">
                        <h2 class="text-xl font-bold text-gray-800 dark:text-gray-100 mb-4 flex items-center">
                            <span class="mr-2">"🛠️"</span>
                            "Tech Stack"
                        </h2>

                        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                            <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-4">
                                <h4 class="text-gray-700 dark:text-gray-200 font-semibold mb-2">"Frontend"</h4>
                                <p class="text-gray-500 dark:text-gray-400 text-sm">"Leptos 0.8 (Rust WASM)"</p>
                            </div>
                            <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-4">
                                <h4 class="text-gray-700 dark:text-gray-200 font-semibold mb-2">"Backend"</h4>
                                <p class="text-gray-500 dark:text-gray-400 text-sm">"Axum (Rust), SQLx"</p>
                            </div>
                            <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-4">
                                <h4 class="text-gray-700 dark:text-gray-200 font-semibold mb-2">"Databases"</h4>
                                <p class="text-gray-500 dark:text-gray-400 text-sm">"PostgreSQL, MySQL, SQLite, Redb"</p>
                            </div>
                            <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-4">
                                <h4 class="text-gray-700 dark:text-gray-200 font-semibold mb-2">"Architecture"</h4>
                                <p class="text-gray-500 dark:text-gray-400 text-sm">"DDD (Domain-Driven Design)"</p>
                            </div>
                        </div>
                    </div>

                    <div class="bg-white dark:bg-gray-700 rounded-lg p-6 border dark:border-0 mb-6">
                        <h2 class="text-xl font-bold text-gray-800 dark:text-gray-100 mb-4 flex items-center">
                            <span class="mr-2">"📋"</span>
                            "Features"
                        </h2>

                        <div class="space-y-4">
                            <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-4">
                                <h4 class="text-gray-700 dark:text-gray-200 font-semibold mb-2">"🔗 Connection Management"</h4>
                                <p class="text-gray-500 dark:text-gray-400 text-sm">"Create, edit, delete database connections with encrypted password storage. Supports PostgreSQL, MySQL, SQLite, and Redb."</p>
                            </div>
                            <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-4">
                                <h4 class="text-gray-700 dark:text-gray-200 font-semibold mb-2">"📝 SQL Query Editor"</h4>
                                <p class="text-gray-500 dark:text-gray-400 text-sm">"Execute SQL queries with syntax highlighting, table-aware query generation, and result export."</p>
                            </div>
                            <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-4">
                                <h4 class="text-gray-700 dark:text-gray-200 font-semibold mb-2">"🔴 Redb Key-Value Browser"</h4>
                                <p class="text-gray-500 dark:text-gray-400 text-sm">"Browse Redb tables with automatic type detection, key prefix search, and pagination."</p>
                            </div>
                            <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-4">
                                <h4 class="text-gray-700 dark:text-gray-200 font-semibold mb-2">"📊 Table Structure Viewer"</h4>
                                <p class="text-gray-500 dark:text-gray-400 text-sm">"Browse table definitions, columns, indexes, and DDL statements."</p>
                            </div>
                            <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-4">
                                <h4 class="text-gray-700 dark:text-gray-200 font-semibold mb-2">"📜 Query History & Bookmarks"</h4>
                                <p class="text-gray-500 dark:text-gray-400 text-sm">"Track executed queries and save favorites for quick access."</p>
                            </div>
                            <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-4">
                                <h4 class="text-gray-700 dark:text-gray-200 font-semibold mb-2">"🔧 Database Tools"</h4>
                                <p class="text-gray-500 dark:text-gray-400 text-sm">"Data import/export, SQL formatting, and data comparison utilities."</p>
                            </div>
                        </div>
                    </div>

                    <div class="bg-white dark:bg-gray-700 rounded-lg p-6 border dark:border-0 mb-6">
                        <h2 class="text-xl font-bold text-gray-800 dark:text-gray-100 mb-4 flex items-center">
                            <span class="mr-2">"🏗️"</span>
                            "Architecture"
                        </h2>

                        <div class="space-y-4">
                            <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-4">
                                <h4 class="text-blue-600 dark:text-blue-400 font-semibold mb-2">"Frontend Layer"</h4>
                                <p class="text-gray-500 dark:text-gray-400 text-sm">"Leptos 0.8 CSR SPA compiled to WASM. Includes responsive UI components, SQL editor, data tables, and Explorer sidebar with database type grouping."</p>
                            </div>

                            <div class="text-center">
                                <span class="text-gray-400 dark:text-gray-500">"▼"</span>
                            </div>

                            <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-4">
                                <h4 class="text-green-600 dark:text-green-400 font-semibold mb-2">"Application Layer"</h4>
                                <p class="text-gray-500 dark:text-gray-400 text-sm">"Business use case orchestration: Connection, Query, History, DataEdit, Import, and Redb handlers."</p>
                            </div>

                            <div class="text-center">
                                <span class="text-gray-400 dark:text-gray-500">"▼"</span>
                            </div>

                            <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-4">
                                <h4 class="text-yellow-600 dark:text-yellow-400 font-semibold mb-2">"Domain Layer"</h4>
                                <p class="text-gray-500 dark:text-gray-400 text-sm">"Aggregates, value objects, repository traits, domain events, encryption and connection pool abstractions."</p>
                            </div>

                            <div class="text-center">
                                <span class="text-gray-400 dark:text-gray-500">"▼"</span>
                            </div>

                            <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-4">
                                <h4 class="text-purple-600 dark:text-purple-400 font-semibold mb-2">"Infrastructure Layer"</h4>
                                <p class="text-gray-500 dark:text-gray-400 text-sm">"SQLite persistence, AES-GCM encryption, in-memory event bus, connection pool cache, and Redb executor with automatic type detection."</p>
                            </div>
                        </div>
                    </div>

                    <div class="bg-white dark:bg-gray-700 rounded-lg p-6 border dark:border-0">
                        <h2 class="text-xl font-bold text-gray-800 dark:text-gray-100 mb-4 flex items-center">
                            <span class="mr-2">"📅"</span>
                            "Changelog"
                        </h2>

                        <div class="space-y-3">
                            <div class="flex items-start border-l-2 border-blue-500 pl-4">
                                <span class="text-gray-500 dark:text-gray-400 text-sm font-medium">"v1.1.0 (2026-06-03)"</span>
                                <ul class="text-gray-600 dark:text-gray-300 text-sm ml-4">
                                    <li>"🔴 Redb key-value database support"</li>
                                    <li>"🗂 Explorer sidebar with database type grouping"</li>
                                    <li>"📋 Table dropdown for auto-generating SELECT queries"</li>
                                    <li>"🏷 Database type badges in Connections page"</li>
                                    <li>"🔄 Hot reload development scripts"</li>
                                    <li>"🐛 Fixed SQLite null values for untyped columns"</li>
                                    <li>"🐛 Fixed Redb automatic type detection"</li>
                                    <li>"🐛 Improved error reporting"</li>
                                </ul>
                            </div>
                            <div class="flex items-start border-l-2 border-green-500 pl-4">
                                <span class="text-gray-500 dark:text-gray-400 text-sm font-medium">"v1.0.0 (2026-05-15)"</span>
                                <ul class="text-gray-600 dark:text-gray-300 text-sm ml-4">
                                    <li>"✨ Initial release"</li>
                                    <li>"🔗 PostgreSQL, MySQL, SQLite support"</li>
                                    <li>"📝 SQL query editor"</li>
                                    <li>"📊 Table structure viewer"</li>
                                    <li>"📜 Query history and bookmarks"</li>
                                    <li>"🎨 Dark mode support"</li>
                                    <li>"💾 Data export functionality"</li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
