use anyhow::{bail, Context, Result};

use crate::{db::{self, open_connection}, ui::{self, list::render_articles}};

pub fn execute(archived: bool, format: String) -> Result<()> {
    let conn = open_connection()?;

    let articles = db::queries::list_articles(&conn, 10, archived)
        .context("Failed to query articles")?;

    if articles.is_empty() {
        let msg = if archived {
            "No archived articles found"
        } else {
            "No articles found. Add one with `stash add <url>`"
        };

        println!("{}", msg);
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

    crate::ui::list::render_articles(&articles, output_format)
        .context("Failed to render articles")?;

    Ok(())
}
