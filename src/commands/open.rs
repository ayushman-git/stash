use anyhow::Result;

use crate::{db::{
    open_connection,
    queries::{find_by_ids, get_random_articles, list_articles},
}, ui::list::{render_articles, OutputFormat}};

pub fn execute(ids: &[i64], random: Option<i64>) -> Result<()> {
    let conn = open_connection()?;

    let articles = match random {
        Some(count) => get_random_articles(&conn, count, false)?,
        None => {
            if ids.is_empty() {
                list_articles(&conn, 1, false)?
            } else {
                find_by_ids(&conn, ids)?
            }
        }
    };
   
    if articles.is_empty() {
        println!("No articles found. Add one with `stash add <url>`");
        return Ok(());
    }

    for article in &articles {
        browser::that(&article.url)?;
    }

    render_articles(&articles, OutputFormat::Table, false, false)?;

    Ok(())
}
