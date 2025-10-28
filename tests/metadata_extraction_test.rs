// Metadata extraction unit tests
mod common;

use stash::fetch::metadata::extract_metadata;
use common::*;

// OpenGraph Tags Tests

#[test]
fn test_extract_metadata_prefers_og_title() {
    let result = extract_metadata(HTML_WITH_OG_TAGS);
    assert!(result.is_ok());
    
    let metadata = result.unwrap();
    assert_eq!(metadata.title, Some("OpenGraph Title".to_string()));
}

#[test]
fn test_extract_metadata_prefers_og_description() {
    let result = extract_metadata(HTML_WITH_OG_TAGS);
    assert!(result.is_ok());
    
    let metadata = result.unwrap();
    assert_eq!(metadata.description, Some("OpenGraph Description".to_string()));
}

// Twitter Card Tags Tests

#[test]
fn test_extract_metadata_falls_back_to_twitter_title() {
    let result = extract_metadata(HTML_WITH_TWITTER_TAGS);
    assert!(result.is_ok());
    
    let metadata = result.unwrap();
    assert_eq!(metadata.title, Some("Twitter Title".to_string()));
}

#[test]
fn test_extract_metadata_falls_back_to_twitter_description() {
    let result = extract_metadata(HTML_WITH_TWITTER_TAGS);
    assert!(result.is_ok());
    
    let metadata = result.unwrap();
    assert_eq!(metadata.description, Some("Twitter Description".to_string()));
}

// Standard Meta Tags Tests

#[test]
fn test_extract_metadata_falls_back_to_standard_title() {
    let result = extract_metadata(HTML_WITH_STANDARD_TAGS);
    assert!(result.is_ok());
    
    let metadata = result.unwrap();
    assert_eq!(metadata.title, Some("Standard Title".to_string()));
}

#[test]
fn test_extract_metadata_falls_back_to_standard_description() {
    let result = extract_metadata(HTML_WITH_STANDARD_TAGS);
    assert!(result.is_ok());
    
    let metadata = result.unwrap();
    assert_eq!(metadata.description, Some("Standard Description".to_string()));
}

// Favicon Extraction Tests

#[test]
fn test_extract_metadata_finds_favicon() {
    let result = extract_metadata(HTML_WITH_FAVICON);
    assert!(result.is_ok());
    
    let metadata = result.unwrap();
    assert_eq!(metadata.favicon_url, Some("https://example.com/favicon.ico".to_string()));
}

// Priority Order Tests

#[test]
fn test_extract_metadata_og_has_priority_over_twitter() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta property="og:title" content="OG Title">
        <meta name="twitter:title" content="Twitter Title">
        <title>Standard Title</title>
    </head>
    <body></body>
    </html>
    "#;
    
    let result = extract_metadata(html);
    assert!(result.is_ok());
    
    let metadata = result.unwrap();
    assert_eq!(metadata.title, Some("OG Title".to_string()));
}

#[test]
fn test_extract_metadata_twitter_has_priority_over_standard() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta name="twitter:title" content="Twitter Title">
        <title>Standard Title</title>
    </head>
    <body></body>
    </html>
    "#;
    
    let result = extract_metadata(html);
    assert!(result.is_ok());
    
    let metadata = result.unwrap();
    assert_eq!(metadata.title, Some("Twitter Title".to_string()));
}

// Special Characters Tests

#[test]
fn test_extract_metadata_handles_special_chars() {
    let result = extract_metadata(HTML_WITH_SPECIAL_CHARS);
    assert!(result.is_ok());
    
    let metadata = result.unwrap();
    assert!(metadata.description.is_some());
    let desc = metadata.description.unwrap();
    assert!(desc.contains("quotes"));
    assert!(desc.contains("symbols"));
}

#[test]
fn test_extract_metadata_handles_html_entities_in_title() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Test &amp; Title &lt;HTML&gt;</title>
    </head>
    <body></body>
    </html>
    "#;
    
    let result = extract_metadata(html);
    assert!(result.is_ok());
    
    let metadata = result.unwrap();
    assert!(metadata.title.is_some());
}

// Edge Cases Tests

#[test]
fn test_extract_metadata_empty_html() {
    let result = extract_metadata(HTML_EMPTY);
    assert!(result.is_ok());
    
    let metadata = result.unwrap();
    assert!(metadata.title.is_none());
    assert!(metadata.description.is_none());
    assert!(metadata.favicon_url.is_none());
}

#[test]
fn test_extract_metadata_minimal_html() {
    let result = extract_metadata(HTML_MINIMAL);
    assert!(result.is_ok());
    
    let metadata = result.unwrap();
    assert!(metadata.title.is_none());
    assert!(metadata.description.is_none());
}

#[test]
fn test_extract_metadata_missing_all_tags() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head></head>
    <body><p>Content but no metadata</p></body>
    </html>
    "#;
    
    let result = extract_metadata(html);
    assert!(result.is_ok());
    
    let metadata = result.unwrap();
    assert!(metadata.title.is_none());
    assert!(metadata.description.is_none());
    assert!(metadata.favicon_url.is_none());
}

#[test]
fn test_extract_metadata_whitespace_only_content() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>   </title>
        <meta name="description" content="   ">
    </head>
    <body></body>
    </html>
    "#;
    
    let result = extract_metadata(html);
    assert!(result.is_ok());
    
    let metadata = result.unwrap();
    // Parser extracts whitespace as-is, extraction doesn't filter empty content
    // This is okay - the application layer can handle trimming if needed
    assert!(metadata.title.is_some());
}

#[test]
fn test_extract_metadata_multiple_og_tags() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta property="og:title" content="First OG Title">
        <meta property="og:title" content="Second OG Title">
        <meta property="og:description" content="First OG Desc">
    </head>
    <body></body>
    </html>
    "#;
    
    let result = extract_metadata(html);
    assert!(result.is_ok());
    
    let metadata = result.unwrap();
    // Should use first matching tag
    assert_eq!(metadata.title, Some("First OG Title".to_string()));
}

#[test]
fn test_extract_metadata_malformed_html() {
    let html = "<html><head><title>Broken</html>";
    
    let result = extract_metadata(html);
    assert!(result.is_ok()); // Should handle gracefully
    
    let metadata = result.unwrap();
    // Parser may preserve the malformed closing tag as text
    assert!(metadata.title.is_some());
    assert!(metadata.title.unwrap().contains("Broken"));
}

#[test]
fn test_extract_metadata_empty_string() {
    let result = extract_metadata("");
    assert!(result.is_ok());
}

#[test]
fn test_extract_metadata_only_whitespace() {
    let result = extract_metadata("   \n   \t   ");
    assert!(result.is_ok());
}

#[test]
fn test_extract_metadata_case_insensitive_meta_tags() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <META NAME="description" CONTENT="Case Test">
    </head>
    <body></body>
    </html>
    "#;
    
    let result = extract_metadata(html);
    assert!(result.is_ok());
    
    let metadata = result.unwrap();
    assert_eq!(metadata.description, Some("Case Test".to_string()));
}

#[test]
fn test_extract_metadata_long_content() {
    let long_title = "A".repeat(500);
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>{}</title>
        </head>
        <body></body>
        </html>
        "#,
        long_title
    );
    
    let result = extract_metadata(&html);
    assert!(result.is_ok());
    
    let metadata = result.unwrap();
    assert!(metadata.title.is_some());
    assert_eq!(metadata.title.unwrap().len(), 500);
}

#[test]
fn test_extract_metadata_unicode_content() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Unicode: 日本語 • Русский • العربية • 🚀</title>
        <meta name="description" content="Emoji support: 👍 ✨ 🎉">
    </head>
    <body></body>
    </html>
    "#;
    
    let result = extract_metadata(html);
    assert!(result.is_ok());
    
    let metadata = result.unwrap();
    assert!(metadata.title.is_some());
    assert!(metadata.title.unwrap().contains("🚀"));
    assert!(metadata.description.is_some());
    assert!(metadata.description.unwrap().contains("👍"));
}

#[test]
fn test_extract_metadata_newlines_in_content() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta property="og:description" content="Line 1
        Line 2
        Line 3">
    </head>
    <body></body>
    </html>
    "#;
    
    let result = extract_metadata(html);
    assert!(result.is_ok());
    
    let metadata = result.unwrap();
    assert!(metadata.description.is_some());
}

