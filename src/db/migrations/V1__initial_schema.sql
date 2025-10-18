-- Initial schema for articles table
CREATE TABLE IF NOT EXISTS articles (
    -- User-facing incremental ID (1, 2, 3...)
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    
    -- Git-style short hash (8 chars) from blake3(canonical_url)
    hash TEXT NOT NULL UNIQUE,
    
    -- Original URL as provided by user
    url TEXT NOT NULL,
    
    -- Cleaned, deduplicated URL (after canonicalization)
    canonical_url TEXT NOT NULL UNIQUE,
    
    -- Article metadata
    title TEXT,
    site TEXT,  -- Domain name (e.g., "github.com")
    description TEXT,
    favicon_url TEXT,
    
    -- Cached content (Markdown format after readability extraction)
    content_markdown TEXT,
    
    -- Timestamps
    saved_at INTEGER NOT NULL,  -- Unix timestamp
    last_opened_at INTEGER,     -- Unix timestamp
    
    -- State flags (booleans as INTEGER: 0=false, 1=true)
    read INTEGER NOT NULL DEFAULT 0,
    archived INTEGER NOT NULL DEFAULT 0,
    starred INTEGER NOT NULL DEFAULT 0,
    
    -- User data
    note TEXT,
    tags TEXT NOT NULL DEFAULT '[]'  -- JSON array: ["rust", "cli"]
);

-- Indexes for common queries
CREATE INDEX IF NOT EXISTS idx_articles_archived ON articles(archived);
CREATE INDEX IF NOT EXISTS idx_articles_read ON articles(read);
CREATE INDEX IF NOT EXISTS idx_articles_starred ON articles(starred);
CREATE INDEX IF NOT EXISTS idx_articles_saved_at ON articles(saved_at DESC);
CREATE INDEX IF NOT EXISTS idx_articles_hash ON articles(hash);
CREATE INDEX IF NOT EXISTS idx_articles_site ON articles(site);

-- Full-text search virtual table (for `search` command)
CREATE VIRTUAL TABLE IF NOT EXISTS articles_fts USING fts5(
    title,
    description,
    content_markdown,
    tags,
    content=articles,
    content_rowid=id
);

-- Triggers to keep FTS index in sync
CREATE TRIGGER IF NOT EXISTS articles_fts_insert AFTER INSERT ON articles BEGIN
    INSERT INTO articles_fts(rowid, title, description, content_markdown, tags)
    VALUES (new.id, new.title, new.description, new.content_markdown, new.tags);
END;

CREATE TRIGGER IF NOT EXISTS articles_fts_update AFTER UPDATE ON articles BEGIN
    UPDATE articles_fts 
    SET title = new.title,
        description = new.description,
        content_markdown = new.content_markdown,
        tags = new.tags
    WHERE rowid = new.id;
END;

CREATE TRIGGER IF NOT EXISTS articles_fts_delete AFTER DELETE ON articles BEGIN
    DELETE FROM articles_fts WHERE rowid = old.id;
END;