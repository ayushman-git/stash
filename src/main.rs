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
        url: Option<String>,

        #[arg(short, long, value_delimiter = ',')]
        tags: Vec<String>,

        #[arg(long)]
        title: Option<String>,

        #[arg(long)]
        no_fetch: bool,
    },

    #[command(alias = "ls")]
    List {
        #[arg(short, long)]
        all: bool,

        #[arg(short = 'A', long)]
        archived: bool,

        #[arg(short = 'n', default_value = "10")]
        limit: i64,

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
    Open {
        #[arg(value_delimiter = ',')]
        ids: Vec<i64>,
    },
    Star {
        #[arg(value_delimiter = ',')]
        ids: Vec<i64>,
    },
    Unstar {
        #[arg(value_delimiter = ',')]
        ids: Vec<i64>,
    },
    Pick,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add {
            url,
            tags,
            title,
            no_fetch,
        } => {
            commands::add::execute(url, tags, title, no_fetch)?;
        }
        Commands::List {
            all,
            archived,
            format,
            limit,
        } => {
            commands::list::execute(all, archived, format, limit)?;
        }
        Commands::Remove { ids, force } => {
            commands::remove::execute(&ids, force)?;
        }
        Commands::Open { ids } => {
            commands::open::execute(&ids)?;
        }
        Commands::Star { ids } => {
            commands::star::execute(&ids)?;
        }
        Commands::Unstar { ids } => {
            commands::unstar::execute(&ids)?;
        }
        Commands::Pick => {
            commands::pick::execute()?;
        }
    }
    Ok(())
}
