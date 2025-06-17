-- Add migration script here
CREATE TABLE content (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    page_name TEXT NOT NULL,
    content TEXT NOT NULL,
    content_id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO content (page_name, content_id, content) VALUES ('home', 'home_1', 'Welcome to the home page!');

CREATE UNIQUE INDEX IF NOT EXISTS idx_content_page_name_content_id
ON content(page_name, content_id);