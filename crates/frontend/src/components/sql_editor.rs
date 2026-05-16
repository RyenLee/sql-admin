use crate::state::use_app_state;
use leptos::html;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

static SQL_KEYWORDS: &[&str] = &[
    "SELECT",
    "FROM",
    "WHERE",
    "INSERT",
    "INTO",
    "VALUES",
    "UPDATE",
    "SET",
    "DELETE",
    "CREATE",
    "TABLE",
    "DROP",
    "ALTER",
    "ADD",
    "COLUMN",
    "INDEX",
    "VIEW",
    "TRIGGER",
    "JOIN",
    "INNER",
    "LEFT",
    "RIGHT",
    "OUTER",
    "ON",
    "AND",
    "OR",
    "NOT",
    "NULL",
    "IS",
    "IN",
    "LIKE",
    "BETWEEN",
    "EXISTS",
    "ORDER",
    "BY",
    "GROUP",
    "HAVING",
    "LIMIT",
    "OFFSET",
    "AS",
    "DISTINCT",
    "COUNT",
    "SUM",
    "AVG",
    "MIN",
    "MAX",
    "UNION",
    "ALL",
    "CASE",
    "WHEN",
    "THEN",
    "ELSE",
    "END",
    "PRIMARY",
    "KEY",
    "FOREIGN",
    "REFERENCES",
    "CONSTRAINT",
    "DEFAULT",
    "CHECK",
    "UNIQUE",
    "AUTOINCREMENT",
    "BEGIN",
    "COMMIT",
    "ROLLBACK",
    "TRANSACTION",
    "IF",
    "EXPLAIN",
    "VACUUM",
    "ANALYZE",
    "REINDEX",
    "PRAGMA",
    "INTEGER",
    "TEXT",
    "BLOB",
    "REAL",
    "NUMERIC",
    "BOOLEAN",
    "DATE",
    "TIMESTAMP",
    "VARCHAR",
    "CHAR",
    "BIGINT",
    "SMALLINT",
    "FLOAT",
    "DOUBLE",
    "DECIMAL",
    "SERIAL",
    "BIGSERIAL",
    "UUID",
    "CASCADE",
    "RESTRICT",
    "NO",
    "ACTION",
    "REPLACE",
    "ABORT",
    "ASC",
    "DESC",
    "NULLS",
    "FIRST",
    "LAST",
    "WITH",
    "RECURSIVE",
    "RETURNING",
    "OVER",
    "PARTITION",
    "WINDOW",
    "ROWS",
    "RANGE",
    "UNBOUNDED",
    "PRECEDING",
    "FOLLOWING",
    "CURRENT",
    "ROW",
    "INTERSECT",
    "EXCEPT",
    "CROSS",
    "NATURAL",
    "USING",
    "FULL",
    "LATERAL",
    "MATERIALIZED",
    "TEMP",
    "TEMPORARY",
    "IF",
    "NOT",
    "EXISTS",
    "SCHEMA",
    "DATABASE",
    "TO",
    "GRANT",
    "REVOKE",
    "TRUNCATE",
    "RENAME",
    "COALESCE",
    "CAST",
    "CONVERT",
    "EXTRACT",
    "SUBSTRING",
    "POSITION",
    "TRIM",
    "UPPER",
    "LOWER",
    "LENGTH",
    "CONCAT",
    "ABS",
    "ROUND",
    "CEIL",
    "FLOOR",
    "POWER",
    "SQRT",
    "MOD",
    "NOW",
    "CURRENT_DATE",
    "CURRENT_TIME",
    "CURRENT_TIMESTAMP",
    "STATEMENT",
    "PREPARE",
    "EXECUTE",
    "DEALLOCATE",
    "FETCH",
    "OPEN",
    "CLOSE",
    "CURSOR",
    "SAVEPOINT",
    "RELEASE",
    "ILIKE",
    "SIMILAR",
    "REGEXP",
    "RLIKE",
    "GLOB",
    "TRUE",
    "FALSE",
    "UNKNOWN",
    "READ",
    "WRITE",
    "ONLY",
    "DEFERRABLE",
    "IMMEDIATE",
    "CONCURRENTLY",
    "USING",
    "HASH",
    "BTREE",
    "GIST",
    "GIN",
    "BRIN",
    "COPY",
    "DELIMITER",
    "CSV",
    "HEADER",
    "QUOTE",
    "ESCAPE",
    "FORCE",
    "USE",
    "IGNORE",
    "STRAIGHT_JOIN",
    "HIGH_PRIORITY",
    "LOW_PRIORITY",
    "DELAYED",
    "QUICK",
    "LOCK",
    "SHARE",
];

static SQL_FUNCTIONS: &[&str] = &[
    "COUNT",
    "SUM",
    "AVG",
    "MIN",
    "MAX",
    "COALESCE",
    "CAST",
    "CONVERT",
    "EXTRACT",
    "SUBSTRING",
    "POSITION",
    "TRIM",
    "UPPER",
    "LOWER",
    "LENGTH",
    "CONCAT",
    "ABS",
    "ROUND",
    "CEIL",
    "FLOOR",
    "POWER",
    "SQRT",
    "MOD",
    "NOW",
    "CURRENT_DATE",
    "CURRENT_TIME",
    "CURRENT_TIMESTAMP",
    "GROUP_CONCAT",
    "IFNULL",
    "NULLIF",
    "TYPEOF",
    "TOTAL",
    "HEX",
    "UNHEX",
    "ZEROBLOL",
    "RANDOMBLOB",
    "LIKELIHOOD",
    "LIKELY",
    "UNLIKELY",
    "INSTR",
    "REPLACE",
    "SUBSTR",
    "LTRIM",
    "RTRIM",
    "CHAR",
    "UNICODE",
    "LAST_INSERT_ROWID",
    "CHANGES",
    "TOTAL_CHANGES",
    "JSON",
    "JSON_EXTRACT",
    "JSON_SET",
    "JSON_REMOVE",
    "JSON_ARRAY",
    "JSON_OBJECT",
    "JSON_TYPE",
    "JSON_VALID",
    "JSON_QUOTE",
    "JSON_GROUP_ARRAY",
    "JSON_GROUP_OBJECT",
    "JULIANDAY",
    "STRFTIME",
    "DATE",
    "TIME",
    "DATETIME",
    "CURRENT_TIMESTAMP",
    "CURRENT_DATE",
    "CURRENT_TIME",
];

struct ThemeColors {
    keyword: &'static str,
    function: &'static str,
    string: &'static str,
    ident: &'static str,
    number: &'static str,
    comment: &'static str,
    gutter_bg: &'static str,
    editor_bg: &'static str,
    gutter_text: &'static str,
    caret: &'static str,
}

static DARK_COLORS: ThemeColors = ThemeColors {
    keyword: "text-blue-400 font-semibold",
    function: "text-purple-400",
    string: "text-green-400",
    ident: "text-yellow-400",
    number: "text-orange-400",
    comment: "text-gray-500 italic",
    gutter_bg: "#1e1e2e",
    editor_bg: "#1a1a2e",
    gutter_text: "text-gray-500",
    caret: "caret-gray-300",
};

static LIGHT_COLORS: ThemeColors = ThemeColors {
    keyword: "text-blue-700 font-semibold",
    function: "text-purple-700",
    string: "text-green-700",
    ident: "text-yellow-700",
    number: "text-orange-700",
    comment: "text-gray-400 italic",
    gutter_bg: "#f0f0f0",
    editor_bg: "#fafafa",
    gutter_text: "text-gray-400",
    caret: "caret-gray-700",
};

fn highlight_sql(sql: &str, colors: &ThemeColors) -> String {
    let mut result = String::new();
    let mut chars = sql.chars().peekable();
    let mut current_word = String::new();

    while let Some(ch) = chars.next() {
        if ch == '\'' {
            if !current_word.is_empty() {
                result.push_str(&highlight_word(&current_word, colors));
                current_word.clear();
            }
            result.push_str(&format!(r#"<span class="{}">'"#, colors.string));
            let mut string_content = String::from("'");
            loop {
                match chars.next() {
                    Some('\'') => {
                        string_content.push('\'');
                        if chars.peek() == Some(&'\'') {
                            chars.next();
                            string_content.push('\'');
                            continue;
                        }
                        break;
                    }
                    Some(c) => string_content.push(c),
                    None => break,
                }
            }
            let escaped = html_escape(&string_content);
            result.push_str(&escaped);
            result.push_str("</span>");
        } else if ch == '"' {
            if !current_word.is_empty() {
                result.push_str(&highlight_word(&current_word, colors));
                current_word.clear();
            }
            result.push_str(&format!(r#"<span class="{}">""#, colors.ident));
            let mut ident = String::from("\"");
            loop {
                match chars.next() {
                    Some('"') => {
                        ident.push('"');
                        if chars.peek() == Some(&'"') {
                            ident.push('"');
                            continue;
                        }
                        break;
                    }
                    Some(c) => ident.push(c),
                    None => break,
                }
            }
            let escaped = html_escape(&ident);
            result.push_str(&escaped);
            result.push_str("</span>");
        } else if ch == '-' && chars.peek() == Some(&'-') {
            if !current_word.is_empty() {
                result.push_str(&highlight_word(&current_word, colors));
                current_word.clear();
            }
            let mut comment = String::from("--");
            chars.next();
            loop {
                match chars.next() {
                    Some('\n') | None => break,
                    Some(c) => comment.push(c),
                }
            }
            result.push_str(&format!(r#"<span class="{}">"#, colors.comment));
            result.push_str(&html_escape(&comment));
            result.push_str("</span>\n");
        } else if ch == '/' && chars.peek() == Some(&'*') {
            if !current_word.is_empty() {
                result.push_str(&highlight_word(&current_word, colors));
                current_word.clear();
            }
            let mut comment = String::from("/*");
            chars.next();
            loop {
                match chars.next() {
                    Some('*') => {
                        comment.push('*');
                        if chars.peek() == Some(&'/') {
                            comment.push('/');
                            chars.next();
                            break;
                        }
                    }
                    Some('\n') => {
                        comment.push('\n');
                    }
                    Some(c) => comment.push(c),
                    None => break,
                }
            }
            result.push_str(&format!(r#"<span class="{}">"#, colors.comment));
            result.push_str(&html_escape(&comment));
            result.push_str("</span>");
        } else if ch.is_alphanumeric() || ch == '_' || ch == '$' {
            current_word.push(ch);
        } else {
            if !current_word.is_empty() {
                result.push_str(&highlight_word(&current_word, colors));
                current_word.clear();
            }
            if ch == '\n' {
                result.push('\n');
            } else {
                result.push_str(&html_escape(&ch.to_string()));
            }
        }
    }

    if !current_word.is_empty() {
        result.push_str(&highlight_word(&current_word, colors));
    }

    result
}

fn highlight_word(word: &str, colors: &ThemeColors) -> String {
    let upper = word.to_uppercase();
    if SQL_KEYWORDS.contains(&upper.as_str()) {
        format!(
            r#"<span class="{}">{}</span>"#,
            colors.keyword,
            html_escape(word)
        )
    } else if SQL_FUNCTIONS.contains(&upper.as_str()) {
        format!(
            r#"<span class="{}">{}</span>"#,
            colors.function,
            html_escape(word)
        )
    } else if word.chars().all(|c| c.is_ascii_digit() || c == '.') {
        format!(r#"<span class="{}">{}</span>"#, colors.number, word)
    } else {
        html_escape(word)
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[component]
pub fn SqlEditor(
    query: ReadSignal<String>,
    set_query: WriteSignal<String>,
    #[prop(optional)] on_execute: Option<Callback<(), ()>>,
) -> impl IntoView {
    let app_state = use_app_state();
    let dark_mode = app_state.dark_mode;
    let (line_count, set_line_count) = signal(1usize);
    let highlight_ref: NodeRef<html::Div> = NodeRef::new();
    let gutter_ref: NodeRef<html::Div> = NodeRef::new();

    let update_line_count = move |text: &str| {
        set_line_count.set(text.lines().count().max(1));
    };

    view! {
        <div class=move || {
            if dark_mode.get() {
                "relative flex border border-gray-700 rounded-md overflow-hidden bg-gray-900"
            } else {
                "relative flex border border-gray-300 rounded-md overflow-hidden bg-white"
            }
        }>
            <div class="flex-shrink-0 pl-2 pr-1 select-none border-r overflow-hidden"
                style=move || {
                    let border_color = if dark_mode.get() { "#374151" } else { "#d1d5db" };
                    let bg = if dark_mode.get() { DARK_COLORS.gutter_bg } else { LIGHT_COLORS.gutter_bg };
                    format!("background-color: {}; min-width: 2.5rem; border-color: {}; height: 280px;", bg, border_color)
                }
            >
                <div
                    node_ref=gutter_ref
                    class=move || {
                        format!("font-mono text-xs {} leading-relaxed whitespace-pre-wrap text-right py-3", if dark_mode.get() { DARK_COLORS.gutter_text } else { LIGHT_COLORS.gutter_text })
                    }
                    style="line-height: 1.5;"
                >
                    {move || {
                        (1..=line_count.get()).map(|n| {
                            view! {
                                <div>{n}</div>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </div>
            </div>

            <div class="relative flex-1" style="height: 280px;">
                <div
                    node_ref=highlight_ref
                    class="absolute inset-0 py-3 px-3 font-mono text-sm pointer-events-none whitespace-pre-wrap overflow-auto"
                    style=move || {
                        let bg = if dark_mode.get() { DARK_COLORS.editor_bg } else { LIGHT_COLORS.editor_bg };
                        format!("line-height: 1.5; tab-size: 4; background-color: {};", bg)
                    }>
                    {move || {
                        let colors = if dark_mode.get() { &DARK_COLORS } else { &LIGHT_COLORS };
                        let highlighted = highlight_sql(&query.get(), colors);
                        view! {
                            <div inner_html=highlighted></div>
                        }
                    }}
                </div>
                <textarea
                    class=move || {
                        format!(
                            "relative z-10 w-full h-full py-3 px-3 font-mono text-sm resize-none bg-transparent text-transparent {} outline-none",
                            if dark_mode.get() { DARK_COLORS.caret } else { LIGHT_COLORS.caret }
                        )
                    }
                    style="line-height: 1.5; tab-size: 4; -webkit-text-fill-color: transparent;"
                    placeholder="Enter your SQL query..."
                    spellcheck="false"
                    prop:value=query
                    on:scroll=move |ev| {
                        let textarea = ev.target()
                            .and_then(|t| t.dyn_into::<leptos::web_sys::HtmlTextAreaElement>().ok());
                        let highlight_el = highlight_ref.get();
                        let gutter_el = gutter_ref.get();
                        if let Some(ta) = textarea {
                            if let Some(he) = &highlight_el {
                                he.set_scroll_top(ta.scroll_top());
                                he.set_scroll_left(ta.scroll_left());
                            }
                            if let Some(ge) = &gutter_el {
                                ge.set_scroll_top(ta.scroll_top());
                            }
                        }
                    }
                    on:input=move |ev| {
                        let val = event_target_value(&ev);
                        update_line_count(&val);
                        set_query.set(val);
                    }
                    on:keydown=move |ev| {
                        let key = ev.key();
                        if key == "Tab" {
                            ev.prevent_default();
                            let textarea = ev.target()
                                .unwrap()
                                .dyn_into::<leptos::web_sys::HtmlTextAreaElement>()
                                .unwrap();
                            let start = textarea.selection_start().ok().flatten().unwrap_or(0) as usize;
                            let end = textarea.selection_end().ok().flatten().unwrap_or(0) as usize;
                            let mut current = query.get();
                            if start != end {
                                let before = &current[..start];
                                let selected = &current[start..end];
                                let after = &current[end..];
                                let indented: String = selected
                                    .lines()
                                    .map(|line| format!("    {}", line))
                                    .collect::<Vec<_>>()
                                    .join("\n");
                                current = format!("{}{}{}", before, indented, after);
                                set_query.set(current);
                                let _ = textarea.set_selection_start(Some(start as u32));
                                let _ = textarea.set_selection_end(Some((start + indented.len()) as u32));
                            } else {
                                let (before, after) = current.split_at(start);
                                current = format!("{}    {}", before, after);
                                set_query.set(current);
                                let _ = textarea.set_selection_start(Some((start + 4) as u32));
                                let _ = textarea.set_selection_end(Some((start + 4) as u32));
                            }
                        } else if key == "Enter" && (ev.ctrl_key() || ev.meta_key()) {
                            ev.prevent_default();
                            if let Some(ref cb) = on_execute {
                                cb.run(());
                            }
                        }
                    }
                />
            </div>
        </div>
    }
}
