use anyhow::Result;

use crate::{
    db::{open_connection, queries::{set_read_all, set_read_by_ids}},
    ui::list::{render_articles, OutputFormat},
};

pub fn execute(ids: &[i64], all: bool) -> Result<()> {
    let conn = open_connection()?;
    let articles_affected = if all {
        set_read_all(&conn, true, false)?
    } else {
        set_read_by_ids(&conn, &ids, true)?
    };

    if articles_affected.is_empty() {
        println!("No articles found with IDs {:?}", ids);
    } else {
        render_articles(&articles_affected, OutputFormat::Table, false, false)?;
    }
    Ok(())
}
