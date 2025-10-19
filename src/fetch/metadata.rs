use anyhow::Result;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub favicon_url: Option<String>,
}

pub fn extract_metadata(html: &str) -> Result<Metadata> {
    let document = Html::parse_document(html);

    let title = extract_title(&document);
    let description = extract_description(&document);
    let favicon_url = extract_link(&document, "rel", "icon");

    let metadata = Metadata {
        title,
        description,
        favicon_url,
    };

    Ok(metadata)
}

fn extract_link(document: &Html, attr: &str, value: &str) -> Option<String> {
    let selector = Selector::parse(&format!(r#"link[{}="{}"]"#, attr, value)).ok()?;

    document
        .select(&selector)
        .next()
        .and_then(|el| el.value().attr("href"))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn extract_title(document: &Html) -> Option<String> {
    if let Some(title) = extract_meta_content(&document, "property", "og:title") {
        return Some(title);
    }

    if let Some(title) = extract_meta_content(&document, "name", "twitter:title") {
        return Some(title);
    }

    let title_selector = Selector::parse("title").ok()?;
    document
        .select(&title_selector)
        .next()
        .map(|el| el.inner_html().trim().to_string())
}

fn extract_description(document: &Html) -> Option<String> {
    if let Some(description) = extract_meta_content(&document, "property", "og:description") {
        return Some(description);
    }

    if let Some(description) = extract_meta_content(&document, "name", "twitter:description") {
        return Some(description);
    }

    extract_meta_content(&document, "name", "description")
}

fn extract_meta_content(document: &Html, attr: &str, value: &str) -> Option<String> {
    let selector = Selector::parse(&format!(r#"meta[{}="{}"]"#, attr, value)).ok()?;

    document
        .select(&selector)
        .next()
        .and_then(|el| el.value().attr("content"))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}
