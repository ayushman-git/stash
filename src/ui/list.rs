use anyhow::Result;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table, presets};

use crate::{
    db::models::Article,
    ui::{formatters::datetime_humanize, icons::Icons, theme::Theme},
};

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

fn get_row_style(article: &Article, theme: &Theme) -> (Color, bool, Option<Color>) {
    if article.archived {
        (theme.read_color(), false, None)
    } else if !article.read && article.starred {
        (theme.starred_color(), true, None)
    } else if !article.read {
        (theme.unread_color(), false, None)
    } else {
        (theme.read_color(), false, None)
    }
}

pub fn render_table(articles: &[Article], all: bool, archived: bool) -> Result<()> {
    // Detect terminal theme
    let theme = Theme::detect();
    let header_color = theme.header_color();
    
    let mut table = Table::new();
    table.load_preset(presets::NOTHING);
    table.set_content_arrangement(ContentArrangement::Dynamic);

    let mut headers = vec![
        Cell::new("ID")
            .fg(header_color)
            .add_attribute(Attribute::Bold),
        Cell::new("").fg(header_color).add_attribute(Attribute::Bold),
        Cell::new("Title")
            .fg(header_color)
            .add_attribute(Attribute::Bold),
        Cell::new("Read")
            .fg(header_color)
            .add_attribute(Attribute::Bold),
        Cell::new("Site")
            .fg(header_color)
            .add_attribute(Attribute::Bold),
        Cell::new("Tags")
            .fg(header_color)
            .add_attribute(Attribute::Bold),
        Cell::new("Saved")
            .fg(header_color)
            .add_attribute(Attribute::Bold),
    ];
    if all || archived {
        headers.push(
            Cell::new("Archived")
                .fg(header_color)
                .add_attribute(Attribute::Bold),
        );
    }
    table.set_header(headers);

    for article in articles {
        let (color, _bold, _bg) = get_row_style(&article, &theme);

        let mut row = vec![
            Cell::new(article.id).fg(color),
            Cell::new(if article.starred {
                Icons::Star.glyph()
            } else {
                ""
            }),
            Cell::new(article.title.as_deref().unwrap_or("<no title>"))
                .fg(color)
                .add_attribute(Attribute::Bold),
            Cell::new(if article.read {
                Icons::Tick.glyph()
            } else {
                Icons::CircleEmpty.glyph()
            })
            .fg(color),
            Cell::new(article.site.as_deref().unwrap_or("")).fg(color),
            Cell::new(article.tags.join(", ")).fg(color),
            Cell::new(datetime_humanize(article.saved_at)).fg(color),
        ];

        if all || archived {
            row.push(
                Cell::new(if article.archived {
                    Icons::Deleted.glyph()
                } else {
                    ""
                })
                .fg(theme.archived_color()),
            );
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
