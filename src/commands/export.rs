use anyhow::Result;
use colored::Colorize;
use std::path::PathBuf;

use crate::db::{open_connection, queries};
use crate::export::{html, json, markdown};

pub fn execute(
    format: String,
    output: Option<String>,
    ids: Option<Vec<i64>>,
    tags: Option<Vec<String>>,
) -> Result<()> {
    let conn = open_connection()?;
    
    // Fetch articles based on filters
    let articles = if let Some(article_ids) = ids {
        queries::find_by_ids(&conn, &article_ids)?
    } else if let Some(tag_list) = tags {
        // Get all articles and filter by tags
        queries::list_articles_filtered(&conn, i64::MAX, true, false, false, &tag_list, "time", false)?
    } else {
        // Get all articles
        queries::list_articles_filtered(&conn, i64::MAX, true, false, false, &[], "time", false)?
    };
    
    if articles.is_empty() {
        println!("No articles to export");
        return Ok(());
    }
    
    // Determine output path
    let output_path = match output {
        Some(path) => PathBuf::from(path),
        None => {
            // Generate default filename
            let default_name = match format.as_str() {
                "json" => "stash-export.json",
                "markdown" => "stash-export-md",
                "html" => "stash-export.html",
                _ => "stash-export",
            };
            PathBuf::from(default_name)
        }
    };
    
    // Export based on format
    match format.as_str() {
        "json" => {
            json::export_to_json(&articles, &output_path)?;
            println!(
                "{} Exported {} article(s) to {}",
                "✓".green().bold(),
                articles.len(),
                output_path.display()
            );
        }
        "markdown" => {
            markdown::export_to_markdown(&articles, &output_path)?;
            println!(
                "{} Exported {} article(s) to {}",
                "✓".green().bold(),
                articles.len(),
                output_path.display()
            );
        }
        "html" => {
            html::export_to_html(&articles, &output_path)?;
            println!(
                "{} Exported {} article(s) to {}",
                "✓".green().bold(),
                articles.len(),
                output_path.display()
            );
        }
        _ => {
            anyhow::bail!("Unknown format: {}. Use json, markdown, or html", format);
        }
    }
    
    Ok(())
}

