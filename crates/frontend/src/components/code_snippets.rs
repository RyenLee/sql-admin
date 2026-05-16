#[cfg(target_arch = "wasm32")]
use gloo_timers::callback::Timeout;
use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct CodeSnippet {
    #[allow(dead_code)]
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub category: &'static str,
    pub code: &'static str,
}

pub const PRESET_SNIPPETS: &[CodeSnippet] = &[
    CodeSnippet {
        id: "1",
        title: "SELECT Basic",
        description: "Get first N records from table",
        category: "SELECT",
        code: "SELECT * FROM table_name LIMIT 10;",
    },
    CodeSnippet {
        id: "2",
        title: "SELECT with WHERE",
        description: "Filter records by condition",
        category: "SELECT",
        code: "SELECT column1, column2 FROM table_name WHERE condition;",
    },
    CodeSnippet {
        id: "3",
        title: "ORDER BY",
        description: "Sort by specified column",
        category: "SELECT",
        code: "SELECT * FROM table_name ORDER BY column_name DESC;",
    },
    CodeSnippet {
        id: "4",
        title: "GROUP BY",
        description: "Group and aggregate data",
        category: "SELECT",
        code: "SELECT column, COUNT(*) FROM table_name GROUP BY column;",
    },
    CodeSnippet {
        id: "5",
        title: "JOIN Tables",
        description: "Join multiple tables",
        category: "JOIN",
        code: "SELECT a.*, b.column FROM table_a a JOIN table_b b ON a.id = b.a_id;",
    },
    CodeSnippet {
        id: "6",
        title: "INSERT",
        description: "Insert new record",
        category: "INSERT",
        code: "INSERT INTO table_name (col1, col2) VALUES ('value1', 'value2');",
    },
    CodeSnippet {
        id: "7",
        title: "UPDATE",
        description: "Update existing record",
        category: "UPDATE",
        code: "UPDATE table_name SET column = 'value' WHERE id = 1;",
    },
    CodeSnippet {
        id: "8",
        title: "DELETE",
        description: "Delete records",
        category: "DELETE",
        code: "DELETE FROM table_name WHERE condition;",
    },
    CodeSnippet {
        id: "9",
        title: "CREATE TABLE",
        description: "Create new table",
        category: "DDL",
        code: "CREATE TABLE table_name (id INTEGER PRIMARY KEY, name TEXT);",
    },
    CodeSnippet {
        id: "10",
        title: "INDEX",
        description: "Create index",
        category: "DDL",
        code: "CREATE INDEX idx_name ON table_name(column_name);",
    },
    CodeSnippet {
        id: "11",
        title: "View Schema",
        description: "View table structure",
        category: "UTIL",
        code: "PRAGMA table_info(table_name);",
    },
    CodeSnippet {
        id: "12",
        title: "Search Table",
        description: "Search all tables",
        category: "UTIL",
        code: "SELECT name FROM sqlite_master WHERE type='table';",
    },
];

#[component]
pub fn CodeSnippetPanel(on_select: Callback<String>) -> impl IntoView {
    let dark_mode = use_context::<crate::state::AppState>()
        .map(|s| s.dark_mode)
        .unwrap_or(RwSignal::new(false));

    let snippets = PRESET_SNIPPETS;

    view! {
        <div class=move || {
            if dark_mode.get() {
                "bg-gray-800 rounded-lg shadow h-full flex flex-col"
            } else {
                "bg-white rounded-lg shadow h-full flex flex-col"
            }
        }>
            <div class=move || {
                if dark_mode.get() {
                    "p-3 border-b border-gray-700"
                } else {
                    "p-3 border-b"
                }
            }>
                <h3 class=move || {
                    if dark_mode.get() {
                        "text-sm font-semibold text-gray-300"
                    } else {
                        "text-sm font-semibold text-gray-700"
                    }
                }>"Code Snippets"</h3>
            </div>

            <div class="flex-1 overflow-auto p-2 space-y-2">
                {snippets.iter().map(|snippet| {
                    view! {
                        <CodeSnippetCard
                            snippet=snippet.clone()
                            dark_mode=dark_mode
                            on_select=on_select.clone()
                        />
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

#[component]
pub fn CodeSnippetCard(
    snippet: CodeSnippet,
    dark_mode: RwSignal<bool>,
    on_select: Callback<String>,
) -> impl IntoView {
    #[allow(unused_variables)]
    let (copied, set_copied) = signal(false);

    let handle_copy = move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = leptos::web_sys::window()
                .unwrap()
                .navigator()
                .clipboard()
                .write_text(snippet.code);
            set_copied.set(true);
            Timeout::new(2000, move || {
                set_copied.set(false);
            })
            .forget();
        }
    };

    let handle_click = move |_| {
        on_select.run(snippet.code.to_string());
    };

    let highlighted_code = highlight_sql(snippet.code);

    let category_class = move || {
        let base = "text-xs font-medium px-2 py-0.5 rounded";
        if dark_mode.get() {
            format!("{} bg-gray-600 text-gray-200", base)
        } else {
            format!("{} bg-blue-100 text-blue-700", base)
        }
    };

    let title_class = move || {
        if dark_mode.get() {
            "text-sm font-semibold text-gray-200"
        } else {
            "text-sm font-semibold text-gray-800"
        }
    };

    let card_class = move || {
        if dark_mode.get() {
            "bg-gray-700/50 hover:bg-gray-700 border border-gray-600 rounded-lg p-3 cursor-pointer transition-colors"
        } else {
            "bg-gray-50 hover:bg-gray-100 border border-gray-200 rounded-lg p-3 cursor-pointer transition-colors"
        }
    };

    let copy_button_class = move || {
        if copied.get() {
            if dark_mode.get() {
                "p-1.5 rounded hover:bg-gray-600 text-green-400"
            } else {
                "p-1.5 rounded hover:bg-gray-200 text-green-600"
            }
        } else {
            if dark_mode.get() {
                "p-1.5 rounded hover:bg-gray-600 text-gray-400 hover:text-gray-200"
            } else {
                "p-1.5 rounded hover:bg-gray-200 text-gray-500 hover:text-gray-700"
            }
        }
    };

    let description_class = move || {
        if dark_mode.get() {
            "text-xs text-gray-400 mb-2"
        } else {
            "text-xs text-gray-500 mb-2"
        }
    };

    let pre_class = move || {
        if dark_mode.get() {
            "text-xs bg-gray-800 rounded p-2 overflow-x-auto font-mono"
        } else {
            "text-xs bg-white rounded p-2 overflow-x-auto font-mono border border-gray-200"
        }
    };

    let code_class = move || {
        if dark_mode.get() {
            "text-gray-300"
        } else {
            "text-gray-700"
        }
    };

    view! {
        <div class=card_class on:click=handle_click>
            <div class="flex items-start justify-between mb-2">
                <div class="flex items-center gap-2">
                    <span class=category_class>{snippet.category}</span>
                    <span class=title_class>{snippet.title}</span>
                </div>
                <button
                    class=copy_button_class
                    title=move || if copied.get() { "Copied!" } else { "Copy" }
                    on:click=handle_copy
                >
                    {move || {
                        if copied.get() {
                            view! { <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg> }.into_any()
                        } else {
                            view! { <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path></svg> }.into_any()
                        }
                    }}
                </button>
            </div>

            <p class=description_class>{snippet.description}</p>

            <pre class=pre_class>
                <code inner_html=highlighted_code class=code_class>
                </code>
            </pre>
        </div>
    }
}

fn highlight_sql(code: &str) -> String {
    let keywords = [
        "SELECT", "FROM", "WHERE", "AND", "OR", "NOT", "IN", "LIKE", "ORDER", "BY", "GROUP",
        "HAVING", "JOIN", "ON", "INSERT", "INTO", "VALUES", "UPDATE", "SET", "DELETE", "CREATE",
        "TABLE", "INDEX", "PRIMARY", "KEY", "TEXT", "INTEGER", "PRAGMA", "ASC", "DESC", "LIMIT",
        "COUNT", "SUM", "AVG", "MAX", "MIN", "AS",
    ];

    let mut result = code.to_string();

    for keyword in keywords {
        let pattern = match regex::Regex::new(&format!(r"\b({})\b", keyword)) {
            Ok(p) => p,
            Err(_) => continue,
        };
        result = pattern
            .replace_all(&result, |caps: &regex::Captures| {
                format!(
                    "<span class=\"text-blue-400 font-semibold\">{}</span>",
                    &caps[1]
                )
            })
            .to_string();
    }

    result
}
