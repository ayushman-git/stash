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

pub fn insert_article(conn: &Connection, article: NewArticle) -> Result<Article> {
    let now = Utc::now().timestamp();
    let tags_json = serde_json::to_string(&article.tags)?;

    let inserted_article = conn
        .query_row(
            "INSERT INTO articles (
            hash, url, canonical_url, title, site, description, 
            favicon_url, content_markdown, saved_at, tags
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
        RETURNING *",
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
            row_to_article,
        )
        .context("Failed to insert article")?;

    Ok(inserted_article)
}

pub fn find_by_hash(conn: &Connection, hash: &str) -> Result<Option<Article>> {
    let mut stmt = conn.prepare("SELECT * FROM articles where hash = ?1")?;

    let article = stmt
        .query_row(params![hash], row_to_article)
        .optional()
        .context("Failed to query article by hash")?;

    Ok(article)
}

pub fn find_by_ids(conn: &Connection, ids: &[i64]) -> Result<Vec<Article>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");

    let query = format!("SELECT * FROM articles WHERE id IN ({})", placeholders);

    let mut stmt = conn.prepare(&query)?;
    let articles = stmt
        .query_map(params_from_iter(ids), row_to_article)?
        .collect::<rusqlite::Result<Vec<_>>>()
        .context("Failed to query articles by IDs")?;

    Ok(articles)
}

pub fn list_articles(conn: &Connection, limit: i64, all: bool) -> Result<Vec<Article>> {
    let query = if all {
        "SELECT * FROM articles
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

pub fn list_articles_filtered(
    conn: &Connection,
    limit: i64,
    all: bool,
    archived: bool,
    starred: bool,
    tags: &[String],
    sort_field: &str,
    reverse: bool,
) -> Result<Vec<Article>> {
    let mut conditions = Vec::new();
    
    if !all {
        conditions.push("read = 0".to_string());
        if !archived {
            conditions.push("archived = 0".to_string());
        }
    }
    
    if archived {
        conditions.push("archived = 1".to_string());
    }
    
    if starred {
        conditions.push("starred = 1".to_string());
    }
    
    // Filter by tags - article must contain ALL specified tags
    if !tags.is_empty() {
        for tag in tags {
            // Escape single quotes in tags to prevent SQL injection
            let escaped_tag = tag.replace('\'', "''");
            conditions.push(format!(
                "EXISTS (SELECT 1 FROM json_each(tags) WHERE value = '{}')",
                escaped_tag
            ));
        }
    }
    
    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };
    
    // Map sort field to database column and determine sort order
    let sort_column = match sort_field {
        "time" => "saved_at",
        "title" => "title COLLATE NOCASE",  // Case-insensitive sort
        "site" => "site COLLATE NOCASE",    // Case-insensitive sort
        "read" => "read",
        "star" => "starred",
        _ => "saved_at", // default fallback
    };
    
    // For text fields (title, site), default to ASC (A-Z)
    // For other fields (time, read, star), default to DESC (newest/true first)
    let default_order = match sort_field {
        "title" | "site" => "ASC",
        _ => "DESC",
    };
    
    let sort_order = if reverse {
        if default_order == "ASC" { "DESC" } else { "ASC" }
    } else {
        default_order
    };
    
    let query = format!(
        "SELECT * FROM articles {} ORDER BY {} {} LIMIT ?1",
        where_clause, sort_column, sort_order
    );
    
    let mut stmt = conn.prepare(&query)?;
    let articles = stmt
        .query_map(params![limit], row_to_article)?
        .collect::<rusqlite::Result<Vec<_>>>()
        .context("Failed to list articles")?;
    
    Ok(articles)
}

pub fn get_random_articles(conn: &Connection, count: i64, all: bool) -> Result<Vec<Article>> {
    let query = if all {
        "SELECT * FROM articles ORDER BY RANDOM() LIMIT ?1"
    } else {
        "SELECT * FROM articles WHERE read = 0 AND archived = 0 
         ORDER BY RANDOM() LIMIT ?1"
    };

    let mut stmt = conn.prepare(query)?;
    let articles = stmt
        .query_map(params![count], row_to_article)?
        .collect::<rusqlite::Result<Vec<_>>>()
        .context("Failed to fetch random articles")?;

    Ok(articles)
}

pub fn list_archived_articles(conn: &Connection, limit: i64) -> Result<Vec<Article>> {
    let query =
        "SELECT * FROM articles WHERE archived = 1 ORDER BY starred DESC, saved_at DESC LIMIT ?1";

    let mut stmt = conn.prepare(query)?;
    let articles = stmt
        .query_map(params![limit], row_to_article)?
        .collect::<rusqlite::Result<Vec<_>>>()
        .context("Failed to list archived articles")?;

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

pub fn set_starred_by_ids(conn: &Connection, ids: &[i64], starred: bool) -> Result<Vec<Article>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");

    let query = format!(
        "UPDATE articles SET starred = {} WHERE id IN ({})",
        if starred { 1 } else { 0 },
        placeholders
    );

    let affected = conn
        .execute(&query, params_from_iter(ids))
        .context("Failed to update starred status")?;

    if affected > 0 {
        find_by_ids(conn, ids)
    } else {
        Ok(Vec::new())
    }
}

pub fn mark_read_by_ids(conn: &Connection, ids: &[i64]) -> Result<Vec<Article>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");

    let query = format!(
        "UPDATE articles SET read = 1 WHERE id IN ({})",
        placeholders
    );

    let affected = conn
        .execute(&query, params_from_iter(ids))
        .context("Failed to mark articles as read")?;

    if affected > 0 {
        find_by_ids(conn, ids)
    } else {
        Ok(Vec::new())
    }
}

pub fn set_read_by_ids(conn: &Connection, ids: &[i64], read: bool) -> Result<Vec<Article>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");

    let query = format!(
        "UPDATE articles SET read = {} WHERE id IN ({})",
        if read { 1 } else { 0 },
        placeholders
    );

    let affected = conn
        .execute(&query, params_from_iter(ids))
        .context("Failed to update read status")?;

    if affected > 0 {
        find_by_ids(conn, ids)
    } else {
        Ok(Vec::new())
    }
}

pub fn set_read_all(conn: &Connection, read: bool, include_archived: bool) -> Result<Vec<Article>> {
    // First, get IDs of articles to update
    let id_query = if include_archived {
        "SELECT id FROM articles"
    } else {
        "SELECT id FROM articles WHERE archived = 0"
    };

    let mut stmt = conn.prepare(id_query)?;
    let ids: Vec<i64> = stmt
        .query_map([], |row| row.get(0))?
        .collect::<rusqlite::Result<Vec<_>>>()
        .context("Failed to fetch article IDs")?;

    if ids.is_empty() {
        return Ok(Vec::new());
    }

    set_read_by_ids(conn, &ids, read)
}

pub fn get_article_by_id(conn: &Connection, id: i64) -> Result<Option<Article>> {
    let mut stmt = conn.prepare("SELECT * FROM articles WHERE id = ?1")?;

    let article = stmt
        .query_row(params![id], row_to_article)
        .optional()
        .context("Failed to query article by ID")?;

    Ok(article)
}

pub fn update_tags(conn: &Connection, id: i64, tags: Vec<String>) -> Result<Article> {
    let tags_json = serde_json::to_string(&tags)?;

    conn.execute(
        "UPDATE articles SET tags = ?1 WHERE id = ?2",
        params![tags_json, id],
    )
    .context("Failed to update article tags")?;

    let article = get_article_by_id(conn, id)?
        .context("Article not found after update")?;

    Ok(article)
}

pub fn get_all_tags_with_counts(conn: &Connection) -> Result<Vec<(String, usize)>> {
    use std::collections::HashMap;
    
    // Get all articles
    let mut stmt = conn.prepare("SELECT tags FROM articles")?;
    let tags_list = stmt
        .query_map([], |row| {
            let tags_json: String = row.get(0)?;
            Ok(tags_json)
        })?
        .collect::<rusqlite::Result<Vec<_>>>()
        .context("Failed to query tags")?;
    
    // Count occurrences of each tag
    let mut tag_counts: HashMap<String, usize> = HashMap::new();
    
    for tags_json in tags_list {
        let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
        for tag in tags {
            *tag_counts.entry(tag).or_insert(0) += 1;
        }
    }
    
    // Convert to vector and sort alphabetically
    let mut result: Vec<(String, usize)> = tag_counts.into_iter().collect();
    result.sort_by(|a, b| a.0.cmp(&b.0));
    
    Ok(result)
}
