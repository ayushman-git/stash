use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::{Context, Result};
use which::which;

use crate::db::{open_connection, queries::list_articles};

pub fn execute() -> Result<()> {
    let fzf_path = which("fzf")
        .context("fzf not found in PATH. Install with: brew install fzf")?;
    
    let conn = open_connection()?;
    let articles = list_articles(&conn, 100, false)
        .context("Failed to query articles")?;

    if articles.is_empty() {
        anyhow::bail!("No articles available to pick from");
    }
    
    let fzf_input = articles
        .iter()
        .map(|a| {
            format!(
                "{} | {} | {}",
                a.id,
                a.title.as_deref().unwrap_or("(no title)"),
                a.site.as_deref().unwrap_or(""),
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let mut fzf = Command::new(fzf_path)
        .arg("--height=40%")
        .arg("--reverse")
        .arg("--prompt=Select article: ")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to spawn fzf")?;
    
    {
        let stdin = fzf.stdin.as_mut()
            .context("Failed to open fzf stdin")?;
        stdin.write_all(fzf_input.as_bytes())
            .context("Failed to write to fzf")?;
    }

    let output = fzf.wait_with_output()
        .context("fzf process failed")?;

    if !output.status.success() {
        anyhow::bail!("Selection cancelled");
    }

    let selection = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 from fzf")?;
    
    let id_str = selection.trim()
        .split(" | ")
        .next()
        .context("Failed to parse fzf output")?;
    
    let id: i64 = id_str.parse()
        .context("Invalid article ID in selection")?;

    let article = articles.iter()
        .find(|a| a.id == id)
        .context("Selected article not found")?;

    browser::that(&article.url)
        .context("Failed to open browser")?;

    println!("Opened: {}", article.title.as_deref().unwrap_or(&article.url));

    Ok(())
}
