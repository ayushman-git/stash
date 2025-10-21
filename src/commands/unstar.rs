use anyhow::Result;

use crate::{
    db::{open_connection, queries::set_starred_by_ids},
    ui::list::{render_articles, OutputFormat},
};

pub fn execute(ids: &[i64]) -> Result<()> {
    if ids.is_empty() {
        println!("No article IDs provided");
        return Ok(());
    }

    let conn = open_connection()?;
    let articles_affected = set_starred_by_ids(&conn, ids, false)?;

    render_articles(&articles_affected, OutputFormat::Table, false, false)?;

    Ok(())
}
