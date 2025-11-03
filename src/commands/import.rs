use anyhow::Result;
use colored::Colorize;
use std::path::PathBuf;

use crate::db::{models::NewArticle, open_connection, queries};
use crate::export::json;

pub fn execute(
    path: String,
    merge: bool,
    dry_run: bool,
) -> Result<()> {
    let import_path = PathBuf::from(&path);
    
    if !import_path.exists() {
        anyhow::bail!("File or directory not found: {}", path);
    }
    
    // Import articles from JSON
    let articles = if import_path.is_file() {
        json::import_from_json(&import_path)?
    } else {
        anyhow::bail!("Directory import not yet implemented. Please provide a JSON file.");
    };
    
    if articles.is_empty() {
        println!("No articles found to import");
        return Ok(());
    }
    
    println!("Found {} article(s) to import", articles.len());
    
    if dry_run {
        println!("\n{} (dry run - no changes will be made)", "ℹ".cyan().bold());
        for article in &articles {
            println!(
                "  {} - {}",
                article.id,
                article.title.as_deref().unwrap_or("<no title>")
            );
        }
        return Ok(());
    }
    
    let conn = open_connection()?;
    
    let mut imported = 0;
    let mut skipped = 0;
    let mut errors = 0;
    
    for article in articles {
        // Check if article already exists by hash
        let existing = queries::find_by_hash(&conn, &article.hash)?;
        
        if existing.is_some() {
            if !merge {
                println!("  {} Skipping duplicate: {}", "⊘".yellow(), article.hash);
                skipped += 1;
                continue;
            }
            // For merge mode, we skip duplicates (could update instead in future)
            println!("  {} Skipping existing: {}", "⊘".yellow(), article.hash);
            skipped += 1;
            continue;
        }
        
        // Insert the article
        let new_article = NewArticle {
            hash: article.hash,
            url: article.url,
            canonical_url: article.canonical_url,
            title: article.title,
            site: article.site,
            description: article.description,
            favicon_url: article.favicon_url,
            content_markdown: article.content_markdown,
            tags: article.tags,
        };
        
        match queries::insert_article(&conn, new_article) {
            Ok(inserted) => {
                println!(
                    "  {} Imported: {}",
                    "✓".green(),
                    inserted.title.as_deref().unwrap_or("<no title>")
                );
                imported += 1;
            }
            Err(e) => {
                eprintln!(
                    "  {} Failed to import article: {}",
                    "✗".red(),
                    e
                );
                errors += 1;
            }
        }
    }
    
    println!(
        "\n{} Import complete: {} imported, {} skipped, {} errors",
        "✓".green().bold(),
        imported,
        skipped,
        errors
    );
    
    Ok(())
}

