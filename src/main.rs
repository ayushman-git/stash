mod commands;
mod config;
mod db;
mod export;
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
enum ConfigAction {
    Set {
        key: String,
        value: String,
    },
    Get {
        key: String,
    },
    List,
    Reset,
    Path,
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

    Restore {
        ids: Vec<i64>,

        #[arg(short, long)]
        all: bool,
    },

    Edit {
        id: i64,
    },

    Note {
        id: i64,

        #[arg(value_name = "TEXT")]
        text: Option<String>,

        #[arg(short, long)]
        append: bool,

        #[arg(short, long)]
        clear: bool,
    },

    Export {
        #[arg(short, long, default_value = "json")]
        format: String,

        #[arg(short, long)]
        output: Option<String>,

        #[arg(long, value_delimiter = ',')]
        ids: Option<Vec<i64>>,

        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },

    Import {
        path: String,

        #[arg(short, long)]
        merge: bool,

        #[arg(long)]
        dry_run: bool,
    },

    Config {
        #[command(subcommand)]
        action: ConfigAction,
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
    TagRename {
        old_tag: String,
        new_tag: String,
    },
    TagMerge {
        #[arg(value_delimiter = ',', required = true)]
        tags: Vec<String>,

        #[arg(long)]
        into: String,
    },
    TagDelete {
        tag: String,

        #[arg(short, long)]
        force: bool,
    },
    TagStats {
        #[arg(short, long, default_value = "alpha")]
        sort: String,

        #[arg(short, long, default_value = "1")]
        min_count: i64,
    },
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
        Commands::Restore { ids, all } => {
            commands::restore::execute(&ids, all)?;
        }
        Commands::Edit { id } => {
            commands::edit::execute(&id)?;
        }
        Commands::Note { id, text, append, clear } => {
            commands::note::execute(&id, text, append, clear)?;
        }
        Commands::Export { format, output, ids, tags } => {
            commands::export::execute(format, output, ids, tags)?;
        }
        Commands::Import { path, merge, dry_run } => {
            commands::import::execute(path, merge, dry_run)?;
        }
        Commands::Config { action } => {
            match action {
                ConfigAction::Set { key, value } => {
                    commands::config::execute_set(key, value)?;
                }
                ConfigAction::Get { key } => {
                    commands::config::execute_get(key)?;
                }
                ConfigAction::List => {
                    commands::config::execute_list()?;
                }
                ConfigAction::Reset => {
                    commands::config::execute_reset()?;
                }
                ConfigAction::Path => {
                    commands::config::execute_path()?;
                }
            }
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
        Commands::TagRename { old_tag, new_tag } => {
            commands::tag_rename::execute(old_tag, new_tag)?;
        }
        Commands::TagMerge { tags, into } => {
            commands::tag_merge::execute(tags, into)?;
        }
        Commands::TagDelete { tag, force } => {
            commands::tag_delete::execute(tag, force)?;
        }
        Commands::TagStats { sort, min_count } => {
            commands::tag_stats::execute(sort, min_count)?;
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
