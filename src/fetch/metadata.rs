use anyhow::Result;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// Pre-compiled CSS selectors for metadata extraction.
/// Using OnceLock ensures selectors are parsed once and reused across calls.
struct MetadataSelectors {
    og_title: Selector,
    twitter_title: Selector,
    title: Selector,
    og_description: Selector,
    twitter_description: Selector,
    description: Selector,
    favicon: Selector,
}

impl MetadataSelectors {
    fn new() -> Self {
        Self {
            og_title: Selector::parse(r#"meta[property="og:title"]"#)
                .expect("Failed to parse og:title selector"),
            twitter_title: Selector::parse(r#"meta[name="twitter:title"]"#)
                .expect("Failed to parse twitter:title selector"),
            title: Selector::parse("title")
                .expect("Failed to parse title selector"),
            og_description: Selector::parse(r#"meta[property="og:description"]"#)
                .expect("Failed to parse og:description selector"),
            twitter_description: Selector::parse(r#"meta[name="twitter:description"]"#)
                .expect("Failed to parse twitter:description selector"),
            description: Selector::parse(r#"meta[name="description"]"#)
                .expect("Failed to parse description selector"),
            favicon: Selector::parse(r#"link[rel="icon"]"#)
                .expect("Failed to parse favicon selector"),
        }
    }
}

/// Global singleton for pre-compiled selectors
fn selectors() -> &'static MetadataSelectors {
    static SELECTORS: OnceLock<MetadataSelectors> = OnceLock::new();
    SELECTORS.get_or_init(MetadataSelectors::new)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub favicon_url: Option<String>,
}

pub fn extract_metadata(html: &str) -> Result<Metadata> {
    let document = Html::parse_document(html);
    let selectors = selectors();

    let title = extract_title(&document, selectors);
    let description = extract_description(&document, selectors);
    let favicon_url = extract_favicon(&document, selectors);

    let metadata = Metadata {
        title,
        description,
        favicon_url,
    };

    Ok(metadata)
}

fn extract_favicon(document: &Html, selectors: &MetadataSelectors) -> Option<String> {
    document
        .select(&selectors.favicon)
        .next()
        .and_then(|el| el.value().attr("href"))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn extract_title(document: &Html, selectors: &MetadataSelectors) -> Option<String> {
    // Try og:title first
    if let Some(title) = extract_content(document, &selectors.og_title) {
        return Some(title);
    }

    // Try twitter:title next
    if let Some(title) = extract_content(document, &selectors.twitter_title) {
        return Some(title);
    }

    // Fall back to <title> tag
    document
        .select(&selectors.title)
        .next()
        .map(|el| el.inner_html().trim().to_string())
}

fn extract_description(document: &Html, selectors: &MetadataSelectors) -> Option<String> {
    // Try og:description first
    if let Some(description) = extract_content(document, &selectors.og_description) {
        return Some(description);
    }

    // Try twitter:description next
    if let Some(description) = extract_content(document, &selectors.twitter_description) {
        return Some(description);
    }

    // Fall back to meta name="description"
    extract_content(document, &selectors.description)
}

/// Helper to extract content attribute from a meta tag using a pre-compiled selector
fn extract_content(document: &Html, selector: &Selector) -> Option<String> {
    document
        .select(selector)
        .next()
        .and_then(|el| el.value().attr("content"))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}
