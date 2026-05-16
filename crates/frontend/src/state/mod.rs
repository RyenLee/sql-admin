use crate::tab_manager::{Tab, TabKind, TabManager, make_home_tab_id};
use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct AppStatus {
    pub connected_db: Option<String>,
    pub db_type: Option<String>,
    pub row_count: Option<usize>,
    pub exec_time: Option<String>,
    pub status_text: String,
    pub db_size: Option<String>,
}

impl Default for AppStatus {
    fn default() -> Self {
        Self {
            connected_db: None,
            db_type: None,
            row_count: None,
            exec_time: None,
            status_text: "Ready".to_string(),
            db_size: None,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub status: RwSignal<AppStatus>,
    pub dark_mode: RwSignal<bool>,
    pub tab_manager: RwSignal<TabManager>,
    pub refresh_trigger: RwSignal<u32>,
    pub pending_navigation: RwSignal<Option<String>>,
}

pub fn provide_app_state() {
    let mut tab_manager = TabManager::new();
    let home_tab = Tab {
        id: make_home_tab_id(),
        kind: TabKind::Home,
        title: "Home".to_string(),
    };
    tab_manager.ensure_tab(home_tab);

    let state = AppState {
        status: RwSignal::new(AppStatus::default()),
        dark_mode: RwSignal::new(false),
        tab_manager: RwSignal::new(tab_manager),
        refresh_trigger: RwSignal::new(0),
        pending_navigation: RwSignal::new(None),
    };
    provide_context(state);
}

pub fn use_app_state() -> AppState {
    use_context::<AppState>().expect("AppState not provided")
}
