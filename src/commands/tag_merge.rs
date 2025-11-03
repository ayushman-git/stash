use anyhow::Result;
use colored::Colorize;

use crate::db::{open_connection, queries};

pub fn execute(tags: Vec<String>, into: String) -> Result<()> {
    if tags.is_empty() {
        anyhow::bail!("No tags provided to merge");
    }
    
    let conn = open_connection()?;
    
    let updated = queries::merge_tags(&conn, &tags, &into)?;
    
    if updated == 0 {
        println!("No articles found with any of the specified tags");
    } else {
        println!(
            "{} Merged tags [{}] into '{}' in {} article(s)",
            "✓".green().bold(),
            tags.join(", "),
            into,
            updated
        );
    }
    
    Ok(())
}

