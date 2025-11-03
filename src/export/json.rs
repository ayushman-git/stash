use anyhow::{Context, Result};
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::db::models::Article;

pub fn export_to_json(articles: &[Article], output_path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(articles)
        .context("Failed to serialize articles to JSON")?;
    
    let mut file = File::create(output_path)
        .context(format!("Failed to create file: {}", output_path.display()))?;
    
    file.write_all(json.as_bytes())
        .context("Failed to write JSON to file")?;
    
    Ok(())
}

pub fn import_from_json(json_path: &Path) -> Result<Vec<Article>> {
    let json_content = std::fs::read_to_string(json_path)
        .context(format!("Failed to read file: {}", json_path.display()))?;
    
    let articles: Vec<Article> = serde_json::from_str(&json_content)
        .context("Failed to parse JSON")?;
    
    Ok(articles)
}

