use anyhow::{Context, Result};
use crate::db;
use crate::ui::tui;

pub fn execute() -> Result<()> {
    let conn = db::open_connection().context("Failed to connect to database")?;
    
    tui::launch_tui(conn).context("Failed to launch TUI")?;
    
    Ok(())
}

