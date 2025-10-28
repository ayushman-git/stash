use anyhow::{Context, Result, bail};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use similar::{ChangeTag, TextDiff};
use std::fs;
use std::io::Write;
use std::process::Command;

use crate::db::{open_connection, queries};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EditableArticle {
    title: Option<String>,
    url: String,
    note: Option<String>,
    tags: Vec<String>,
    starred: bool,
    read: bool,
    archived: bool,
}

pub fn execute(id: &i64) -> Result<()> {
    let conn = open_connection()?;

    // Fetch the article
    let article = match queries::get_article_by_id(&conn, *id)? {
        Some(a) => a,
        None => {
            eprintln!("Error: Article with ID {} not found", id);
            std::process::exit(2);
        }
    };

    // Create editable version
    let original = EditableArticle {
        title: article.title.clone(),
        url: article.url.clone(),
        note: article.note.clone(),
        tags: article.tags.clone(),
        starred: article.starred,
        read: article.read,
        archived: article.archived,
    };

    // Serialize to YAML
    let yaml_content = serde_yaml::to_string(&original)
        .context("Failed to serialize article to YAML")?;

    // Create temporary file
    let mut temp_file = tempfile::Builder::new()
        .prefix("stash-edit-")
        .suffix(".yml")
        .tempfile()
        .context("Failed to create temporary file")?;

    temp_file
        .write_all(yaml_content.as_bytes())
        .context("Failed to write to temporary file")?;

    let temp_path = temp_file.path().to_path_buf();

    // Get editor from environment
    let editor = std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| {
            // Try to find a suitable editor
            for ed in &["vim", "vi", "nano", "emacs"] {
                if which::which(ed).is_ok() {
                    return ed.to_string();
                }
            }
            "vi".to_string()
        });

    // Open editor
    let status = Command::new(&editor)
        .arg(&temp_path)
        .status()
        .context(format!("Failed to open editor: {}", editor))?;

    if !status.success() {
        bail!("Editor exited with non-zero status");
    }

    // Read back the modified content
    let modified_content = fs::read_to_string(&temp_path)
        .context("Failed to read modified file")?;

    // Parse the modified YAML
    let modified: EditableArticle = match serde_yaml::from_str(&modified_content) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error: Failed to parse YAML: {}", e);
            std::process::exit(1);
        }
    };

    // Validate URL
    if let Err(e) = url::Url::parse(&modified.url) {
        eprintln!("Error: Invalid URL format: {}", e);
        std::process::exit(1);
    }

    // Check if anything changed
    if original.title == modified.title
        && original.url == modified.url
        && original.note == modified.note
        && original.tags == modified.tags
        && original.starred == modified.starred
        && original.read == modified.read
        && original.archived == modified.archived
    {
        println!("No changes made.");
        return Ok(());
    }

    // Update the article in the database
    let updated = queries::update_article_metadata(
        &conn,
        *id,
        modified.title.clone(),
        modified.url.clone(),
        modified.note.clone(),
        modified.tags.clone(),
        modified.starred,
        modified.read,
        modified.archived,
    )?;

    // Display diff
    println!("\n{}", "Changes:".bold());
    display_diff(&original, &modified);

    println!(
        "\n{} Article #{} updated successfully",
        "✓".green().bold(),
        updated.id
    );

    Ok(())
}

fn display_diff(original: &EditableArticle, modified: &EditableArticle) {
    let original_yaml = serde_yaml::to_string(original).unwrap_or_default();
    let modified_yaml = serde_yaml::to_string(modified).unwrap_or_default();

    let diff = TextDiff::from_lines(&original_yaml, &modified_yaml);

    for change in diff.iter_all_changes() {
        let line = change.to_string();
        let content = line.trim_end();
        
        match change.tag() {
            ChangeTag::Delete => {
                println!("{}", format!("- {}", content).red());
            }
            ChangeTag::Insert => {
                println!("{}", format!("+ {}", content).green());
            }
            ChangeTag::Equal => {
                println!("  {}", content);
            }
        }
    }
}

