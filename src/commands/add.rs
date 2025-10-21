use anyhow::{Context, Result};
use dialoguer::Input;

use crate::{
    db::{models::NewArticle, open_connection, queries},
    fetch::{
        content::convert_html_to_md,
        http::{extract_site, fetch_html},
        metadata::extract_metadata,
    },
    ui::list::{OutputFormat, render_articles},
};

pub fn execute(
    url: Option<String>,
    tags: Vec<String>,
    title_by_user: Option<String>,
    no_fetch: bool,
) -> Result<()> {
    let (url, tags) = match url {
        Some(u) => (u, tags),
        None => {
            let url = Input::new()
                .with_prompt("URL")
                .interact_text()
                .context("Failed to read URL input")?;

            let tags_input: String = Input::new()
                .with_prompt("Tags (comma-separated, optional)")
                .allow_empty(true)
                .interact_text()
                .context("Failed to read the tags")?;

            let tags = if tags_input.is_empty() {
                Vec::new()
            } else {
                tags_input
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect()
            };

            (url, tags)
        }
    };

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

    let html_opt = if no_fetch {
        None
    } else {
        fetch_html(&url).ok()
    };

    let (title, description, favicon_url, content_markdown) = match html_opt {
        Some(html) => {
            let meta = extract_metadata(&html).ok();
            let title = title_by_user.or_else(|| meta.as_ref().and_then(|m| m.title.clone()));
            let description = meta.as_ref().and_then(|m| m.description.clone());
            let favicon_url = meta.as_ref().and_then(|m| m.favicon_url.clone());

            let content_markdown = convert_html_to_md(&html);

            (title, description, favicon_url, content_markdown)
        }
        None => {
            eprintln!("Saving URL only...");

            let domain = extract_site(&url);
            let fallback_title = title_by_user
                .or_else(|| domain.clone().map(|d| d).or(Some("Untitled".to_string())));
            (fallback_title, None, None, None)
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

    let article = queries::insert_article(&conn, new_article)
        .context("Failed to save article to database")?;

    render_articles(&vec![article], OutputFormat::Table, false, false)?;
    Ok(())
}
