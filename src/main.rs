mod commands;
mod db;
mod fetch;
mod ui;

use anyhow::Result;
use clap::{Parser, Subcommand};

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
        #[arg(short, long, value_delimiter = ',')]
        tags: Vec<String>,
    },

    #[command(alias = "ls")]
    List {
        #[arg(short, long)]
        archived: bool,

        #[arg(short, long, default_value = "table")]
        format: String,
    },

    #[command(alias = "rm")]
    Remove {
        ids: Vec<i64>,

        #[arg(short, long)]
        force: bool,
    },

    #[command(alias = "o")]
    Open { id: i64 },
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
        Commands::Remove { ids, force } => {
            commands::remove::execute(&ids, force)?;
        }
        Commands::Open { id } => {
            commands::open::execute(&id)?;
        }
    }
    Ok(())
}
