use crate::api::client;
use leptos::prelude::*;
use sql_admin_api_types::{RedbBatchDeleteRequest, RedbEditRequest, RedbKeyValue, RedbQueryRequest};
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn RedbKeyViewer(connection_id: String, table_name: String) -> impl IntoView {
    let (keys, set_keys) = signal(Vec::<RedbKeyValue>::new());
    let (total, set_total) = signal(0u64);
    let (has_more, set_has_more) = signal(false);
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);
    let (offset, set_offset) = signal(0u64);
    let (prefix, set_prefix) = signal(String::new());
    let (selected_key, set_selected_key) = signal(None::<RedbKeyValue>);
    let (selected_keys, set_selected_keys) = signal(std::collections::HashSet::<String>::new());

    // Store props in StoredValue for FnMut closures
    let conn_id_stored = StoredValue::new(connection_id.clone());
    let table_stored = StoredValue::new(table_name.clone());

    let load_keys = StoredValue::new({
        let connection_id = connection_id.clone();
        let table_name = table_name.clone();
        move |pfx: String, off: u64| {
            let conn_id = connection_id.clone();
            let tbl = table_name.clone();
            spawn_local(async move {
                set_loading.set(true);
                let req = RedbQueryRequest {
                    connection_id: conn_id,
                    table: tbl,
                    key_prefix: if pfx.is_empty() { None } else { Some(pfx) },
                    key_pattern: None,
                    limit: 50,
                    offset: off,
                };
                match client::query_redb_keys(req).await {
                    Ok(result) => {
                        set_keys.set(result.keys);
                        set_total.set(result.total);
                        set_has_more.set(result.has_more);
                        set_error.set(None);
                        set_selected_keys.set(std::collections::HashSet::new());
                    }
                    Err(e) => {
                        set_error.set(Some(e));
                    }
                }
                set_loading.set(false);
            });
        }
    });

    load_keys.with_value(|f| f(prefix.get_untracked(), offset.get_untracked()));

    let on_prev = move |_| {
        let new_offset = offset.get_untracked().saturating_sub(50);
        set_offset.set(new_offset);
        load_keys.with_value(|f| f(prefix.get_untracked(), new_offset));
    };

    let on_next = move |_| {
        let new_offset = offset.get_untracked() + 50;
        set_offset.set(new_offset);
        load_keys.with_value(|f| f(prefix.get_untracked(), new_offset));
    };

    let on_search = move |_| {
        set_offset.set(0);
        load_keys.with_value(|f| f(prefix.get_untracked(), 0));
    };

    let toggle_select_all = move |_| {
        let current = selected_keys.get();
        let all_keys: Vec<String> = keys.get().iter().map(|k| k.key.clone()).collect();
        if current.len() == all_keys.len() && !all_keys.is_empty() {
            set_selected_keys.set(std::collections::HashSet::new());
        } else {
            set_selected_keys.set(all_keys.into_iter().collect());
        }
    };

    let toggle_key = move |key: String| {
        set_selected_keys.update(|s| {
            if s.contains(&key) {
                s.remove(&key);
            } else {
                s.insert(key);
            }
        });
    };

    let batch_delete_selected = move |_| {
        let sel: Vec<String> = selected_keys.get().into_iter().collect();
        if sel.is_empty() {
            return;
        }
        let conn_id = conn_id_stored.with_value(|v| v.clone());
        let tbl = table_stored.with_value(|v| v.clone());
        spawn_local(async move {
            let req = RedbBatchDeleteRequest {
                connection_id: conn_id.clone(),
                table: tbl.clone(),
                keys: sel,
            };
            match client::batch_delete_redb_keys(conn_id, req).await {
                Ok(_) => {
                    set_selected_keys.set(std::collections::HashSet::new());
                    set_selected_key.set(None);
                    load_keys.with_value(|f| f(prefix.get_untracked(), offset.get_untracked()));
                }
                Err(e) => {
                    set_error.set(Some(e));
                }
            }
        });
    };

    view! {
        <div class="flex flex-col h-full">
            <div class="flex items-center gap-2 px-4 py-2 border-b border-gray-200 dark:border-gray-700">
                <span class="text-sm text-gray-700 dark:text-gray-300">"Key Prefix:"</span>
                <input
                    type="text"
                    class="flex-1 px-3 py-1 border dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100 rounded text-sm focus:outline-none focus:ring-1 focus:ring-blue-500"
                    placeholder="Enter key prefix..."
                    prop:value=prefix
                    on:input=move |ev| set_prefix.set(event_target_value(&ev))
                    on:keydown=move |ev| {
                        if ev.key() == "Enter" {
                            on_search(());
                        }
                    }
                />
                <button
                    class="px-3 py-1 bg-blue-600 text-white text-sm rounded hover:bg-blue-700"
                    on:click=move |_| on_search(())
                >"Search"</button>
            </div>

            {move || if !selected_keys.get().is_empty() {
                view! {
                    <div class="sticky top-0 bg-yellow-50 dark:bg-yellow-900 border-b border-yellow-700 dark:border-yellow-700 px-4 py-2 flex items-center gap-3 z-10">
                        <span class="text-xs text-yellow-800 dark:text-yellow-200">
                            {move || format!("{} key(s) selected", selected_keys.get().len())}
                        </span>
                        <button
                            class="text-xs px-2 py-1 bg-red-500 text-white rounded hover:bg-red-600"
                            on:click=batch_delete_selected
                        >
                            "Delete Selected"
                        </button>
                    </div>
                }.into_any()
            } else {
                let _: () = view! {};
                ().into_any()
            }}

            <div class="flex-1 overflow-auto">
                {move || {
                    if loading.get() {
                        view! {
                            <div class="px-4 py-8 text-center">
                                <div class="text-gray-500 dark:text-gray-400">"Loading..."</div>
                            </div>
                        }.into_any()
                    } else if let Some(e) = error.get() {
                        view! {
                            <div class="px-4 py-8 text-center">
                                <div class="text-red-400">{e}</div>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <table class="w-full text-sm text-gray-800 dark:text-gray-200">
                                <thead>
                                    <tr class="bg-gray-50 dark:bg-gray-700">
                                        <th class="px-2 py-2 w-8">
                                            <input
                                                type="checkbox"
                                                class="rounded dark:bg-gray-700"
                                                on:change=toggle_select_all
                                                checked=move || {
                                                    let k = keys.get();
                                                    !k.is_empty() && selected_keys.get().len() == k.len()
                                                }
                                            />
                                        </th>
                                        <th class="px-4 py-2 text-left font-medium">"Key"</th>
                                        <th class="px-4 py-2 text-left font-medium">"Type"</th>
                                        <th class="px-4 py-2 text-left font-medium">"Value Preview"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <For each=move || keys.get() key=|kv| kv.key.clone() let(kv)>
                                        {
                                            let kv_for_click = kv.clone();
                                            let kv_for_class_key = kv.key.clone();
                                            let kv_for_type = kv.clone();
                                            let kv_for_checkbox_checked = kv.key.clone();
                                            let kv_for_checkbox_toggle = kv.key.clone();
                                            let key_display = kv.key.clone();
                                            let type_display = kv.value_type.clone();
                                            let value_preview = serde_json::to_string(&kv.value).unwrap_or_default().chars().take(80).collect::<String>();
                                            view! {
                                                <tr
                                                    class=move || {
                                                        let base = "cursor-pointer border-b ";
                                                        let border = "border-gray-100 dark:border-gray-700 ";
                                                        let hover = "hover:bg-gray-50 dark:hover:bg-gray-700";
                                                        let selected = if selected_key.get().map(|s| s.key == kv_for_class_key).unwrap_or(false) {
                                                            "bg-blue-50 dark:bg-gray-700"
                                                        } else if selected_keys.get().contains(&kv_for_class_key) {
                                                            "bg-yellow-50 dark:bg-yellow-900"
                                                        } else { "" };
                                                        format!("{}{}{}{}", base, border, hover, selected)
                                                    }
                                                    on:click=move |_| set_selected_key.set(Some(kv_for_click.clone()))
                                                >
                                                    <td class="px-2 py-1.5" on:click=move |ev| ev.stop_propagation()>
                                                        <input
                                                            type="checkbox"
                                                            class="rounded dark:bg-gray-700"
                                                            checked=move || selected_keys.get().contains(&kv_for_checkbox_checked)
                                                            on:change={
                                                                move |_| toggle_key(kv_for_checkbox_toggle.clone())
                                                            }
                                                        />
                                                    </td>
                                                    <td class="px-4 py-1.5 font-mono text-xs">{key_display}</td>
                                                    <td class="px-4 py-1.5">
                                                        <span class=move || {
                                                            "px-1.5 py-0.5 rounded text-xs ".to_string()
                                                                + match kv_for_type.value_type.as_str() {
                                                                    "object" | "array" => "bg-blue-900/30 text-blue-300",
                                                                    "number" => "bg-green-900/30 text-green-300",
                                                                    "boolean" => "bg-purple-900/30 text-purple-300",
                                                                    "null" => "bg-gray-700 text-gray-400",
                                                                    _ => "bg-orange-900/30 text-orange-300",
                                                                }
                                                        }>{type_display}</span>
                                                    </td>
                                                    <td class="px-4 py-1.5 font-mono text-xs truncate max-w-xs">
                                                        {value_preview}
                                                    </td>
                                                </tr>
                                            }
                                        }
                                    </For>
                                </tbody>
                            </table>
                        }.into_any()
                    }
                }}
            </div>

            <div class="flex items-center justify-between px-4 py-2 border-t border-gray-200 dark:border-gray-700 text-sm text-gray-700 dark:text-gray-300">
                <span>{move || format!("Showing {} of {} keys", keys.get().len(), total.get())}</span>
                <div class="flex gap-2">
                    <button
                        class="px-3 py-1 bg-gray-600 text-white rounded text-sm hover:bg-gray-700 disabled:opacity-50"
                        disabled=move || offset.get() == 0
                        on:click=on_prev
                    >"Prev"</button>
                    <button
                        class="px-3 py-1 bg-gray-600 text-white rounded text-sm hover:bg-gray-700 disabled:opacity-50"
                        disabled=move || !has_more.get()
                        on:click=on_next
                    >"Next"</button>
                </div>
            </div>

            {move || {
                selected_key.get().map(|kv| {
                    view! {
                        <div class="border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 px-4 py-3">
                            <div class="flex items-center justify-between mb-2">
                                <div class="flex items-center gap-2">
                                    <span class="text-sm font-medium text-gray-800 dark:text-gray-200">"Key: "</span>
                                    <span class="font-mono text-sm">{kv.key.clone()}</span>
                                    <span class="px-1.5 py-0.5 rounded text-xs bg-blue-900/30 text-blue-300">
                                        {kv.value_type.clone()}
                                    </span>
                                </div>
                                <div class="flex gap-2">
                                    <button
                                        class="px-3 py-1 bg-red-600 text-white text-sm rounded hover:bg-red-700"
                                        on:click={
                                            let key = kv.key.clone();
                                            move |_| {
                                                let conn_id = conn_id_stored.with_value(|v| v.clone());
                                                let tbl = table_stored.with_value(|v| v.clone());
                                                let req = RedbEditRequest {
                                                    connection_id: conn_id,
                                                    table: tbl,
                                                    key: key.clone(),
                                                    new_value: None,
                                                };
                                                spawn_local(async move {
                                                    if client::edit_redb_key(req).await.is_ok() {
                                                        set_selected_key.set(None);
                                                        load_keys.with_value(|f| f(prefix.get_untracked(), offset.get_untracked()));
                                                    }
                                                });
                                            }
                                        }
                                    >"Delete"</button>
                                </div>
                            </div>
                            <pre class="text-xs font-mono overflow-auto max-h-40 p-2 rounded bg-gray-100 dark:bg-gray-900 text-gray-800 dark:text-gray-200">{serde_json::to_string_pretty(&kv.value).unwrap_or_default()}</pre>
                        </div>
                    }
                })
            }}
        </div>
    }
}
