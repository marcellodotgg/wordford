-- set up orgs
CREATE TABLE orgs (
    id TEXT PRIMARY KEY,
    slug TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    logo TEXT,
    url TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- create an index for slug to optimize lookups
CREATE UNIQUE INDEX IF NOT EXISTS idx_orgs_slug ON orgs(slug);

-- set up apps
CREATE TABLE apps (
    id TEXT PRIMARY KEY,
    org_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    url TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (org_id) REFERENCES orgs(id),
    UNIQUE (org_id, name)
);

-- set up pages
CREATE TABLE pages (
    id TEXT PRIMARY KEY,
    app_id TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (app_id) REFERENCES apps(id),
    UNIQUE (app_id, name)
);

-- create an index for (app_id, name) to optimize lookups
CREATE UNIQUE INDEX IF NOT EXISTS idx_pages_appid_name ON pages(app_id, name);

-- set up content {
CREATE TABLE content (
    id TEXT PRIMARY KEY,
    page_id TEXT NOT NULL,
    name TEXT NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (page_id) REFERENCES pages(id),
    UNIQUE (page_id, name)
);

-- create a dummy org for initial setup
INSERT INTO orgs (id, slug, name, description, logo, url)
VALUES ("1", 'example-org', 'Example Organization', 'An example organization for demonstration purposes.', 'https://example.com/logo.png', 'https://example.com');
-- create a dummy app for the example org
INSERT INTO apps (id, org_id, name, description, url)
VALUES ("1", (SELECT id FROM orgs WHERE slug = 'example-org'), 'Example App', 'An example app for demonstration purposes.', 'https://example.com/app');
-- create a dummy page for the example app
INSERT INTO pages (id, app_id, name)
VALUES ("1", (SELECT id FROM apps WHERE name = 'Example App'), 'home');
-- create a dummy content for the example page
INSERT INTO content (id, page_id, name, body)
VALUES ("1", (SELECT id FROM pages WHERE name = 'home'), 'home_1', 'Sample Content 1');
INSERT INTO content (id, page_id, name, body)
VALUES ("2", (SELECT id FROM pages WHERE name = 'home'), 'home_2', 'Sample Content 2');
