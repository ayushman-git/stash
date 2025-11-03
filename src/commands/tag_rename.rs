use anyhow::Result;
use colored::Colorize;

use crate::db::{open_connection, queries};

pub fn execute(old_tag: String, new_tag: String) -> Result<()> {
    let conn = open_connection()?;
    
    let updated = queries::rename_tag(&conn, &old_tag, &new_tag)?;
    
    if updated == 0 {
        println!("No articles found with tag '{}'", old_tag);
    } else {
        println!(
            "{} Renamed tag '{}' to '{}' in {} article(s)",
            "✓".green().bold(),
            old_tag,
            new_tag,
            updated
        );
    }
    
    Ok(())
}

