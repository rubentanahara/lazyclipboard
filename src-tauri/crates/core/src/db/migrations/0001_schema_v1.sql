CREATE TABLE groups (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE COLLATE NOCASE CHECK (name = trim(name) AND name <> ''),
    position INTEGER NOT NULL,
    never_send_to_ai INTEGER NOT NULL DEFAULT 0 CHECK (never_send_to_ai IN (0, 1)),
    created_at INTEGER NOT NULL
);

CREATE TABLE items (
    id INTEGER PRIMARY KEY,
    group_id INTEGER NOT NULL REFERENCES groups (id) ON DELETE CASCADE,
    kind TEXT NOT NULL CHECK (kind IN ('text', 'rich_text', 'link', 'image')),
    plain_text TEXT,
    html TEXT,
    image_file TEXT,
    image_width INTEGER,
    image_height INTEGER,
    byte_size INTEGER NOT NULL,
    content_hash BLOB NOT NULL,
    source_app TEXT,
    captured_at INTEGER NOT NULL,
    deleted_at INTEGER,
    CHECK (kind = 'image' OR plain_text IS NOT NULL),
    CHECK ((html IS NOT NULL) = (kind = 'rich_text')),
    CHECK ((image_file IS NOT NULL) = (kind = 'image'))
);

CREATE INDEX items_group_captured_at
    ON items (group_id, captured_at DESC)
    WHERE deleted_at IS NULL;

CREATE UNIQUE INDEX items_group_content_hash
    ON items (group_id, content_hash)
    WHERE deleted_at IS NULL;

CREATE TABLE settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    json TEXT NOT NULL
);

CREATE TABLE usage_daily (
    day TEXT NOT NULL,
    metric TEXT NOT NULL,
    bucket_ms INTEGER NOT NULL,
    count INTEGER NOT NULL,
    PRIMARY KEY (day, metric, bucket_ms)
);
