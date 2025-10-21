use anyhow::Result;

use crate::{
    db::{open_connection, queries::set_starred_by_ids},
    ui::list::{OutputFormat, render_articles},
};

pub fn execute(ids: &[i64]) -> Result<()> {
    if ids.is_empty() {
        println!("No article IDs provided");
        return Ok(());
    }

    let conn = open_connection()?;
    let articles_affected = set_starred_by_ids(&conn, ids, true)?;

    render_articles(&articles_affected, OutputFormat::Table, false, false)?;

    Ok(())
}
