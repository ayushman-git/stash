use anyhow::{Context, Result};
use colored::Colorize;

use crate::config::{self, schema::Config};

pub fn execute_set(key: String, value: String) -> Result<()> {
    let mut config = config::load_config()?;
    
    // Parse the key and set the value
    let parts: Vec<&str> = key.split('.').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid key format. Use format: section.key (e.g., defaults.editor)");
    }
    
    let section = parts[0];
    let field = parts[1];
    
    match section {
        "defaults" => match field {
            "editor" => config.defaults.editor = value,
            "browser" => config.defaults.browser = value,
            "output_format" => {
                if !["table", "json", "ids"].contains(&value.as_str()) {
                    anyhow::bail!("Invalid output_format. Must be one of: table, json, ids");
                }
                config.defaults.output_format = value;
            }
            "list_limit" => {
                config.defaults.list_limit = value.parse()
                    .context("list_limit must be a number")?;
            }
            "auto_read" => {
                config.defaults.auto_read = value.parse()
                    .context("auto_read must be true or false")?;
            }
            _ => anyhow::bail!("Unknown defaults field: {}", field),
        },
        "colors" => match field {
            "theme" => {
                if !["auto", "dark", "light", "none"].contains(&value.as_str()) {
                    anyhow::bail!("Invalid theme. Must be one of: auto, dark, light, none");
                }
                config.colors.theme = value;
            }
            _ => anyhow::bail!("Unknown colors field: {}", field),
        },
        "fetch" => match field {
            "timeout_seconds" => {
                config.fetch.timeout_seconds = value.parse()
                    .context("timeout_seconds must be a number")?;
            }
            "follow_redirects" => {
                config.fetch.follow_redirects = value.parse()
                    .context("follow_redirects must be true or false")?;
            }
            "user_agent" => config.fetch.user_agent = value,
            _ => anyhow::bail!("Unknown fetch field: {}", field),
        },
        _ => anyhow::bail!("Unknown section: {}. Valid sections: defaults, colors, fetch", section),
    }
    
    config::save_config(&config)?;
    println!("{} Config updated: {} = {}", "✓".green().bold(), key, config.defaults.editor);
    
    Ok(())
}

pub fn execute_get(key: String) -> Result<()> {
    let config = config::load_config()?;
    
    let parts: Vec<&str> = key.split('.').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid key format. Use format: section.key (e.g., defaults.editor)");
    }
    
    let section = parts[0];
    let field = parts[1];
    
    let value = match section {
        "defaults" => match field {
            "editor" => config.defaults.editor,
            "browser" => config.defaults.browser,
            "output_format" => config.defaults.output_format,
            "list_limit" => config.defaults.list_limit.to_string(),
            "auto_read" => config.defaults.auto_read.to_string(),
            _ => anyhow::bail!("Unknown defaults field: {}", field),
        },
        "colors" => match field {
            "theme" => config.colors.theme,
            _ => anyhow::bail!("Unknown colors field: {}", field),
        },
        "fetch" => match field {
            "timeout_seconds" => config.fetch.timeout_seconds.to_string(),
            "follow_redirects" => config.fetch.follow_redirects.to_string(),
            "user_agent" => config.fetch.user_agent,
            _ => anyhow::bail!("Unknown fetch field: {}", field),
        },
        _ => anyhow::bail!("Unknown section: {}", section),
    };
    
    println!("{}", value);
    Ok(())
}

pub fn execute_list() -> Result<()> {
    let config = config::load_config()?;
    
    println!("{}", "[defaults]".bold());
    println!("  editor = {}", config.defaults.editor);
    println!("  browser = {}", config.defaults.browser);
    println!("  output_format = {}", config.defaults.output_format);
    println!("  list_limit = {}", config.defaults.list_limit);
    println!("  auto_read = {}", config.defaults.auto_read);
    
    println!("\n{}", "[colors]".bold());
    println!("  theme = {}", config.colors.theme);
    
    println!("\n{}", "[fetch]".bold());
    println!("  timeout_seconds = {}", config.fetch.timeout_seconds);
    println!("  follow_redirects = {}", config.fetch.follow_redirects);
    println!("  user_agent = {}", config.fetch.user_agent);
    
    Ok(())
}

pub fn execute_reset() -> Result<()> {
    let default_config = Config::default();
    config::save_config(&default_config)?;
    
    println!("{} Config reset to defaults", "✓".green().bold());
    Ok(())
}

pub fn execute_path() -> Result<()> {
    let config_path = config::get_config_path()?;
    println!("{}", config_path.display());
    Ok(())
}

