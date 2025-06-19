-- set up apps
CREATE TABLE apps (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    url TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (name)
);

-- set up pages
CREATE TABLE pages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    app_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE,
    UNIQUE (app_id, name)
);

-- create an index for (app_id, name) to optimize lookups
CREATE UNIQUE INDEX IF NOT EXISTS idx_pages_appid_name ON pages(app_id, name);

-- set up content {
CREATE TABLE content (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    page_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    body TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (page_id) REFERENCES pages(id) ON DELETE CASCADE,
    UNIQUE (page_id, name)
);

-- create a dummy app for the example org
INSERT INTO apps (name, description, url)
VALUES ('Example App', 'An example app for demonstration purposes.', 'https://example.com/app');
-- create a dummy page for the example app
INSERT INTO pages (app_id, name)
VALUES ((SELECT id FROM apps WHERE name = 'Example App'), 'home');
-- create a dummy content for the example page
INSERT INTO content (page_id, name, body)
VALUES ((SELECT id FROM pages WHERE name = 'home'), 'home_1', 'Sample Content 1');
INSERT INTO content (page_id, name, body)
VALUES ((SELECT id FROM pages WHERE name = 'home'), 'home_2', 'Sample Content 2');
