pub mod schema;

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

use schema::Config;

pub fn get_config_path() -> Result<PathBuf> {
    let config_dir = directories::ProjectDirs::from("", "", "stash")
        .context("Failed to determine config directory")?
        .config_dir()
        .to_path_buf();
    
    Ok(config_dir.join("config.toml"))
}

pub fn ensure_config_dir() -> Result<PathBuf> {
    let config_dir = directories::ProjectDirs::from("", "", "stash")
        .context("Failed to determine config directory")?
        .config_dir()
        .to_path_buf();
    
    fs::create_dir_all(&config_dir)
        .context(format!("Failed to create config directory: {}", config_dir.display()))?;
    
    Ok(config_dir)
}

pub fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;
    
    if !config_path.exists() {
        // Return default config if file doesn't exist
        return Ok(Config::default());
    }
    
    let content = fs::read_to_string(&config_path)
        .context(format!("Failed to read config file: {}", config_path.display()))?;
    
    let config: Config = toml::from_str(&content)
        .context("Failed to parse config file")?;
    
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<()> {
    ensure_config_dir()?;
    let config_path = get_config_path()?;
    
    let content = toml::to_string_pretty(config)
        .context("Failed to serialize config")?;
    
    fs::write(&config_path, content)
        .context(format!("Failed to write config file: {}", config_path.display()))?;
    
    Ok(())
}

