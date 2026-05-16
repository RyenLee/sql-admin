use leptos::prelude::*;
use serde_json::Value;

#[component]
pub fn DataTable(
    columns: Vec<String>,
    rows: Vec<Vec<Value>>,
    #[prop(optional)] editable: bool,
    #[prop(optional)] on_cell_edit: Option<Callback<(usize, usize, String), ()>>,
    #[prop(optional)] on_row_delete: Option<Callback<usize, ()>>,
) -> impl IntoView {
    let app_state = use_context::<crate::state::AppState>();
    let dark_mode = app_state
        .map(|s| s.dark_mode)
        .unwrap_or(RwSignal::new(false));
    let rows = StoredValue::new(rows);
    let (editing_cell, set_editing_cell) = signal(None::<(usize, usize)>);
    let (edit_value, set_edit_value) = signal(String::new());
    let (selected_rows, set_selected_rows) = signal(std::collections::HashSet::<usize>::new());
    let (sort_col, set_sort_col) = signal(None::<usize>);
    let (sort_desc, set_sort_desc) = signal(false);
    let (column_filters, set_column_filters) = signal(vec![String::new(); columns.len()]);
    let (context_menu, set_context_menu) = signal(None::<(usize, usize, f64, f64)>);

    let toggle_select_all = move |_| {
        let current = selected_rows.get();
        let r = rows.read_value();
        let filtered = get_filtered_indices(&r, &column_filters.get());
        if current.len() == filtered.len() && !filtered.is_empty() {
            set_selected_rows.set(std::collections::HashSet::new());
        } else {
            set_selected_rows.set(filtered.into_iter().collect());
        }
    };

    let toggle_row = move |idx: usize| {
        set_selected_rows.update(|s| {
            if s.contains(&idx) {
                s.remove(&idx);
            } else {
                s.insert(idx);
            }
        });
    };

    let start_edit = move |row_idx: usize, col_idx: usize, current_val: String| {
        set_editing_cell.set(Some((row_idx, col_idx)));
        set_edit_value.set(current_val);
    };

    let commit_edit = move |_| {
        if let Some((row_idx, col_idx)) = editing_cell.get() {
            if let Some(ref cb) = on_cell_edit {
                cb.run((row_idx, col_idx, edit_value.get()));
            }
        }
        set_editing_cell.set(None);
    };

    let cancel_edit = move |_| {
        set_editing_cell.set(None);
    };

    let handle_sort = move |col_idx: usize| {
        set_sort_col.update(|current| {
            if *current == Some(col_idx) {
                set_sort_desc.update(|d| *d = !*d);
            } else {
                *current = Some(col_idx);
                set_sort_desc.set(false);
            }
        });
    };

    let sorted_indices = move || {
        let r = rows.read_value();
        let filters = column_filters.get();
        let mut indices = get_filtered_indices(&r, &filters);
        if let Some(sc) = sort_col.get() {
            let desc = sort_desc.get();
            indices.sort_by(|&a, &b| {
                let va = &r[a][sc];
                let vb = &r[b][sc];
                let sa = value_to_string(va);
                let sb = value_to_string(vb);
                if desc { sb.cmp(&sa) } else { sa.cmp(&sb) }
            });
        }
        indices
    };

    let delete_selected = move |_| {
        let selected: Vec<usize> = {
            let mut s: Vec<usize> = selected_rows.get().into_iter().collect();
            s.sort_by(|a, b| b.cmp(a));
            s
        };
        for idx in selected {
            if let Some(ref cb) = on_row_delete {
                cb.run(idx);
            }
        }
        set_selected_rows.set(std::collections::HashSet::new());
    };

    let handle_context_menu = move |ev: leptos::ev::MouseEvent, row_idx: usize, col_idx: usize| {
        ev.prevent_default();
        set_context_menu.set(Some((
            row_idx,
            col_idx,
            ev.client_x() as f64,
            ev.client_y() as f64,
        )));
    };

    #[cfg(target_arch = "wasm32")]
    fn copy_to_clipboard(text: &str) {
        let window = web_sys::window().unwrap();
        let navigator = window.navigator();
        let clipboard = navigator.clipboard();
        let _ = clipboard.write_text(text);
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn copy_to_clipboard(_text: &str) {}

    let copy_cell_value = move |row_idx: usize, col_idx: usize| {
        let r = rows.read_value();
        if row_idx < r.len() && col_idx < r[0].len() {
            copy_to_clipboard(&value_to_string(&r[row_idx][col_idx]));
        }
        set_context_menu.set(None);
    };

    let copy_row_csv = move |row_idx: usize| {
        let r = rows.read_value();
        if row_idx < r.len() {
            let line: Vec<String> = r[row_idx]
                .iter()
                .map(|v| {
                    let s = value_to_string(v);
                    if s.contains(',') || s.contains('"') || s.contains('\n') {
                        format!("\"{}\"", s.replace('"', "\"\""))
                    } else {
                        s
                    }
                })
                .collect();
            copy_to_clipboard(&line.join(","));
        }
        set_context_menu.set(None);
    };

    let set_cell_null = move |row_idx: usize, col_idx: usize| {
        if let Some(ref cb) = on_cell_edit {
            cb.run((row_idx, col_idx, "NULL".to_string()));
        }
        set_context_menu.set(None);
    };

    let delete_row_action = move |row_idx: usize| {
        if let Some(ref cb) = on_row_delete {
            cb.run(row_idx);
        }
        set_context_menu.set(None);
    };

    view! {
        <div class=move || {
            if dark_mode.get() {
                "overflow-auto flex-1 max-h-[70vh] min-h-[150px] bg-gray-900"
            } else {
                "overflow-auto flex-1 max-h-[70vh] min-h-[150px]"
            }
        }>
            {move || if editable && !selected_rows.get().is_empty() {
                view! {
                    <div class=move || {
                        if dark_mode.get() {
                            "sticky top-0 bg-yellow-900 border-b border-yellow-700 px-3 py-2 flex items-center gap-3 z-10"
                        } else {
                            "sticky top-0 bg-yellow-50 border-b px-3 py-2 flex items-center gap-3 z-10"
                        }
                    }>
                        <span class=move || {
                            if dark_mode.get() {
                                "text-xs text-yellow-200"
                            } else {
                                "text-xs text-yellow-800"
                            }
                        }>
                            {move || format!("{} row(s) selected", selected_rows.get().len())}
                        </span>
                        <button
                            class="text-xs px-2 py-1 bg-red-500 text-white rounded hover:bg-red-600"
                            on:click=delete_selected
                        >
                            "Delete Selected"
                        </button>
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }}

            <table class="w-full text-sm">
                <thead class=move || {
                    if dark_mode.get() {
                        "sticky top-0 bg-gray-800 z-10"
                    } else {
                        "sticky top-0 bg-gray-50 z-10"
                    }
                }>
                    <tr>
                        {if editable {
                            view! {
                                <th class="px-2 py-2 w-8">
                                    <input
                                        type="checkbox"
                                        class="rounded dark:bg-gray-700"
                                        on:change=toggle_select_all
                                        checked=move || {
                                            let r = rows.read_value();
                                            !r.is_empty() && selected_rows.get().len() == r.len()
                                        }
                                    />
                                </th>
                            }.into_any()
                        } else {
                            view! {}.into_any()
                        }}

                        {columns.iter().enumerate().map(|(col_idx, col)| {
                            let col = col.clone();
                            let col_idx_clone = col_idx;
                            view! {
                                <th
                                    class=move || {
                                        if dark_mode.get() {
                                            "px-3 py-2 text-left font-medium text-gray-300 whitespace-nowrap cursor-pointer select-none hover:bg-gray-700"
                                        } else {
                                            "px-3 py-2 text-left font-medium text-gray-600 whitespace-nowrap cursor-pointer select-none hover:bg-gray-100"
                                        }
                                    }
                                    on:click=move |_| handle_sort(col_idx_clone)
                                >
                                    <span class="flex items-center gap-1">
                                        {col}
                                        {move || {
                                            if sort_col.get() == Some(col_idx_clone) {
                                                if sort_desc.get() {
                                                    view! { <span class="text-xs">"▼"</span> }.into_any()
                                                } else {
                                                    view! { <span class="text-xs">"▲"</span> }.into_any()
                                                }
                                            } else {
                                                view! {}.into_any()
                                            }
                                        }}
                                    </span>
                                </th>
                            }
                        }).collect_view()}
                    </tr>
                    <tr class=move || {
                        if dark_mode.get() {
                            "bg-gray-800"
                        } else {
                            "bg-gray-50"
                        }
                    }>
                        {if editable {
                            view! { <th class="px-2 py-1"></th> }.into_any()
                        } else {
                            view! {}.into_any()
                        }}
                        {columns.iter().enumerate().map(|(col_idx, _col)| {
                            let col_idx_clone = col_idx;
                            view! {
                                <th class="px-1 py-1">
                                    <input
                                        type="text"
                                        class=move || {
                                            if dark_mode.get() {
                                                "w-full px-2 py-0.5 text-xs border border-gray-600 bg-gray-700 text-gray-200 rounded focus:outline-none focus:border-blue-400"
                                            } else {
                                                "w-full px-2 py-0.5 text-xs border border-gray-200 rounded focus:outline-none focus:border-blue-400"
                                            }
                                        }
                                        placeholder="Filter..."
                                        prop:value=move || column_filters.get().get(col_idx_clone).cloned().unwrap_or_default()
                                        on:input=move |ev| {
                                            let val = event_target_value(&ev);
                                            set_column_filters.update(|filters| {
                                                if col_idx_clone < filters.len() {
                                                    filters[col_idx_clone] = val;
                                                }
                                            });
                                        }
                                    />
                                </th>
                            }
                        }).collect_view()}
                    </tr>
                </thead>

                <tbody>
                    {move || {
                        sorted_indices().into_iter().map(|row_idx| {
                            let row = rows.read_value()[row_idx].clone();
                            let edit_state = editing_cell.get();

                            view! {
                                <tr class=move || {
                                    let is_selected = selected_rows.get().contains(&row_idx);
                                    if is_selected {
                                        if dark_mode.get() {
                                            "border-t bg-yellow-900"
                                        } else {
                                            "border-t bg-yellow-50"
                                        }
                                    } else {
                                        if dark_mode.get() {
                                            "border-t border-gray-700 hover:bg-gray-800"
                                        } else {
                                            "border-t hover:bg-blue-50"
                                        }
                                    }
                                }>
                                    {if editable {
                                        view! {
                                            <td class="px-2 py-1.5">
                                                <input
                                                    type="checkbox"
                                                    class="rounded dark:bg-gray-700"
                                                    checked=move || selected_rows.get().contains(&row_idx)
                                                    on:change=move |_| toggle_row(row_idx)
                                                />
                                            </td>
                                        }.into_any()
                                    } else {
                                        view! {}.into_any()
                                    }}

                                    {row.iter().enumerate().map(|(col_idx, cell)| {
                                        let is_editing = edit_state == Some((row_idx, col_idx));
                                        let cell_str = value_to_string(cell);
                                        let cell_str_for_edit = cell_str.clone();
                                        let is_null = matches!(cell, Value::Null);

                                        if is_editing && editable {
                                            view! {
                                                <td class="px-1 py-0.5">
                                                    <input
                                                        type="text"
                                                        class=move || {
                                                            if dark_mode.get() {
                                                                "w-full px-2 py-1 border border-blue-400 rounded text-sm bg-gray-800 text-white focus:outline-none focus:ring-1 focus:ring-blue-400"
                                                            } else {
                                                                "w-full px-2 py-1 border border-blue-400 rounded text-sm focus:outline-none focus:ring-1 focus:ring-blue-400"
                                                            }
                                                        }
                                                        prop:value=edit_value
                                                        on:input=move |ev| set_edit_value.set(event_target_value(&ev))
                                                        on:keydown=move |ev| {
                                                            if ev.key() == "Enter" {
                                                                commit_edit(());
                                                            } else if ev.key() == "Escape" {
                                                                cancel_edit(());
                                                            }
                                                        }
                                                        on:blur=move |_| commit_edit(())
                                                    />
                                                </td>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <td
                                                    class=if is_null {
                                                        if dark_mode.get() {
                                                            "px-3 py-1.5 text-gray-500 italic whitespace-nowrap cursor-default"
                                                        } else {
                                                            "px-3 py-1.5 text-gray-400 italic whitespace-nowrap cursor-default"
                                                        }
                                                    } else if editable {
                                                        if dark_mode.get() {
                                                            "px-3 py-1.5 text-gray-200 whitespace-nowrap max-w-xs truncate cursor-pointer hover:bg-gray-700"
                                                        } else {
                                                            "px-3 py-1.5 text-gray-800 whitespace-nowrap max-w-xs truncate cursor-pointer hover:bg-blue-100"
                                                        }
                                                    } else {
                                                        if dark_mode.get() {
                                                            "px-3 py-1.5 text-gray-200 whitespace-nowrap max-w-xs truncate"
                                                        } else {
                                                            "px-3 py-1.5 text-gray-800 whitespace-nowrap max-w-xs truncate"
                                                        }
                                                    }
                                                    on:dblclick=move |_| {
                                                        if editable {
                                                            let val = if is_null {
                                                                String::new()
                                                            } else {
                                                                cell_str_for_edit.clone()
                                                            };
                                                            start_edit(row_idx, col_idx, val);
                                                        }
                                                    }
                                                    on:contextmenu=move |ev| handle_context_menu(ev, row_idx, col_idx)
                                                >
                                                    {cell_str}
                                                </td>
                                            }.into_any()
                                        }
                                    }).collect_view()}
                                </tr>
                            }
                        }).collect_view()
                    }}
                </tbody>
            </table>
            {move || {
                if let Some((menu_row_idx, menu_col_idx, x, y)) = context_menu.get() {
                    view! {
                        <div
                            class=move || {
                                if dark_mode.get() {
                                    "fixed z-50 bg-gray-800 border border-gray-600 rounded-lg shadow-lg py-1 min-w-[140px] text-sm"
                                } else {
                                    "fixed z-50 bg-white border rounded-lg shadow-lg py-1 min-w-[140px] text-sm"
                                }
                            }
                            style=format!("left: {}px; top: {}px", x as i32, y as i32)
                            on:click=move |ev| ev.stop_propagation()
                        >
                            <button
                                class=move || {
                                    if dark_mode.get() {
                                        "w-full text-left px-3 py-1.5 hover:bg-gray-700 text-gray-200"
                                    } else {
                                        "w-full text-left px-3 py-1.5 hover:bg-blue-50 text-gray-700"
                                    }
                                }
                                on:click=move |_| copy_cell_value(menu_row_idx, menu_col_idx)
                            >
                                "Copy Cell Value"
                            </button>
                            <button
                                class=move || {
                                    if dark_mode.get() {
                                        "w-full text-left px-3 py-1.5 hover:bg-gray-700 text-gray-200"
                                    } else {
                                        "w-full text-left px-3 py-1.5 hover:bg-blue-50 text-gray-700"
                                    }
                                }
                                on:click=move |_| copy_row_csv(menu_row_idx)
                            >
                                "Copy Row as CSV"
                            </button>
                            {if editable {
                                view! {
                                    <div class="border-t border-gray-600 dark:border-gray-600"></div>
                                    <button
                                        class=move || {
                                            if dark_mode.get() {
                                                "w-full text-left px-3 py-1.5 hover:bg-gray-700 text-gray-200"
                                            } else {
                                                "w-full text-left px-3 py-1.5 hover:bg-blue-50 text-gray-700"
                                            }
                                        }
                                        on:click=move |_| set_cell_null(menu_row_idx, menu_col_idx)
                                    >
                                        "Set to NULL"
                                    </button>
                                    <button
                                        class=move || {
                                            if dark_mode.get() {
                                                "w-full text-left px-3 py-1.5 hover:bg-red-900 text-red-400"
                                            } else {
                                                "w-full text-left px-3 py-1.5 hover:bg-red-50 text-red-600"
                                            }
                                        }
                                        on:click=move |_| delete_row_action(menu_row_idx)
                                    >
                                        "Delete Row"
                                    </button>
                                }.into_any()
                            } else {
                                view! {}.into_any()
                            }}
                        </div>
                        <div
                            class="fixed inset-0 z-40"
                            on:click=move |_| set_context_menu.set(None)
                            on:contextmenu=move |ev| {
                                ev.prevent_default();
                                set_context_menu.set(None);
                            }
                        ></div>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}
        </div>
    }
}

fn get_filtered_indices(rows: &[Vec<Value>], filters: &[String]) -> Vec<usize> {
    let active_filters: Vec<(usize, &String)> = filters
        .iter()
        .enumerate()
        .filter(|(_, f)| !f.is_empty())
        .collect();

    if active_filters.is_empty() {
        return (0..rows.len()).collect();
    }

    rows.iter()
        .enumerate()
        .filter(|(_, row)| {
            active_filters.iter().all(|(col_idx, filter)| {
                let cell_str = row
                    .get(*col_idx)
                    .map(|v| value_to_string(v).to_lowercase())
                    .unwrap_or_default();
                cell_str.contains(&filter.to_lowercase())
            })
        })
        .map(|(idx, _)| idx)
        .collect()
}

fn value_to_string(cell: &Value) -> String {
    match cell {
        Value::Null => "NULL".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(arr) => serde_json::to_string(arr).unwrap_or_default(),
        Value::Object(obj) => serde_json::to_string(obj).unwrap_or_default(),
    }
}
