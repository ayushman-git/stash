use crate::db::models::{Article, NewArticle};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension, Row, params, params_from_iter};

pub fn row_to_article(row: &Row) -> rusqlite::Result<Article> {
    let saved_at_unix: i64 = row.get("saved_at")?;
    let last_opened_unix: Option<i64> = row.get("last_opened_at")?;

    let tags_json: String = row.get("tags")?;

    Ok(Article {
        id: row.get("id")?,
        hash: row.get("hash")?,
        url: row.get("url")?,
        canonical_url: row.get("canonical_url")?,
        title: row.get("title")?,
        site: row.get("site")?,
        description: row.get("description")?,
        favicon_url: row.get("favicon_url")?,
        content_markdown: row.get("content_markdown")?,
        saved_at: DateTime::from_timestamp(saved_at_unix, 0).unwrap_or_else(|| Utc::now()),
        last_opened_at: last_opened_unix.and_then(|ts| DateTime::from_timestamp(ts, 0)),
        read: row.get::<_, i64>("read")? != 0,
        archived: row.get::<_, i64>("archived")? != 0,
        starred: row.get::<_, i64>("starred")? != 0,
        note: row.get("note")?,
        tags: serde_json::from_str(&tags_json).unwrap_or_default(),
    })
}

pub fn insert_article(conn: &Connection, article: NewArticle) -> Result<i64> {
    let now = Utc::now().timestamp();
    let tags_json = serde_json::to_string(&article.tags)?;

    conn.execute(
        "INSERT INTO articles (
            hash, url, canonical_url, title, site, description, 
            favicon_url, content_markdown, saved_at, tags
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            article.hash,
            article.url,
            article.canonical_url,
            article.title,
            article.site,
            article.description,
            article.favicon_url,
            article.content_markdown,
            now,
            tags_json,
        ],
    )
    .context("Failed to insert article")?;

    Ok(conn.last_insert_rowid())
}

pub fn find_by_hash(conn: &Connection, hash: &str) -> Result<Option<Article>> {
    let mut stmt = conn.prepare("SELECT * FROM articles where hash = ?1")?;

    let article = stmt
        .query_row(params![hash], row_to_article)
        .optional()
        .context("Failed to query article by hash")?;

    Ok(article)
}

pub fn list_articles(conn: &Connection, limit: usize, archived: bool) -> Result<Vec<Article>> {
    let query = if archived {
        "SELECT * FROM articles WHERE archived = 1
         ORDER BY starred DESC, saved_at DESC LIMIT ?1"
    } else {
        "SELECT * FROM articles WHERE read = 0 AND archived = 0
         ORDER BY starred DESC, saved_at DESC LIMIT ?1"
    };

    let mut stmt = conn.prepare(query)?;
    let articles = stmt
        .query_map(params![limit], row_to_article)?
        .collect::<rusqlite::Result<Vec<_>>>()
        .context("Failed to list articles")?;

    Ok(articles)
}

pub fn archive_by_ids(conn: &Connection, ids: &[i64]) -> Result<usize> {
    if ids.is_empty() {
        return Ok(0);
    }

    // Generate placeholders: "?, ?, ?"
    let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");

    let query = format!(
        "UPDATE articles SET archived = 1 WHERE id IN ({})",
        placeholders
    );

    // Execute with dynamic parameter binding
    let affected = conn
        .execute(&query, params_from_iter(ids))
        .context("Failed to archive articles")?;

    Ok(affected)
}

pub fn delete_by_ids(conn: &Connection, ids: &[i64]) -> Result<usize> {
    if ids.is_empty() {
        return Ok(0);
    }

    let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");

    let query = format!("DELETE FROM articles WHERE id IN ({})", placeholders);

    let affected = conn
        .execute(&query, params_from_iter(ids))
        .context("Failed to delete articles")?;

    Ok(affected)
}
