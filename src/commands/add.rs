use anyhow::Result;

use crate::{db::{models::NewArticle, open_connection, queries}, fetch::{http::{extract_site, fetch_html}, metadata::{extract_metadata, Metadata}}};

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

    let html = fetch_html(&url)?;
    let meta: Metadata = extract_metadata(&html)?;

    let new_article = NewArticle {
        hash,
        url: url.clone(),
        canonical_url: url.clone(),
        title: meta.title,
        description: meta.description,
        favicon_url: meta.favicon_url,
        site: extract_site(&url),
        content_markdown: None,
        tags,
    };

    let id = queries::insert_article(&conn, new_article)?;
    println!("Article saved with ID: {}", id);

    Ok(())
}