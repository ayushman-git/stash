use anyhow::{Context, Result};
use directories::ProjectDirs;
use rusqlite::Connection;
use std::path::PathBuf;

// Embed migration files at compilation
mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("src/db/migrations");
}

pub mod models;
pub mod queries;
pub mod schema;

pub fn get_db_path() -> Result<PathBuf> {
    let project_dirs = ProjectDirs::from("", "", "stash")
        .context("Failed to determine project directories")?;

    let data_dir = project_dirs.data_dir();
    std::fs::create_dir_all(data_dir)
        .context("Failed to create data directory")?;

    Ok(data_dir.join("articles.db"))
}
