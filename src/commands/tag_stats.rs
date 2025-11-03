use anyhow::Result;
use colored::Colorize;
use comfy_table::{presets, Attribute, Cell, Color, ContentArrangement, Table};

use crate::db::{open_connection, queries};

pub fn execute(sort: String, min_count: i64) -> Result<()> {
    let conn = open_connection()?;
    
    let mut tag_counts = queries::get_all_tags_with_counts(&conn)?;
    
    // Filter by minimum count
    if min_count > 1 {
        tag_counts.retain(|(_, count)| *count >= min_count as usize);
    }
    
    if tag_counts.is_empty() {
        println!("No tags found");
        return Ok(());
    }
    
    // Sort based on option
    match sort.as_str() {
        "count" => {
            tag_counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        }
        "alpha" => {
            // Already sorted alphabetically by default
        }
        _ => {
            anyhow::bail!("Invalid sort option: {}. Use 'count' or 'alpha'", sort);
        }
    }
    
    // Create table
    let mut table = Table::new();
    table.load_preset(presets::NOTHING);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    
    table.set_header(vec![
        Cell::new("Tag")
            .fg(Color::Cyan)
            .add_attribute(Attribute::Bold),
        Cell::new("Count")
            .fg(Color::Cyan)
            .add_attribute(Attribute::Bold),
    ]);
    
    let total_tags = tag_counts.len();
    let total_uses: usize = tag_counts.iter().map(|(_, count)| count).sum();
    
    for (tag, count) in &tag_counts {
        table.add_row(vec![
            Cell::new(tag).fg(Color::White),
            Cell::new(count).fg(Color::Green),
        ]);
    }
    
    println!("{}", table);
    println!(
        "\n{} Total: {} unique tags, {} total uses",
        "ℹ".cyan().bold(),
        total_tags,
        total_uses
    );
    
    Ok(())
}

