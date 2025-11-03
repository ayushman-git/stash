use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::db::models::Article;

pub fn export_to_markdown(articles: &[Article], output_dir: &Path) -> Result<()> {
    // Create output directory if it doesn't exist
    fs::create_dir_all(output_dir)
        .context(format!("Failed to create directory: {}", output_dir.display()))?;
    
    for article in articles {
        let filename = sanitize_filename(article);
        let file_path = output_dir.join(filename);
        
        let content = format_article_as_markdown(article);
        
        fs::write(&file_path, content)
            .context(format!("Failed to write file: {}", file_path.display()))?;
    }
    
    Ok(())
}

#[allow(dead_code)]
pub fn import_from_markdown(markdown_dir: &Path) -> Result<Vec<Article>> {
    let mut articles = Vec::new();
    
    let entries = fs::read_dir(markdown_dir)
        .context(format!("Failed to read directory: {}", markdown_dir.display()))?;
    
    for entry in entries {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            if let Ok(article) = parse_markdown_file(&path) {
                articles.push(article);
            }
        }
    }
    
    Ok(articles)
}

fn sanitize_filename(article: &Article) -> String {
    let title = article.title.as_deref().unwrap_or("untitled");
    let safe_title: String = title
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .take(100)
        .collect();
    
    format!("{}-{}.md", article.id, safe_title)
}

fn format_article_as_markdown(article: &Article) -> String {
    let mut content = String::new();
    
    // Title
    content.push_str(&format!("# {}\n\n", article.title.as_deref().unwrap_or("Untitled")));
    
    // Metadata
    content.push_str("---\n\n");
    content.push_str(&format!("- **ID**: {}\n", article.id));
    content.push_str(&format!("- **URL**: {}\n", article.url));
    
    if let Some(site) = &article.site {
        content.push_str(&format!("- **Site**: {}\n", site));
    }
    
    content.push_str(&format!("- **Saved**: {}\n", article.saved_at));
    
    if !article.tags.is_empty() {
        content.push_str(&format!("- **Tags**: {}\n", article.tags.join(", ")));
    }
    
    content.push_str(&format!("- **Read**: {}\n", if article.read { "Yes" } else { "No" }));
    content.push_str(&format!("- **Starred**: {}\n", if article.starred { "Yes" } else { "No" }));
    content.push_str(&format!("- **Archived**: {}\n", if article.archived { "Yes" } else { "No" }));
    
    content.push_str("\n---\n\n");
    
    // Description
    if let Some(description) = &article.description {
        content.push_str(&format!("## Description\n\n{}\n\n", description));
    }
    
    // Note
    if let Some(note) = &article.note {
        content.push_str(&format!("## Note\n\n{}\n\n", note));
    }
    
    // Content
    if let Some(markdown_content) = &article.content_markdown {
        content.push_str("## Content\n\n");
        content.push_str(markdown_content);
        content.push('\n');
    }
    
    content
}

#[allow(dead_code)]
fn parse_markdown_file(_path: &Path) -> Result<Article> {
    // This is a simplified parser - in production you'd want more robust parsing
    // For now, we'll return an error as this is complex to implement properly
    anyhow::bail!("Markdown import not fully implemented - use JSON import instead")
}

