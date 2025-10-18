mod commands;

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
        #[arg(short, long)]
        tags: Vec<String>,
    },
    List {
        archived: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { url, tags } => {
            commands::add::execute(url, tags);
        }
        Commands::List { archived } => {
            commands::list::execute(archived);
        }
    }
}
