CREATE TABLE IF NOT EXISTS comments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    post TEXT NOT NULL,
    parent INTEGER,
    content TEXT NOT NULL,
    writer TEXT NOT NULL,
    password TEXT NOT NULL,
    user_uuid TEXT NOT NULL,
    ip TEXT,
    created_at TEXT NOT NULL,
    deleted INTEGER NOT NULL DEFAULT 0
);