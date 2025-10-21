use anyhow::Result;
use chrono::{DateTime, Local, Utc};
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table, presets};

use crate::{db::models::Article, ui::icons::Icons};

pub enum OutputFormat {
    Table,
    Json,
    Ids,
}

pub fn render_articles(
    articles: &[Article],
    format: OutputFormat,
    all: bool,
    archived: bool,
) -> Result<()> {
    match format {
        OutputFormat::Table => render_table(articles, all, archived),
        OutputFormat::Json => render_json(articles),
        OutputFormat::Ids => render_ids(articles),
    }
}

pub fn render_ids(articles: &[Article]) -> Result<()> {
    for article in articles {
        println!("{}", article.id);
    }

    Ok(())
}

fn format_timestamp(dt: &DateTime<Utc>) -> String {
    let local = dt.with_timezone(&Local);
    local.format("%H:%M %d/%m").to_string()
}

pub fn render_table(articles: &[Article], all: bool, archived: bool) -> Result<()> {
    let mut table = Table::new();
    table.load_preset(presets::UTF8_HORIZONTAL_ONLY);
    table.set_content_arrangement(ContentArrangement::Dynamic);

    let mut headers = vec![
        "ID",
        Icons::Star.glyph(),
        "Title",
        "Read",
        "Site",
        "Tags",
        "Saved",
    ];
    if all || archived {
        headers.push("Archived");
    }
    table.set_header(headers);

    for article in articles {
        let title_cell = if article.read {
            Cell::new(article.title.as_deref().unwrap_or("<no title>")).fg(Color::DarkGrey) // Dim read articles
        } else {
            Cell::new(article.title.as_deref().unwrap_or("<no title>"))
                .fg(Color::White)
                .add_attribute(Attribute::Bold) // Bold unread articles
        };
        let mut row = vec![
            Cell::new(article.id),
            Cell::new(if article.starred {
                Icons::Star.glyph()
            } else {
                ""
            }),
            title_cell,
            Cell::new(if article.read {
                Icons::Tick.glyph()
            } else {
                Icons::CircleEmpty.glyph()
            }),
            Cell::new(article.site.as_deref().unwrap_or("")),
            Cell::new(article.tags.join(", ")),
            Cell::new(format_timestamp(&article.saved_at)),
        ];

        if all || archived {
            row.push(Cell::new(if article.archived { "x" } else { "" }));
        }

        table.add_row(row);
    }

    println!("{}", table);
    Ok(())
}

pub fn render_json(articles: &[Article]) -> Result<()> {
    let json = serde_json::to_string_pretty(articles)?;
    println!("{}", json);

    Ok(())
}
