use crate::state::use_app_state;
use leptos::prelude::*;

#[derive(Clone)]
struct ToolAction {
    name: String,
    icon: String,
}

#[component]
pub fn DatabaseToolsPage() -> impl IntoView {
    let app_state = use_app_state();
    let dark_mode = app_state.dark_mode;
    let (status_msg, set_status_msg) = signal(None::<String>);

    let tools = vec![
        ToolAction { name: "Refresh Database".to_string(), icon: "🔄".to_string() },
        ToolAction { name: "Analyze Tables".to_string(), icon: "📊".to_string() },
        ToolAction { name: "Vacuum Database".to_string(), icon: "🧹".to_string() },
        ToolAction { name: "Backup Database".to_string(), icon: "🔒".to_string() },
    ];

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
                    }>"Database Tools"</h1>
                </div>

                {move || {
                    let msg = status_msg.get();
                    msg.map(|m| view! {
                        <div class=move || {
                            if dark_mode.get() {
                                "mb-4 px-4 py-3 bg-green-900 border border-green-700 text-green-300 rounded-lg"
                            } else {
                                "mb-4 px-4 py-3 bg-green-50 border border-green-200 text-green-700 rounded-lg"
                            }
                        }>
                            {m}
                        </div>
                    })
                }}

                <div class=move || {
                    if dark_mode.get() {
                        "bg-gray-800 rounded-lg p-6"
                    } else {
                        "bg-gray-50 rounded-lg p-6"
                    }
                }>
                    <div class="grid grid-cols-2 gap-4">
                        {tools.into_iter().map(|tool| {
                            let tool_name_for_click = tool.name.clone();
                            let tool_name = tool.name.clone();
                            let tool_icon = tool.icon.clone();
                            view! {
                                <button
                                    class=move || {
                                        if dark_mode.get() {
                                            "px-4 py-4 bg-gray-700 text-gray-300 rounded-lg hover:bg-gray-600 cursor-pointer transition-colors text-left"
                                        } else {
                                            "px-4 py-4 bg-white text-gray-700 rounded-lg border hover:bg-gray-50 cursor-pointer transition-colors text-left"
                                        }
                                    }
                                    on:click={
                                        move |_| {
                                            set_status_msg.set(Some(format!("{} completed successfully.", tool_name_for_click)));
                                        }
                                    }
                                >
                                    <div class="text-xl mb-2">{tool_icon}</div>
                                    <div class="font-medium">{tool_name}</div>
                                </button>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </div>
        </div>
    }
}
