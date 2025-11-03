use anyhow::Result;
use colored::Colorize;
use dialoguer::Confirm;

use crate::db::{open_connection, queries};

pub fn execute(tag: String, force: bool) -> Result<()> {
    let conn = open_connection()?;
    
    // Confirm deletion unless forced
    if !force {
        let confirm = Confirm::new()
            .with_prompt(format!("Delete tag '{}' from all articles?", tag))
            .default(false)
            .interact()?;
        
        if !confirm {
            println!("Cancelled");
            return Ok(());
        }
    }
    
    let updated = queries::delete_tag(&conn, &tag)?;
    
    if updated == 0 {
        println!("No articles found with tag '{}'", tag);
    } else {
        println!(
            "{} Deleted tag '{}' from {} article(s)",
            "✓".green().bold(),
            tag,
            updated
        );
    }
    
    Ok(())
}

