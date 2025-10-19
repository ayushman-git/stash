use anyhow::Result;

use crate::db::{open_connection, queries::find_by_ids};

pub fn execute(ids: &[i64]) -> Result<()> {
    let conn = open_connection()?;

    let articles = find_by_ids(&conn, ids)?;

    for article in articles {
        browser::that(article.url)?;
    }
    Ok(())
}