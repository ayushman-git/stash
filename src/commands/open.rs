use anyhow::Result;

use crate::db::{
    open_connection,
    queries::{find_by_ids, list_articles},
};

pub fn execute(ids: &[i64]) -> Result<()> {
    let conn = open_connection()?;
    if ids.is_empty() {
        let articles = list_articles(&conn, 1, false)?;
        if let Some(article) = articles.get(0) {
            browser::that(&article.url)?;
        } else {
            println!("No articles found. Add one with `stash add <url>`")
        }
        return Ok(());
    }
    let articles = find_by_ids(&conn, ids)?;

    if articles.len() == 0 {
        println!("No articles found. Add one with `stash add <url>`");
        return Ok(());
    }

    for article in &articles {
        browser::that(&article.url)?;
    }
    Ok(())
}
