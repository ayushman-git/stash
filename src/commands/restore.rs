use anyhow::Result;

use crate::{
    db::{
        open_connection,
        queries::{unarchive_by_ids, find_by_ids, list_articles_filtered},
    },
    ui::list::{OutputFormat, render_articles},
};

pub fn execute(ids: &[i64], all: bool) -> Result<()> {
    let conn = open_connection()?;
    
    let ids_to_restore = if all {
        // Get all archived article IDs
        let archived_articles = list_articles_filtered(
            &conn,
            i64::MAX,
            true,  // all
            true,  // archived
            false, // starred
            &[],   // tags
            "time",
            false,
        )?;
        archived_articles.iter().map(|a| a.id).collect::<Vec<i64>>()
    } else {
        ids.to_vec()
    };

    if ids_to_restore.is_empty() {
        if all {
            println!("No archived articles to restore");
        } else {
            println!("No article IDs provided");
        }
        return Ok(());
    }

    let affected = unarchive_by_ids(&conn, &ids_to_restore)?;

    if affected == 0 {
        println!("No articles found with those IDs");
    } else {
        println!("Restored {} article(s)", affected);
        
        // Show the restored articles
        let restored_articles = find_by_ids(&conn, &ids_to_restore)?;
        render_articles(&restored_articles, OutputFormat::Table, false, false)?;
    }
    
    Ok(())
}

