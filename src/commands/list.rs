use anyhow::{Context, Result, bail};

use crate::{
    db::{self, open_connection},
    ui,
};

pub fn execute(all: bool, format: String) -> Result<()> {
    let conn = open_connection()?;

    let articles =
        db::queries::list_articles(&conn, 10, all).context("Failed to query articles")?;

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

    crate::ui::list::render_articles(&articles, output_format)
        .context("Failed to render articles")?;

    Ok(())
}
