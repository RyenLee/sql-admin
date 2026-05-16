use crate::state::use_app_state;
use leptos::prelude::*;

#[component]
pub fn AboutPage() -> impl IntoView {
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
                    }>"About LiteAdmin"</h1>
                </div>

                <div class=move || {
                    if dark_mode.get() {
                        "bg-gray-800 rounded-lg p-6"
                    } else {
                        "bg-gray-50 rounded-lg p-6"
                    }
                }>
                    <div class=move || {
                        if dark_mode.get() {
                            "bg-gray-700 rounded-lg p-6 mb-6"
                        } else {
                            "bg-white rounded-lg p-6 border mb-6"
                        }
                    }>
                        <div class="flex items-center space-x-4">
                            <div class="w-12 h-12 bg-blue-600 rounded-lg flex items-center justify-center text-white text-xl font-bold">"L"</div>
                            <div>
                                <h3 class=move || {
                                    if dark_mode.get() {
                                        "text-gray-200 font-bold text-lg"
                                    } else {
                                        "text-gray-800 font-bold text-lg"
                                    }
                                }>"LiteAdmin"</h3>
                                <p class=move || {
                                    if dark_mode.get() { "text-gray-400 text-sm" } else { "text-gray-500 text-sm" }
                                }>"Version 1.0.0"</p>
                                <p class=move || {
                                    if dark_mode.get() { "text-gray-400 text-sm" } else { "text-gray-500 text-sm" }
                                }>"A lightweight SQL database administration tool."</p>
                            </div>
                        </div>
                    </div>

                    <div class=move || {
                        if dark_mode.get() {
                            "bg-gray-700 rounded-lg p-6 mb-6"
                        } else {
                            "bg-white rounded-lg p-6 border mb-6"
                        }
                    }>
                        <h2 class=move || {
                            if dark_mode.get() {
                                "text-xl font-bold text-gray-100 mb-4 flex items-center"
                            } else {
                                "text-xl font-bold text-gray-800 mb-4 flex items-center"
                            }
                        }>
                            <span class="mr-2">"🛠️"</span>
                            "系统技术栈"
                        </h2>
                        
                        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                            <div class=move || {
                                if dark_mode.get() { "bg-gray-800 rounded-lg p-4" } else { "bg-gray-50 rounded-lg p-4" }
                            }>
                                <h4 class=move || {
                                    if dark_mode.get() { "text-gray-200 font-semibold mb-2" } else { "text-gray-700 font-semibold mb-2" }
                                }>"前端框架"</h4>
                                <p class=move || {
                                    if dark_mode.get() { "text-gray-400 text-sm" } else { "text-gray-500 text-sm" }
                                }>"Leptos (Rust WASM)"</p>
                            </div>
                            <div class=move || {
                                if dark_mode.get() { "bg-gray-800 rounded-lg p-4" } else { "bg-gray-50 rounded-lg p-4" }
                            }>
                                <h4 class=move || {
                                    if dark_mode.get() { "text-gray-200 font-semibold mb-2" } else { "text-gray-700 font-semibold mb-2" }
                                }>"后端技术"</h4>
                                <p class=move || {
                                    if dark_mode.get() { "text-gray-400 text-sm" } else { "text-gray-500 text-sm" }
                                }>"Axum (Rust)"</p>
                            </div>
                            <div class=move || {
                                if dark_mode.get() { "bg-gray-800 rounded-lg p-4" } else { "bg-gray-50 rounded-lg p-4" }
                            }>
                                <h4 class=move || {
                                    if dark_mode.get() { "text-gray-200 font-semibold mb-2" } else { "text-gray-700 font-semibold mb-2" }
                                }>"数据库支持"</h4>
                                <p class=move || {
                                    if dark_mode.get() { "text-gray-400 text-sm" } else { "text-gray-500 text-sm" }
                                }>"PostgreSQL, MySQL, SQLite"</p>
                            </div>
                            <div class=move || {
                                if dark_mode.get() { "bg-gray-800 rounded-lg p-4" } else { "bg-gray-50 rounded-lg p-4" }
                            }>
                                <h4 class=move || {
                                    if dark_mode.get() { "text-gray-200 font-semibold mb-2" } else { "text-gray-700 font-semibold mb-2" }
                                }>"开发工具"</h4>
                                <p class=move || {
                                    if dark_mode.get() { "text-gray-400 text-sm" } else { "text-gray-500 text-sm" }
                                }>"SQLx, TailwindCSS, Vite"</p>
                            </div>
                        </div>
                    </div>

                    <div class=move || {
                        if dark_mode.get() {
                            "bg-gray-700 rounded-lg p-6 mb-6"
                        } else {
                            "bg-white rounded-lg p-6 border mb-6"
                        }
                    }>
                        <h2 class=move || {
                            if dark_mode.get() {
                                "text-xl font-bold text-gray-100 mb-4 flex items-center"
                            } else {
                                "text-xl font-bold text-gray-800 mb-4 flex items-center"
                            }
                        }>
                            <span class="mr-2">"📋"</span>
                            "功能模块介绍"
                        </h2>
                        
                        <div class="space-y-4">
                            <div class=move || {
                                if dark_mode.get() { "bg-gray-800 rounded-lg p-4" } else { "bg-gray-50 rounded-lg p-4" }
                            }>
                                <h4 class=move || {
                                    if dark_mode.get() { "text-gray-200 font-semibold mb-2" } else { "text-gray-700 font-semibold mb-2" }
                                }>"🔗 连接管理"</h4>
                                <p class=move || {
                                    if dark_mode.get() { "text-gray-400 text-sm" } else { "text-gray-500 text-sm" }
                                }>"支持创建、编辑、删除数据库连接，支持多种数据库类型，提供连接测试功能。"</p>
                            </div>
                            <div class=move || {
                                if dark_mode.get() { "bg-gray-800 rounded-lg p-4" } else { "bg-gray-50 rounded-lg p-4" }
                            }>
                                <h4 class=move || {
                                    if dark_mode.get() { "text-gray-200 font-semibold mb-2" } else { "text-gray-700 font-semibold mb-2" }
                                }>"📝 SQL 查询编辑器"</h4>
                                <p class=move || {
                                    if dark_mode.get() { "text-gray-400 text-sm" } else { "text-gray-500 text-sm" }
                                }>"支持语法高亮的 SQL 编辑器，支持查询执行、结果展示、导出功能。"</p>
                            </div>
                            <div class=move || {
                                if dark_mode.get() { "bg-gray-800 rounded-lg p-4" } else { "bg-gray-50 rounded-lg p-4" }
                            }>
                                <h4 class=move || {
                                    if dark_mode.get() { "text-gray-200 font-semibold mb-2" } else { "text-gray-700 font-semibold mb-2" }
                                }>"📊 表结构查看"</h4>
                                <p class=move || {
                                    if dark_mode.get() { "text-gray-400 text-sm" } else { "text-gray-500 text-sm" }
                                }>"浏览数据库表结构，查看字段信息、索引、DDL 语句等。"</p>
                            </div>
                            <div class=move || {
                                if dark_mode.get() { "bg-gray-800 rounded-lg p-4" } else { "bg-gray-50 rounded-lg p-4" }
                            }>
                                <h4 class=move || {
                                    if dark_mode.get() { "text-gray-200 font-semibold mb-2" } else { "text-gray-700 font-semibold mb-2" }
                                }>"📜 查询历史"</h4>
                                <p class=move || {
                                    if dark_mode.get() { "text-gray-400 text-sm" } else { "text-gray-500 text-sm" }
                                }>"记录执行过的 SQL 查询历史，支持快速复用和管理。"</p>
                            </div>
                            <div class=move || {
                                if dark_mode.get() { "bg-gray-800 rounded-lg p-4" } else { "bg-gray-50 rounded-lg p-4" }
                            }>
                                <h4 class=move || {
                                    if dark_mode.get() { "text-gray-200 font-semibold mb-2" } else { "text-gray-700 font-semibold mb-2" }
                                }>"🔧 数据库工具"</h4>
                                <p class=move || {
                                    if dark_mode.get() { "text-gray-400 text-sm" } else { "text-gray-500 text-sm" }
                                }>"提供数据导入导出、SQL 格式化、数据对比等实用工具。"</p>
                            </div>
                        </div>
                    </div>

                    <div class=move || {
                        if dark_mode.get() {
                            "bg-gray-700 rounded-lg p-6 mb-6"
                        } else {
                            "bg-white rounded-lg p-6 border mb-6"
                        }
                    }>
                        <h2 class=move || {
                            if dark_mode.get() {
                                "text-xl font-bold text-gray-100 mb-4 flex items-center"
                            } else {
                                "text-xl font-bold text-gray-800 mb-4 flex items-center"
                            }
                        }>
                            <span class="mr-2">"🏗️"</span>
                            "系统架构"
                        </h2>
                        
                        <div class="space-y-4">
                            <div class=move || {
                                if dark_mode.get() { "bg-gray-800 rounded-lg p-4" } else { "bg-gray-50 rounded-lg p-4" }
                            }>
                                <h4 class=move || {
                                    if dark_mode.get() { "text-blue-400 font-semibold mb-2" } else { "text-blue-600 font-semibold mb-2" }
                                }>"┌─────────────────────────────────────────────┐"</h4>
                                <h4 class=move || {
                                    if dark_mode.get() { "text-gray-200 font-semibold mb-2" } else { "text-gray-700 font-semibold mb-2" }
                                }>"│ 前端层 (Frontend Layer)                   │"</h4>
                                <p class=move || {
                                    if dark_mode.get() { "text-gray-400 text-sm" } else { "text-gray-500 text-sm" }
                                }>"使用 Leptos 框架构建的单页应用，包含响应式 UI 组件、SQL 编辑器、数据表格等模块，通过 WASM 编译实现高性能前端体验。"</p>
                            </div>
                            
                            <div class="text-center">
                                <span class=move || {
                                    if dark_mode.get() { "text-gray-500" } else { "text-gray-400" }
                                }>"▼"</span>
                            </div>
                            
                            <div class=move || {
                                if dark_mode.get() { "bg-gray-800 rounded-lg p-4" } else { "bg-gray-50 rounded-lg p-4" }
                            }>
                                <h4 class=move || {
                                    if dark_mode.get() { "text-green-400 font-semibold mb-2" } else { "text-green-600 font-semibold mb-2" }
                                }>"┌─────────────────────────────────────────────┐"</h4>
                                <h4 class=move || {
                                    if dark_mode.get() { "text-gray-200 font-semibold mb-2" } else { "text-gray-700 font-semibold mb-2" }
                                }>"│ API 层 (Axum Backend)                    │"</h4>
                                <p class=move || {
                                    if dark_mode.get() { "text-gray-400 text-sm" } else { "text-gray-500 text-sm" }
                                }>"基于 Axum 框架的 RESTful API 服务，处理 Schema 查询、SQL 执行、连接管理等业务逻辑，支持连接池管理和异步数据库操作。"</p>
                            </div>
                            
                            <div class="text-center">
                                <span class=move || {
                                    if dark_mode.get() { "text-gray-500" } else { "text-gray-400" }
                                }>"▼"</span>
                            </div>
                            
                            <div class=move || {
                                if dark_mode.get() { "bg-gray-800 rounded-lg p-4" } else { "bg-gray-50 rounded-lg p-4" }
                            }>
                                <h4 class=move || {
                                    if dark_mode.get() { "text-yellow-400 font-semibold mb-2" } else { "text-yellow-600 font-semibold mb-2" }
                                }>"┌─────────────────────────────────────────────┐"</h4>
                                <h4 class=move || {
                                    if dark_mode.get() { "text-gray-200 font-semibold mb-2" } else { "text-gray-700 font-semibold mb-2" }
                                }>"│ 数据库层 (Database Layer)                 │"</h4>
                                <p class=move || {
                                    if dark_mode.get() { "text-gray-400 text-sm" } else { "text-gray-500 text-sm" }
                                }>"支持 PostgreSQL、MySQL、SQLite 三种主流关系型数据库，通过 SQLx 进行数据库交互，实现统一的数据库访问接口。"</p>
                            </div>
                        </div>
                    </div>

                    <div class=move || {
                        if dark_mode.get() {
                            "bg-gray-700 rounded-lg p-6"
                        } else {
                            "bg-white rounded-lg p-6 border"
                        }
                    }>
                        <h2 class=move || {
                            if dark_mode.get() {
                                "text-xl font-bold text-gray-100 mb-4 flex items-center"
                            } else {
                                "text-xl font-bold text-gray-800 mb-4 flex items-center"
                            }
                        }>
                            <span class="mr-2">"📅"</span>
                            "开发与更新历史"
                        </h2>
                        
                        <div class="space-y-3">
                            <div class=move || {
                                if dark_mode.get() { "flex items-start border-l-2 border-blue-500 pl-4" } else { "flex items-start border-l-2 border-blue-500 pl-4" }
                            }>
                                <span class=move || {
                                    if dark_mode.get() { "text-gray-400 text-sm font-medium" } else { "text-gray-500 text-sm font-medium" }
                                }>"v1.0.0 (2024-01-15)"</span>
                                <ul class=move || {
                                    if dark_mode.get() { "text-gray-300 text-sm ml-4" } else { "text-gray-600 text-sm ml-4" }
                                }>
                                    <li>"✨ 初始版本发布"</li>
                                    <li>"🔗 支持 PostgreSQL、MySQL、SQLite 连接"</li>
                                    <li>"📝 SQL 查询编辑器"</li>
                                    <li>"📊 表结构查看功能"</li>
                                </ul>
                            </div>
                            <div class=move || {
                                if dark_mode.get() { "flex items-start border-l-2 border-green-500 pl-4" } else { "flex items-start border-l-2 border-green-500 pl-4" }
                            }>
                                <span class=move || {
                                    if dark_mode.get() { "text-gray-400 text-sm font-medium" } else { "text-gray-500 text-sm font-medium" }
                                }>"v1.1.0 (2024-02-20)"</span>
                                <ul class=move || {
                                    if dark_mode.get() { "text-gray-300 text-sm ml-4" } else { "text-gray-600 text-sm ml-4" }
                                }>
                                    <li>"🔄 查询历史功能"</li>
                                    <li>"⭐ 查询收藏功能"</li>
                                    <li>"💾 数据导出功能"</li>
                                </ul>
                            </div>
                            <div class=move || {
                                if dark_mode.get() { "flex items-start border-l-2 border-yellow-500 pl-4" } else { "flex items-start border-l-2 border-yellow-500 pl-4" }
                            }>
                                <span class=move || {
                                    if dark_mode.get() { "text-gray-400 text-sm font-medium" } else { "text-gray-500 text-sm font-medium" }
                                }>"v1.2.0 (2024-03-25)"</span>
                                <ul class=move || {
                                    if dark_mode.get() { "text-gray-300 text-sm ml-4" } else { "text-gray-600 text-sm ml-4" }
                                }>
                                    <li>"🎨 深色模式支持"</li>
                                    <li>"⚡ 性能优化"</li>
                                    <li>"🔧 数据库工具集"</li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
