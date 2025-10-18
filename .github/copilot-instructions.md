# Stash - Copilot Instructions

## Project Overview
Stash is a local-first Rust CLI for saving and organizing articles. Built for speed (<50ms startup, <100ms queries), keyboard-driven workflows, and offline-first access with SQLite storage.

## 🎓 Learning-First Approach

**CRITICAL: This project is primarily a learning vehicle for mastering Rust.** When providing assistance:

1. **Always explain the "why" before the "how"**
   - Why this particular Rust pattern or idiom?
   - Why this crate over alternatives?
   - Why this architecture decision?

2. **Break down complex Rust concepts**
   - Explain ownership, borrowing, and lifetimes when they appear
   - Clarify trait bounds and generic constraints
   - Describe error handling patterns (`Result`, `Option`, `?` operator)
   - Explain iterator chains and closure syntax

3. **Provide context for Rust-specific choices**
   - Why blocking I/O instead of async? (Simpler mental model for CLI, no runtime overhead)
   - Why `anyhow::Result` over `std::result::Result<T, E>`? (Ergonomic error context chaining)
   - Why threads over async for parallel fetching? (No tokio runtime, stdlib only)

4. **Teach through examples**
   - Show idiomatic Rust patterns in code suggestions
   - Explain common pitfalls (e.g., `String` vs `&str`, `clone()` overuse)
   - Demonstrate Rust's type system benefits

5. **Reference learning resources**
   - Point to relevant chapters in "The Rust Book" when applicable
   - Explain crate documentation patterns
   - Highlight Rust ecosystem conventions

**Goal: After each interaction, the developer should understand not just what code to write, but why it's written that way and how it leverages Rust's unique features.**

## Architecture

### Module Structure
Follow strict separation of concerns (as designed in `docs/Tech.md`):
```
src/
├── main.rs           # CLI routing with clap derive macros only
├── commands/         # Orchestration layer - one file per command
├── db/               # Database schema, queries, models, migrations (refinery)
├── fetch/            # HTTP requests, metadata extraction, URL canonicalization
├── ui/               # Output formatting (tables via comfy-table, JSON, colored terminal)
├── config/           # TOML config loading with XDG paths
└── utils/            # Clipboard, browser launch, validation, blake3 hashing
```

**Critical: Commands orchestrate only—business logic lives in domain modules. Database layer knows nothing about HTTP. Fetch layer knows nothing about database.**

### Data Model
The `Article` schema (see `docs/PRD.md` §2.1) uses:
- **User-facing IDs**: Simple integers (1, 2, 3...) for CLI interaction
- **Internal hash**: Git-style 8-char blake3 hash from canonical URL for deduplication
- **State booleans**: `read`, `archived`, `starred` (no enums)
- **Tags**: Stored as JSON array `["rust", "cli"]` in SQLite TEXT column
- **Soft deletes**: `rm` sets `archived=true`; `rm -f` is permanent deletion

### URL Canonicalization Pipeline
Before storing any URL:
1. Strip tracking params (`utm_*`, `fbclid`, etc.)
2. Normalize: `http`→`https`, remove `www.`, trailing slashes
3. Follow redirects to canonical URL
4. Generate blake3 hash for deduplication check
5. Extract domain for auto-tagging (e.g., `github.com` → `github` tag)

### Content Pipeline
`fetch/` module must implement:
```
Raw HTML → Readability extraction → Clean HTML → html2md → Markdown → Cache in DB
```
Use `readability` + `html2md` crates. Fallback: store raw HTML if extraction fails.

## Development Patterns

### Command Structure Template
All commands in `commands/*.rs` follow:
```rust
pub fn execute(/* args from clap */) -> anyhow::Result<()> {
    // 1. Validate inputs (utils/)
    // 2. Query/mutate via db/
    // 3. Fetch external data if needed (fetch/)
    // 4. Format output (ui/)
    // 5. Return with context: .context("Failed to add article")?
    Ok(())
}
```

### Error Handling Convention
- Use `anyhow::Result` everywhere
- Add context at each layer: `.context("Meaningful message")?`
- Convert to user-friendly messages in `main.rs`
- Exit codes: `0`=success, `1`=user error, `2`=network, `3`=database

### Database Conventions
- Use `rusqlite` with **blocking/sync API** (no async for CLI simplicity)
- All queries use parameterized statements—never string concatenation
- Migrations via `refinery` in `db/migrations/`, forward-only, run at startup
- Default queries: unread + unarchived, sorted by `saved_at DESC`

### Performance Requirements
Keep these targets in mind:
- Startup: <50ms → Minimize dependencies in `main.rs`
- `ls` command: <100ms → Use indexed queries, limit default to 10
- `add` command: <2s with network fetch → Parallelize metadata + content fetching
- Binary size: <10MB → Enable LTO and strip in release profile

## Key Implementation Details

### Parallel Fetching Pattern
When adding articles, fetch metadata (title, description) and content (readable HTML) in parallel using threads, not async:
```rust
use std::thread;
let (meta, content) = (
    thread::spawn(|| fetch_metadata(url)),
    thread::spawn(|| fetch_content(url))
);
```

### Dynamic Query Building
For `ls` and `search` commands, build SQL conditionally based on flags:
- Start with base `WHERE read = 0 AND archived = 0`
- Add filters: `AND tags LIKE '%"rust"%'` for `--tag rust`
- Append `ORDER BY starred DESC, saved_at DESC LIMIT 10`

### Output Format Selection
Support `--format table|json|ids` (see `docs/PRD.md` §4.3):
- **table**: `comfy-table` with colored output (check `NO_COLOR` env)
- **json**: `serde_json` for scripting
- **ids**: One ID per line for piping to `xargs`

### External Tool Integration
- **fzf/skim**: Detect with `which` crate, fallback to `dialoguer` prompts
- **Browser**: Use `open` crate (abstracts `xdg-open`, `open`, `start`)
- **Clipboard**: `arboard` crate for `--from-clipboard` and `--copy` flags

### Configuration Management
- XDG-compliant paths via `directories` crate
- Data: `~/.local/share/stash/articles.db`
- Config: `~/.config/stash/config.toml`
- Priority: CLI flags > env vars > config file > defaults

## Testing Approach
- Unit tests: URL canonicalization, tag parsing, hash generation, query builders
- Integration tests: Use `assert_cmd` + `tempfile` for full command execution with temp DB
- No mocking needed for HTTP—document `--no-fetch` flag for offline testing

## Common Patterns

### Idempotent Operations
- Adding duplicate URL → return existing ID with exit code 3
- Archiving archived article → no-op, no error
- Marking read article as read → no-op

### Graceful Degradation
- Metadata fetch fails → save URL only, set `title = domain`
- fzf missing → use `dialoguer` numbered prompt
- Clipboard unavailable → prompt for manual input

### Tags Validation
- Force lowercase on input
- Regex: `^[a-z0-9-]+$` (alphanumeric + hyphens)
- Reject on validation failure with exit code 2

## What NOT to Do
- ❌ Don't use async—blocking is simpler and faster for CLI
- ❌ Don't bundle fzf/skim—detect at runtime
- ❌ Don't load all articles into memory—stream or paginate
- ❌ Don't over-engineer database—SQLite is sufficient
- ❌ Don't add telemetry or external calls (except user-initiated fetches)

## Command Aliases to Remember
When implementing clap commands:
- `add` has alias `save`
- `ls` has alias `list`
- `open` has alias `o`
- See full reference in `docs/PRD.md` §3

## Resources
- Full command specifications: `docs/PRD.md`
- Technical architecture: `docs/Tech.md`
- Dependencies rationale: `Cargo.toml` with feature notes
