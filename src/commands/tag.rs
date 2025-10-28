use anyhow::Result;
use std::collections::HashSet;

use crate::{
    db::{open_connection, queries::{get_article_by_id, update_tags}},
    ui::list::{OutputFormat, render_articles},
};

/// Validates that a tag is lowercase, alphanumeric, and may contain hyphens
/// Valid examples: "rust", "rust-lang", "web-dev-101"
/// Invalid: "Rust", "rust_lang", "rust--lang", "-rust", "rust-"
fn validate_tag(tag: &str) -> bool {
    if tag.is_empty() {
        return false;
    }
    
    // Must not start or end with hyphen
    if tag.starts_with('-') || tag.ends_with('-') {
        return false;
    }
    
    let mut prev_was_hyphen = false;
    for c in tag.chars() {
        if c == '-' {
            // No consecutive hyphens
            if prev_was_hyphen {
                return false;
            }
            prev_was_hyphen = true;
        } else if c.is_ascii_lowercase() || c.is_ascii_digit() {
            prev_was_hyphen = false;
        } else {
            // Invalid character (uppercase, underscore, special chars, etc.)
            return false;
        }
    }
    
    true
}

pub fn execute(id: &i64, operations: &[String]) -> Result<()> {
    let conn = open_connection()?;
    
    // Fetch the article
    let article = match get_article_by_id(&conn, *id)? {
        Some(article) => article,
        None => {
            eprintln!("Article with ID {} not found", id);
            std::process::exit(1);
        }
    };
    
    // If no operations provided, just list current tags
    if operations.is_empty() {
        if article.tags.is_empty() {
            println!("No tags for article {}", id);
        } else {
            println!("Tags for article {}: {}", id, article.tags.join(", "));
        }
        return Ok(());
    }
    
    // Parse operations and build new tag set
    let mut tags: HashSet<String> = article.tags.iter().cloned().collect();
    let mut has_operations = false;
    
    for op in operations {
        if let Some(tag) = op.strip_prefix('+') {
            // Add tag
            let tag = tag.to_lowercase();
            if !validate_tag(&tag) {
                eprintln!("Invalid tag format: '{}'. Tags must be lowercase alphanumeric with hyphens only.", tag);
                std::process::exit(2);
            }
            tags.insert(tag);
            has_operations = true;
        } else if let Some(tag) = op.strip_prefix('-') {
            // Remove tag
            let tag = tag.to_lowercase();
            tags.remove(&tag);
            has_operations = true;
        } else {
            eprintln!("Invalid operation: '{}'. Use +tag to add or -tag to remove.", op);
            std::process::exit(2);
        }
    }
    
    // If no valid operations were found, just display current tags
    if !has_operations {
        if article.tags.is_empty() {
            println!("No tags for article {}", id);
        } else {
            println!("Tags for article {}: {}", id, article.tags.join(", "));
        }
        return Ok(());
    }
    
    // Convert back to sorted vector
    let mut tags_vec: Vec<String> = tags.into_iter().collect();
    tags_vec.sort();
    
    // Update in database
    let updated_article = update_tags(&conn, *id, tags_vec)?;
    
    // Display updated article
    render_articles(&vec![updated_article], OutputFormat::Table, false, false)?;
    
    Ok(())
}

