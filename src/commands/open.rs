use anyhow::Result;

use crate::db::{open_connection, queries::find_by_id};

pub fn execute(id: &i64) -> Result<()> {
    let conn = open_connection()?;

    let article = find_by_id(&conn, id)?;
    browser::that(article.unwrap().url);
    Ok(())
}