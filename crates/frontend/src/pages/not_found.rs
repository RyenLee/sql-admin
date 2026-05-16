use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NotFound() -> impl IntoView {
    let app_state = use_context::<crate::state::AppState>();
    let dark_mode = app_state.map(|s| s.dark_mode).unwrap_or(RwSignal::new(false));

    view! {
        <div class="text-center">
            <h1 class=move || {
                if dark_mode.get() { "text-4xl font-bold text-gray-100 mb-4" } else { "text-4xl font-bold text-gray-800 mb-4" }
            }>"404 - Page Not Found"</h1>
            <p class=move || {
                if dark_mode.get() { "text-gray-400 mb-8" } else { "text-gray-600 mb-8" }
            }>"The page you're looking for doesn't exist."</p>
            <A href="/" attr:class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700">
                "Go Back Home"
            </A>
        </div>
    }
}