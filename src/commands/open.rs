use anyhow::{Context, Result};

use crate::{
    db::{
        open_connection,
        queries::{find_by_ids, get_random_articles, list_articles, mark_read_by_ids},
    },
    ui::list::{OutputFormat, render_articles},
};

// Import the `open` crate (aliased as `browser` in Cargo.toml)
// This handles cross-platform browser opening (Windows, Mac, Linux)
use browser;

/// Detect if we're running in WSL (Windows Subsystem for Linux)
fn is_wsl() -> bool {
    #[cfg(target_os = "linux")]
    {
        if let Ok(contents) = std::fs::read_to_string("/proc/version") {
            return contents.to_lowercase().contains("microsoft") || 
                   contents.to_lowercase().contains("wsl");
        }
    }
    false
}

/// Open a URL in the default browser, with special handling for WSL
fn open_in_browser(url: &str) -> Result<()> {
    // Special handling for WSL - use Windows commands
    if is_wsl() {
        let output = std::process::Command::new("cmd.exe")
            .args(&["/c", "start", url])
            .output()
            .context("Failed to execute cmd.exe. Make sure Windows commands are accessible from WSL.")?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("cmd.exe failed to open URL: {}", stderr);
        }
        Ok(())
    } else {
        // Use the `open` crate for native platforms
        #[cfg(target_os = "windows")]
        {
            browser::that_detached(url)
                .with_context(|| format!("Failed to open URL '{}' in browser. Make sure you have a default browser configured.", url))?;
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            browser::that(url)
                .with_context(|| format!("Failed to open URL '{}' in browser. Make sure you have xdg-open or a similar tool installed.", url))?;
        }
        Ok(())
    }
}

pub fn execute(ids: &[i64], random: Option<i64>, keep_unread: bool) -> Result<()> {
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

    let read_articles = if keep_unread {
        articles
    } else {
        mark_read_by_ids(&conn, &articles.iter().map(|a| a.id).collect::<Vec<i64>>())?
    };

    for article in &read_articles {
        open_in_browser(&article.url)?;
    }

    render_articles(&read_articles, OutputFormat::Table, false, false)?;

    Ok(())
}
