CREATE TABLE IF NOT EXISTS query_history (
    id TEXT PRIMARY KEY,
    connection_id TEXT NOT NULL,
    connection_name TEXT NOT NULL DEFAULT '',
    query_text TEXT NOT NULL,
    execution_time_ms INTEGER,
    rows_count INTEGER,
    success INTEGER NOT NULL DEFAULT 1,
    error_message TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (connection_id) REFERENCES connections (id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_query_history_connection ON query_history (connection_id);

CREATE INDEX IF NOT EXISTS idx_query_history_created_at ON query_history (created_at DESC);

CREATE INDEX IF NOT EXISTS idx_query_history_success ON query_history (success);