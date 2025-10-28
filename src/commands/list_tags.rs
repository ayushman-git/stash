use anyhow::Result;

use crate::db::{open_connection, queries::get_all_tags_with_counts};

pub fn execute() -> Result<()> {
    let conn = open_connection()?;
    
    let tags_with_counts = get_all_tags_with_counts(&conn)?;
    
    if tags_with_counts.is_empty() {
        println!("No tags found!");
        return Ok(());
    }
    
    for (tag, count) in tags_with_counts {
        println!("{} ({})", tag, count);
    }
    
    Ok(())
}

