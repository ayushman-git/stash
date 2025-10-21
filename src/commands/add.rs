use anyhow::{Context, Result};

use crate::{
    db::{models::NewArticle, open_connection, queries},
    fetch::{
        content::convert_html_to_md,
        http::{extract_site, fetch_html},
        metadata::extract_metadata,
    },
};

pub fn execute(url: String, tags: Vec<String>, title_by_user: String) -> Result<()> {
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

    let (title, description, favicon_url, content_markdown) = match fetch_html(&url) {
        Ok(html) => {
            let meta = extract_metadata(&html).ok();
            let title = if !title_by_user.is_empty() {
                Some(title_by_user.clone())
            } else {
                meta.as_ref().and_then(|m| m.title.clone())
            };
            let description = meta.as_ref().and_then(|m| m.description.clone());
            let favicon_url = meta.as_ref().and_then(|m| m.favicon_url.clone());

            let content_markdown = convert_html_to_md(&html);

            (title, description, favicon_url, content_markdown)
        }
        Err(e) => {
            eprintln!("Warning: Failed to fetch content: {}", e);
            eprintln!("Saving URL only...");

            let domain = extract_site(&url);
            let fallback_title = if !title_by_user.is_empty() {
                title_by_user
            } else {
                domain.clone().unwrap_or_else(|| "Untitled".to_string())
            };
            (Some(fallback_title), None, None, None)
        }
    };

    let new_article = NewArticle {
        hash,
        url: url.clone(),
        canonical_url: url.clone(),
        title,
        description,
        favicon_url,
        site: extract_site(&url),
        content_markdown,
        tags,
    };

    let id = queries::insert_article(&conn, new_article)
        .context("Failed to save article to database")?;

    println!("Article saved with ID: {}", id);

    Ok(())
}
