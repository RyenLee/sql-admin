use crate::api::client;
use leptos::prelude::*;
use sql_admin_api_types::RedbTableSummary;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn RedbTableList(
    connection_id: String,
    #[prop(default = Callback::new(|_: String| {}))]
    on_select: Callback<String, ()>,
) -> impl IntoView {
    let (tables, set_tables) = signal(Vec::<RedbTableSummary>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);

    let conn_id = connection_id.clone();
    spawn_local(async move {
        set_loading.set(true);
        match client::list_redb_tables(conn_id).await {
            Ok(result) => {
                set_tables.set(result);
                set_error.set(None);
            }
            Err(e) => {
                set_error.set(Some(e));
            }
        }
        set_loading.set(false);
    });

    view! {
        <div class="h-full overflow-y-auto">
            <div class="px-3 py-2 text-xs uppercase tracking-wider text-gray-500 dark:text-gray-400 font-semibold">"Tables"</div>

            {move || {
                if loading.get() {
                    view! {
                        <div class="px-3 py-4 text-center">
                            <div class="text-gray-500 dark:text-gray-400 text-sm">"Loading..."</div>
                        </div>
                    }.into_any()
                } else if let Some(e) = error.get() {
                    view! {
                        <div class="px-3 py-4 text-center">
                            <div class="text-red-400 text-sm">{e}</div>
                        </div>
                    }.into_any()
                } else {
                    let table_list = tables.get();
                    if table_list.is_empty() {
                        view! {
                            <div class="px-3 py-4 text-center">
                                <div class="text-gray-500 dark:text-gray-400 text-sm">"No tables found"</div>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <For each=move || tables.get() key=|t| t.name.clone() let(table)>
                                <div
                                    class="flex items-center px-3 py-1.5 text-sm cursor-pointer rounded mx-1 hover:bg-gray-100 dark:hover:bg-gray-700"
                                    on:click={
                                        let table_name = table.name.clone();
                                        let on_select = on_select;
                                        move |_| {
                                            on_select.run(table_name.clone());
                                        }
                                    }
                                >
                                    <span class="mr-1.5 text-orange-400">"\u{1f5c2}\u{fe0f}"</span>
                                    <span class="truncate text-gray-800 dark:text-gray-200">{table.name.clone()}</span>
                                    <span class="ml-auto text-xs text-gray-500 dark:text-gray-400">{format!("{} keys", table.key_count)}</span>
                                </div>
                            </For>
                        }.into_any()
                    }
                }
            }}
        </div>
    }
}
