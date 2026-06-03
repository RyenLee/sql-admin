use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="text-center">
            <h1 class="text-4xl font-bold text-gray-800 dark:text-gray-100 mb-4">"404 - Page Not Found"</h1>
            <p class="text-gray-600 dark:text-gray-400 mb-8">"The page you're looking for doesn't exist."</p>
            <A href="/" attr:class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700">
                "Go Back Home"
            </A>
        </div>
    }
}
