use anyhow::{Context, Result, bail};

use crate::{
    db::{self, open_connection},
    ui,
};

pub fn execute(all: bool, archived: bool, format: String, limit: i64) -> Result<()> {
    let conn = open_connection()?;

    let articles = if archived {
        db::queries::list_archived_articles(&conn, limit).context("Failed to query articles")?
    } else {
        db::queries::list_articles(&conn, limit, all).context("Failed to query articles")?
    };

    if articles.is_empty() {
        println!("No articles found!");
        return Ok(());
    }

    let output_format = match format.as_str() {
        "json" => ui::list::OutputFormat::Json,
        "ids" => ui::list::OutputFormat::Ids,
        "table" => ui::list::OutputFormat::Table,
        _ => {
            bail!("Invalid format '{}'. Use table, json or ids", format);
        }
    };

    crate::ui::list::render_articles(&articles, output_format, all, archived)
        .context("Failed to render articles")?;

    Ok(())
}
