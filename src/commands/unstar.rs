use anyhow::Result;

use crate::db::{open_connection, queries::set_starred_by_ids};

pub fn execute(ids: &[i64]) -> Result<()> {
    if ids.is_empty() {
        println!("No article IDs provided");
        return Ok(());
    }
    
    let conn = open_connection()?;
    let affected = set_starred_by_ids(&conn, ids, false)?;
    
    if affected > 0 {
        println!("Unstarred {} article(s)", affected);
    } else {
        println!("No articles found with the provided ID(s)");
    }
    
    Ok(())
}
