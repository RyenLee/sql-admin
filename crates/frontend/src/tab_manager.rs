#[derive(Clone, Debug, PartialEq)]
pub enum TabKind {
    Home,
    Query {
        connection_id: String,
    },
    Connections,
    TableStructure {
        connection_id: String,
        table_name: String,
    },
    RedbBrowser {
        connection_id: String,
    },
    QueryHistory,
    Bookmarks,
    Appearance,
    Layout,
    DatabaseTools,
    SqlUtilities,
    QuickStart,
    KeyboardShortcuts,
    About,
}

impl TabKind {
    pub fn route(&self) -> String {
        match self {
            TabKind::Home => "/".to_string(),
            TabKind::Query { .. } => "/query".to_string(),
            TabKind::Connections => "/connections".to_string(),
            TabKind::TableStructure {
                connection_id,
                table_name,
            } => format!("/table/{}/{}", connection_id, table_name),
            TabKind::RedbBrowser { connection_id } => format!("/redb/{}", connection_id),
            TabKind::QueryHistory => "/query-history".to_string(),
            TabKind::Bookmarks => "/bookmarks".to_string(),
            TabKind::Appearance => "/appearance".to_string(),
            TabKind::Layout => "/layout".to_string(),
            TabKind::DatabaseTools => "/database-tools".to_string(),
            TabKind::SqlUtilities => "/sql-utilities".to_string(),
            TabKind::QuickStart => "/quick-start".to_string(),
            TabKind::KeyboardShortcuts => "/keyboard-shortcuts".to_string(),
            TabKind::About => "/about".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tab {
    pub id: String,
    pub kind: TabKind,
    pub title: String,
}

#[derive(Clone)]
pub struct TabManager {
    pub tabs: Vec<Tab>,
    pub active_tab_id: Option<String>,
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active_tab_id: None,
        }
    }

    pub fn ensure_tab(&mut self, tab: Tab) {
        let exists = self.tabs.iter().any(|t| t.id == tab.id);
        if !exists {
            self.tabs.push(tab.clone());
        }
        self.active_tab_id = Some(tab.id.clone());
    }

    pub fn remove_tab(&mut self, tab_id: &str) {
        self.tabs.retain(|t| t.id != tab_id);
        if self.active_tab_id.as_deref() == Some(tab_id) {
            self.active_tab_id = self.tabs.last().map(|t| t.id.clone());
        }
    }

    pub fn active_tab(&self) -> Option<&Tab> {
        self.active_tab_id
            .as_ref()
            .and_then(|id| self.tabs.iter().find(|t| t.id == *id))
    }

    pub fn close_all_tabs_except_home(&mut self) {
        let home_tab_id = make_home_tab_id();
        self.tabs.retain(|t| t.id == home_tab_id);
        self.active_tab_id = Some(home_tab_id);
    }
}

pub fn make_home_tab_id() -> String {
    "home".to_string()
}

pub fn make_query_tab_id(conn_id: String) -> String {
    format!("query-{}", conn_id)
}

pub fn make_connections_tab_id() -> String {
    "connections".to_string()
}

pub fn make_table_structure_tab_id(conn_id: String, table: &str) -> String {
    format!("structure-{}-{}", conn_id, table)
}

pub fn make_redb_browser_tab_id(conn_id: String) -> String {
    format!("redb-{}", conn_id)
}
