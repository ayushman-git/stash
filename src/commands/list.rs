use anyhow::{Context, Result};

use crate::db::{self, open_connection};

pub fn execute(archived: bool) -> Result<()> {
    let conn = open_connection()?;

    let articles = db::queries::list_articles(&conn, 10, archived)
        .context("Failed to query articles")?;

    if articles.is_empty() {
        let msg = if archived {
            "No archived articles found"
        } else {
            "No articles found. Add one with `stash add <url>`"
        };

        println!("{}", msg);
        return Ok(());
    }

    

    Ok(())
}
