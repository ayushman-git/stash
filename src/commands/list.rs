use anyhow::{Context, Result, bail};

use crate::{
    db::{self, open_connection},
    ui,
};

pub fn execute(
    all: bool,
    archived: bool,
    format: String,
    limit: i64,
    starred: bool,
    tags: Vec<String>,
    sort: String,
    reverse: bool,
    browser: bool,
) -> Result<()> {
    let conn = open_connection()?;

    // Validate sort field
    let valid_sorts = ["time", "title", "site", "read", "star"];
    if !valid_sorts.contains(&sort.as_str()) {
        bail!(
            "Invalid sort field '{}'. Use: {}",
            sort,
            valid_sorts.join(", ")
        );
    }

    let articles = db::queries::list_articles_filtered(
        &conn, limit, all, archived, starred, &tags, &sort, reverse
    )
    .context("Failed to query articles")?;

    if articles.is_empty() {
        println!("No articles found!");
        return Ok(());
    }

    // If browser flag is set, render in browser instead
    if browser {
        return crate::ui::browser::render_browser(&articles, all, archived)
            .context("Failed to render articles in browser");
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
