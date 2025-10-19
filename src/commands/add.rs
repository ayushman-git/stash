use anyhow::Result;

use crate::db::{models::NewArticle, open_connection, queries};

pub fn execute(url: String, tags: Vec<String>) -> Result<()> {
    let conn = open_connection()?;

    let hash: String = blake3::hash(url.as_bytes())
        .to_hex()
        .chars()
        .take(8)
        .collect();

    if let Some(existing) = queries::find_by_hash(&conn, &hash)? {
        println!("Article already stashed with ID: {}", existing.id);
        std::process::exit(3); // 3 for duplicate
    }

    let new_article = NewArticle {
        hash,
        url: url.clone(),
        canonical_url: url.clone(),
        title: None,
        site: None,
        description: None,
        favicon_url: None,
        content_markdown: None,
        tags,
    };

    let id = queries::insert_article(&conn, new_article)?;
    println!("Article saved with ID: {}", id);

    Ok(())
}