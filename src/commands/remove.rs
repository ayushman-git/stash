use anyhow::Result;

use crate::db::{
    open_connection,
    queries::{archive_by_ids, delete_by_ids},
};

pub fn execute(ids: &[i64], force: bool) -> Result<()> {
    let conn = open_connection()?;
    let affected = if force {
        delete_by_ids(&conn, &ids)?
    } else {
        archive_by_ids(&conn, &ids)?
    };

    if affected == 0 {
        println!("No articles found with those IDs");
    } else {
        let action = if force { "deleted" } else { "archived" };
        println!("{} {} article(s)", action, affected);
    }
    
    Ok(())
}
