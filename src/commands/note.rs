use anyhow::{Context, Result, bail};
use colored::Colorize;
use std::fs;
use std::io::Write;
use std::process::Command;

use crate::config;
use crate::db::{open_connection, queries};

pub fn execute(id: &i64, note_text: Option<String>, append: bool, clear: bool) -> Result<()> {
    let conn = open_connection()?;

    // Fetch the article
    let article = match queries::get_article_by_id(&conn, *id)? {
        Some(a) => a,
        None => {
            eprintln!("Error: Article with ID {} not found", id);
            std::process::exit(2);
        }
    };

    let new_note = if clear {
        // Clear the note
        None
    } else if let Some(text) = note_text {
        // Add or append inline note
        if append {
            // Append to existing note
            match article.note {
                Some(existing) => Some(format!("{}\n{}", existing, text)),
                None => Some(text),
            }
        } else {
            // Replace note
            Some(text)
        }
    } else {
        // Open editor for note
        let original_note = article.note.clone().unwrap_or_default();
        
        // Create temporary file
        let mut temp_file = tempfile::Builder::new()
            .prefix("stash-note-")
            .suffix(".md")
            .tempfile()
            .context("Failed to create temporary file")?;

        temp_file
            .write_all(original_note.as_bytes())
            .context("Failed to write to temporary file")?;

        let temp_path = temp_file.path().to_path_buf();

        // Get editor from config, fallback to environment variables
        let config = config::load_config()?;
        let editor = if config.defaults.editor != "default" {
            config.defaults.editor
        } else {
            std::env::var("EDITOR")
                .or_else(|_| std::env::var("VISUAL"))
                .unwrap_or_else(|_| {
                    // Try to find a suitable editor
                    for ed in &["vim", "vi", "nano", "emacs"] {
                        if which::which(ed).is_ok() {
                            return ed.to_string();
                        }
                    }
                    "vi".to_string()
                })
        };

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

        // Check if anything changed
        if original_note == modified_content {
            println!("No changes made.");
            return Ok(());
        }

        // Use None if empty, otherwise Some
        if modified_content.trim().is_empty() {
            None
        } else {
            Some(modified_content)
        }
    };

    // Update the article note in the database
    let updated = queries::update_note(&conn, *id, new_note.clone())?;

    // Display result
    if clear {
        println!("{} Note cleared for article #{}", "✓".green().bold(), updated.id);
    } else if let Some(ref note) = new_note {
        println!("{} Note updated for article #{}", "✓".green().bold(), updated.id);
        
        // Display the note with a preview (first 100 chars)
        let preview = if note.len() > 100 {
            format!("{}...", &note[..100])
        } else {
            note.clone()
        };
        println!("\n{}", "Note:".bold());
        println!("{}", preview.dimmed());
    } else {
        println!("{} Note cleared for article #{}", "✓".green().bold(), updated.id);
    }

    Ok(())
}

