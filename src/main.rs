mod commands;
mod ui;
mod db;

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "stash")]
#[command(about = "Manage your articles")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        url: String,
        #[arg(short, long)]
        tags: Vec<String>,
    },
    List {
        #[arg(short, long)]
        archived: bool,

        #[arg(short, long, default_value = "table")]
        format: String,
    },
    Remove {
        #[arg(short, long)]
        ids: Vec<i64>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { url, tags } => {
            commands::add::execute(url, tags)?;
        }
        Commands::List { archived, format } => {
            commands::list::execute(archived, format)?;
        }
        Commands::Remove { ids } => {
            commands::remove::execute(ids)?;
        }
    }
    Ok(())
}
