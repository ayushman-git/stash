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

        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        extra_args: Vec<String>,
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

        #[arg(long)]
        starred: bool,

        #[arg(short = 'T', long, value_delimiter = ',')]
        tag: Vec<String>,

        #[arg(short = 't', long, default_value = "time")]
        sort: String,

        #[arg(short, long)]
        reverse: bool,

        #[arg(short = 'b', long)]
        browser: bool,

        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        extra_args: Vec<String>,
    },

    #[command(alias = "rm")]
    Remove {
        ids: Vec<i64>,

        #[arg(short, long)]
        force: bool,
    },

    Edit {
        id: i64,
    },

    #[command(alias = "o")]
    Open {
        #[arg(value_delimiter = ',')]
        ids: Vec<i64>,

        #[arg(short, long, default_missing_value = "1", num_args = 0..=1)]
        random: Option<i64>,

        #[arg(long)]
        keep_unread: bool,
    },
    Star {
        #[arg(value_delimiter = ',')]
        ids: Vec<i64>,
    },
    Unstar {
        #[arg(value_delimiter = ',')]
        ids: Vec<i64>,
    },
    MarkRead {
        #[arg(value_delimiter = ',')]
        ids: Vec<i64>,

        #[arg(short, long)]
        all: bool,
    },
    MarkUnread {
        #[arg(value_delimiter = ',')]
        ids: Vec<i64>,

        #[arg(short, long)]
        all: bool,
    },
    Pick,
    Tag {
        id: i64,

        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        tags: Vec<String>,
    },
    Tags,
    Tui,
    Search {
        query: String,

        #[arg(short, long)]
        all: bool,

        #[arg(short = 'A', long)]
        archived: bool,

        #[arg(short = 'n', long, default_value = "20")]
        limit: i64,

        #[arg(short = 'f', long, default_value = "table")]
        format: String,

        #[arg(long)]
        starred: bool,

        #[arg(short = 'T', long, value_delimiter = ',')]
        tag: Vec<String>,

        #[arg(short = 'b', long)]
        browser: bool,

        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        extra_args: Vec<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add {
            url,
            tags,
            title,
            no_fetch,
            extra_args,
        } => {
            commands::add::execute(url, tags, title, no_fetch, extra_args)?;
        }
        Commands::List {
            all,
            archived,
            format,
            limit,
            starred,
            tag,
            sort,
            reverse,
            browser,
            extra_args,
        } => {
            // Support multiple tag formats:
            // 1. --tag rust,webdev (comma-separated)
            // 2. --tag rust --tag webdev (multiple flags)
            // 3. +rust +webdev (+ prefix positional args)
            let mut tags: Vec<String> = tag
                .iter()
                .flat_map(|t| t.split(','))
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            
            // Parse +tag syntax from extra_args
            for arg in extra_args {
                if let Some(tag_name) = arg.strip_prefix('+') {
                    if !tag_name.is_empty() {
                        tags.push(tag_name.to_string());
                    }
                }
            }
            
            commands::list::execute(all, archived, format, limit, starred, tags, sort, reverse, browser)?;
        }
        Commands::Remove { ids, force } => {
            commands::remove::execute(&ids, force)?;
        }
        Commands::Edit { id } => {
            commands::edit::execute(&id)?;
        }
        Commands::Open {
            ids,
            random,
            keep_unread,
        } => {
            commands::open::execute(&ids, random, keep_unread)?;
        }
        Commands::Star { ids } => {
            commands::star::execute(&ids)?;
        }
        Commands::Unstar { ids } => {
            commands::unstar::execute(&ids)?;
        }
        Commands::MarkRead { ids, all } => {
            commands::mark_read::execute(&ids, all)?;
        }
        Commands::MarkUnread { ids, all } => {
            commands::mark_unread::execute(&ids, all)?;
        }
        Commands::Pick => {
            commands::pick::execute()?;
        }
        Commands::Tag { id, tags } => {
            commands::tag::execute(&id, &tags)?;
        }
        Commands::Tags => {
            commands::list_tags::execute()?;
        }
        Commands::Tui => {
            commands::tui::execute()?;
        }
        Commands::Search {
            query,
            all,
            archived,
            format,
            limit,
            starred,
            tag,
            browser,
            extra_args,
        } => {
            // Support multiple tag formats (same as list command)
            let mut tags: Vec<String> = tag
                .iter()
                .flat_map(|t| t.split(','))
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            
            // Parse +tag syntax from extra_args
            for arg in extra_args {
                if let Some(tag_name) = arg.strip_prefix('+') {
                    if !tag_name.is_empty() {
                        tags.push(tag_name.to_string());
                    }
                }
            }
            
            commands::search::execute(query, all, archived, format, limit, starred, tags, browser)?;
        }
    }
    Ok(())
}
