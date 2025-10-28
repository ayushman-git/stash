// Common test utilities and fixtures
#![allow(dead_code)] // Test helpers are used across different test files

use rusqlite::Connection;
use stash::db::models::{Article, NewArticle};
use chrono::Utc;

/// Creates an in-memory SQLite database with migrations applied
pub fn setup_test_db() -> Connection {
    let mut conn = Connection::open(":memory:").expect("Failed to create in-memory database");
    
    // Embed and run migrations
    mod embedded {
        use refinery::embed_migrations;
        embed_migrations!("src/db/migrations");
    }
    
    embedded::migrations::runner()
        .run(&mut conn)
        .expect("Failed to run migrations");
    
    // Drop FTS triggers and table for testing since we're not testing search functionality
    // This prevents FTS-related errors in unit tests
    conn.execute_batch(r#"
        DROP TRIGGER IF EXISTS articles_fts_insert;
        DROP TRIGGER IF EXISTS articles_fts_update;
        DROP TRIGGER IF EXISTS articles_fts_delete;
        DROP TABLE IF EXISTS articles_fts;
    "#).expect("Failed to drop FTS table and triggers");
    
    conn
}

/// Builder for creating test Article instances
pub fn create_test_article(
    id: i64,
    hash: &str,
    url: &str,
    title: Option<&str>,
    tags: Vec<&str>,
) -> Article {
    Article {
        id,
        hash: hash.to_string(),
        url: url.to_string(),
        canonical_url: url.to_string(),
        title: title.map(|s| s.to_string()),
        site: Some("example.com".to_string()),
        description: None,
        favicon_url: None,
        content_markdown: None,
        saved_at: Utc::now(),
        last_opened_at: None,
        read: false,
        archived: false,
        starred: false,
        note: None,
        tags: tags.iter().map(|s| s.to_string()).collect(),
    }
}

/// Builder for creating test NewArticle instances
pub fn create_new_article(
    hash: &str,
    url: &str,
    title: Option<&str>,
    tags: Vec<&str>,
) -> NewArticle {
    NewArticle {
        hash: hash.to_string(),
        url: url.to_string(),
        canonical_url: url.to_string(),
        title: title.map(|s| s.to_string()),
        site: Some("example.com".to_string()),
        description: None,
        favicon_url: None,
        content_markdown: None,
        tags: tags.iter().map(|s| s.to_string()).collect(),
    }
}

// HTML test fixtures for metadata extraction tests

pub const HTML_WITH_OG_TAGS: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Fallback Title</title>
    <meta property="og:title" content="OpenGraph Title">
    <meta property="og:description" content="OpenGraph Description">
    <meta name="description" content="Standard Description">
</head>
<body></body>
</html>
"#;

pub const HTML_WITH_TWITTER_TAGS: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Fallback Title</title>
    <meta name="twitter:title" content="Twitter Title">
    <meta name="twitter:description" content="Twitter Description">
    <meta name="description" content="Standard Description">
</head>
<body></body>
</html>
"#;

pub const HTML_WITH_STANDARD_TAGS: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Standard Title</title>
    <meta name="description" content="Standard Description">
</head>
<body></body>
</html>
"#;

pub const HTML_WITH_FAVICON: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Page Title</title>
    <link rel="icon" href="https://example.com/favicon.ico">
</head>
<body></body>
</html>
"#;

pub const HTML_WITH_SPECIAL_CHARS: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Title with &amp; Special &lt;chars&gt;</title>
    <meta property="og:description" content="Description with 'quotes' &amp; symbols">
</head>
<body></body>
</html>
"#;

pub const HTML_EMPTY: &str = r#"
<!DOCTYPE html>
<html>
<head></head>
<body></body>
</html>
"#;

pub const HTML_MINIMAL: &str = "<html><head></head><body></body></html>";

