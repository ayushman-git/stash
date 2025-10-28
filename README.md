# Stash

> Fast, local-first CLI for saving and organizing articles

A Rust-powered article manager designed for speed (<50ms startup, <100ms queries), keyboard-driven workflows, and offline-first access. Built to solve the "save articles but never read them" problem.

## Features

- **Blazing Fast** - Sub-100ms queries, <50ms startup
- **Local-First** - SQLite storage, no cloud dependencies
- **Readable Content** - Automatic extraction of article content as Markdown
- **Smart Organization** - Tags, starring, read/unread states
- **Keyboard-Driven** - Minimal typing with fuzzy picker and TUI mode
- **URL Deduplication** - Automatic canonicalization and tracking param removal
- **Offline Access** - Read articles without internet once saved
- **Beautiful Output** - TUI, colored tables with icons and human-friendly dates

## Installation

### From Source

```bash
git clone https://github.com/ayushman-git/stash.git
cd stash
cargo build --release
cp target/release/stash /usr/local/bin/
```

### Requirements

- Rust 2024 edition or later
- SQLite (bundled)

## Usage

### Quick Start

```bash
# Save an article
stash add "https://example.com/article" --tags rust,cli

# List unread articles
stash list

# Open with fuzzy picker
stash pick

# Open specific article
stash open 1

# Mark as read
stash mark-read 1

# Archive (soft delete)
stash rm 1
```

## Commands

### Core Operations

| Command | Alias | Description |
|---------|-------|-------------|
| `add <url>` | `save` | Save article with auto-metadata extraction |
| `list` | `ls` | List articles with filters |
| `open <id>` | `o` | Open article in browser (auto-marks as read) |
| `pick` | - | Interactive fuzzy picker for articles |
| `tui` | - | Launch Terminal UI for interactive browsing |
| `remove <id>` | `rm` | Archive article (soft delete) |

### Article Management

| Command | Description |
|---------|-------------|
| `edit <id>` | Edit article metadata in $EDITOR |
| `star <id>` | Star important articles |
| `unstar <id>` | Remove star |
| `mark-read <id>` | Mark as read without opening |
| `mark-unread <id>` | Mark as unread |

### Add Command Examples

```bash
# Basic save
stash add "https://example.com/article"

# With tags
stash add "https://example.com/article" --tags rust,programming

# Custom title
stash add "https://example.com/article" --title "My Custom Title"

# Multiple tags
stash add "https://example.com/article" --tags rust,cli,tools

# Skip metadata fetch (offline mode)
stash add "https://example.com/article" --no-fetch
```

### List Command Examples

```bash
# Default: 10 most recent unread articles
stash list

# Show all articles (read + unread)
stash list --all

# Show archived only
stash list --archived

# Limit results
stash list -n 25

# Output formats
stash list --format table   # Default, colored tables
stash list --format json    # For scripting
stash list --format ids     # ID-only for piping
```

### Open Command Examples

```bash
# Open single article
stash open 1

# Open multiple articles
stash open 1,2,3

# Random unread article
stash open --random

# Random from 5 most recent
stash open --random 5

# Open without marking as read
stash open 1 --keep-unread
```

### Edit Command Examples

```bash
# Edit article metadata
stash edit 5

# Opens in $EDITOR with YAML format:
# title: Article Title
# url: https://example.com
# note: Personal notes
# tags:
#   - rust
#   - cli
# starred: false
# read: false
# archived: false

# After saving, shows git-style diff of changes
```

### TUI Command

```bash
# Launch interactive Terminal UI
stash tui

# Keyboard shortcuts:
# j/k or ↑/↓     Navigate articles
# o or Enter     Open article in browser
# r              Mark as read
# u              Mark as unread
# s              Toggle star/favorite
# a              Toggle filter (all/unread)
# R              Refresh list
# q or Esc       Quit
```

### Bulk Operations

```bash
# Mark multiple as read
stash mark-read 1,2,3

# Mark all as read
stash mark-read --all

# Remove multiple articles
stash rm 1,2,3

# Force delete (permanent)
stash rm 1 --force
```

## Architecture

### Module Structure

```
src/
├── main.rs           # CLI routing (clap)
├── commands/         # Command orchestration
│   ├── add.rs
│   ├── edit.rs
│   ├── list.rs
│   ├── open.rs
│   ├── pick.rs
│   └── ...
├── db/               # Database layer
│   ├── schema.rs     # SQLite schema
│   ├── queries.rs    # SQL operations
│   ├── models.rs     # Data types
│   └── migrations/   # Refinery migrations
├── fetch/            # HTTP & content extraction
│   ├── http.rs       # Request handling
│   ├── metadata.rs   # Title, description extraction
│   └── content.rs    # Readability + html2md
├── ui/               # Output formatting
│   ├── formatters.rs # Table/JSON/ID output
│   ├── theme.rs      # Colors and styling
│   └── icons.rs      # Unicode symbols
└── utils/            # Helpers (clipboard, validation, etc.)
```

### Data Flow

```
URL Input → Canonicalization → Deduplication Check
    ↓
Metadata Fetch (parallel) + Content Extraction
    ↓
SQLite Storage (articles.db)
    ↓
UI Formatting → Terminal Output
```

### URL Canonicalization Pipeline

1. Strip tracking params (`utm_*`, `fbclid`, etc.)
2. Normalize: `http`→`https`, remove `www.`, trailing slashes
3. Follow redirects to canonical URL
4. Generate blake3 hash (8-char) for deduplication
5. Extract domain for auto-tagging

### Content Processing

```
Raw HTML → Readability Extraction → Clean HTML → html2md → Markdown → Cache
```

## Data Model

### Article Schema

```sql
CREATE TABLE articles (
    id INTEGER PRIMARY KEY,          -- User-facing ID (1, 2, 3...)
    hash TEXT UNIQUE,                -- blake3 hash for deduplication
    url TEXT NOT NULL,               -- Original URL
    canonical_url TEXT,              -- Cleaned URL
    title TEXT,                      -- Article title
    site TEXT,                       -- Domain name
    description TEXT,                -- Meta description
    content_markdown TEXT,           -- Cached readable content
    saved_at TIMESTAMP,              -- When saved
    read BOOLEAN DEFAULT FALSE,      -- Read state
    archived BOOLEAN DEFAULT FALSE,  -- Soft delete
    starred BOOLEAN DEFAULT FALSE,   -- Starred/important
    tags TEXT,                       -- JSON array: ["rust", "cli"]
    last_opened_at TIMESTAMP         -- Last access time
);
```

### State Transitions

```
┌─────────┐
│ Unread  │ ─────────────┐
│ Active  │              │
└─────────┘              │
     │                   │
     │ open/mark-read    │ rm
     ↓                   ↓
┌─────────┐         ┌──────────┐
│  Read   │────────→│ Archived │
│ Active  │   rm    │          │
└─────────┘         └──────────┘
```

## Configuration

### XDG-Compliant Paths

- **Data**: `~/.local/share/stash/articles.db`
- **Config**: `~/.config/stash/config.toml` _(future)_

### Environment Variables

- `EDITOR` / `VISUAL` - Text editor for `edit` command (auto-detects if unset)
- `NO_COLOR` - Disable colored output
- `BROWSER` - Override default browser for `open` command

## Performance

| Operation | Target | Notes |
|-----------|--------|-------|
| Startup | <50ms | Minimal dependency loading |
| `list` query | <100ms | Indexed queries, default limit 10 |
| `add` with fetch | <2s | Parallel metadata + content fetching |
| Binary size | <10MB | LTO enabled, stripped in release |

## Output Examples

### Table Format (Default)

```
ID  Title                              Site           Tags           Saved      Status
1   ⭐ Building CLI Tools in Rust     rust-lang.org  rust, cli      2 days ago 📖 Unread
2   SQLite Performance Tips           sqlite.org     database, sql  1 week ago ✓ Read
```

### JSON Format (Scripting)

```json
[
  {
    "id": 1,
    "title": "Building CLI Tools in Rust",
    "url": "https://rust-lang.org/article",
    "site": "rust-lang.org",
    "tags": ["rust", "cli"],
    "read": false,
    "starred": true,
    "saved_at": "2025-10-19T10:30:00Z"
  }
]
```

## Development

### Build

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Check without building
cargo check
```

### Project Principles

1. **Separation of Concerns** - Commands orchestrate, domain modules handle logic
2. **No Async** - Blocking/sync API for CLI simplicity and speed
3. **Error Context** - Use `anyhow::Context` at every layer
4. **Idempotent Operations** - Duplicate adds, re-archiving, etc. are no-ops

### Adding a New Command

1. Create `src/commands/your_command.rs`
2. Add `pub fn execute(...) -> anyhow::Result<()>`
3. Wire up in `src/main.rs` with clap derive
4. Follow error handling conventions (context at each layer)

## Testing

```bash
# Unit tests (query builders, URL canonicalization, etc.)
cargo test --lib

# Integration tests (full command execution)
cargo test --test integration_tests
```

## Tech Stack

| Category | Technology | Rationale |
|----------|------------|-----------|
| CLI | `clap` (derive) | Type-safe argument parsing |
| Database | `rusqlite` (sync) | Fast, bundled SQLite |
| HTTP | `reqwest` (blocking) | Reliable with rustls |
| Content | `readability` + `html2md` | Clean article extraction |
| UI | `comfy-table` + `colored` | Beautiful terminal output |
| Hashing | `blake3` | Fast, git-style short hashes |
| Browser | `open` crate | Cross-platform URL opening |

## Contributing

This is primarily a learning project for mastering Rust. Contributions are welcome with a focus on:

- Idiomatic Rust patterns
- Performance optimizations
- Clear error messages
- Documentation improvements

## License

MIT

## Roadmap

- [ ] Config file support (`~/.config/stash/config.toml`)
- [ ] Search command with full-text search
- [ ] Tag management (rename, merge, autocomplete)
- [ ] Export/import (JSON, Markdown)
- [ ] Git-based sync across machines
- [x] TUI mode for richer interaction
- [ ] Browser extensions for one-click saving

---

**Built with 🦀 Rust** | [Report Bug](https://github.com/ayushman-git/stash/issues) | [Request Feature](https://github.com/ayushman-git/stash/issues)
