#![allow(clippy::unused_unit)]
mod api;
mod app;
mod components;
mod pages;
mod state;
mod tab_manager;
mod utils;

use app::App;
use leptos::prelude::*;

fn main() {
    mount_to_body(|| view! { <App /> })
}
