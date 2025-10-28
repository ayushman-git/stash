# Stash Testing Scripts

This directory contains shell scripts for bulk testing the stash article manager. These scripts help you quickly populate your stash database with test articles to verify functionality, test performance, and validate edge cases.

## Scripts Overview

| Script | Purpose | Articles Added | Use Case |
|--------|---------|---------------|----------|
| `test_bulk_add.sh` | Main testing script | ~16 | General functionality testing |
| `test_advanced_scenarios.sh` | Edge cases | ~15 | URL canonicalization, special chars |
| `cleanup_test_data.sh` | Cleanup utility | - | Remove test articles |

## Prerequisites

Before running these scripts, ensure:

1. **Stash is built**: Run `cargo build --release` from the project root
2. **Executable permissions**: Run `chmod +x scripts/*.sh` to make scripts executable
3. **Stash is in PATH** (optional): Or the scripts will auto-detect from `./target/release/stash` or `./target/debug/stash`

## Usage

### 1. Basic Bulk Testing (`test_bulk_add.sh`)

Adds ~16 articles covering various real-world scenarios.

```bash
# Run from project root
./scripts/test_bulk_add.sh

# Or with dry-run to see what would be added
./scripts/test_bulk_add.sh --dry-run
```

**Scenarios Covered:**
- ✓ Basic URL additions from various domains
- ✓ Articles with single and multiple tags
- ✓ Custom titles vs auto-extracted titles
- ✓ Duplicate URL detection
- ✓ Articles without tags
- ✓ Various content types (docs, blogs, GitHub)
- ✓ Real URLs from diverse sources:
  - Rust documentation (doc.rust-lang.org)
  - GitHub repositories
  - Technical blogs (fasterthanli.me, nnethercote.github.io)
  - CLI guidelines (clig.dev)
  - Database docs (sqlite.org)
  - UI libraries (ratatui.rs)

**Expected Output:**
```
╔════════════════════════════════════════════════════╗
║   Stash Bulk Testing Script - Comprehensive       ║
╚════════════════════════════════════════════════════╝

[1] Basic Rust documentation article
  URL: https://doc.rust-lang.org/book/ch01-00-getting-started.html
  Tags: rust,tutorial,documentation
  ✓ Added successfully

[2] Popular Rust CLI tool repository
  URL: https://github.com/BurntSushi/ripgrep
  Tags: rust,cli,tools
  ✓ Added successfully

...

╔════════════════════════════════════════════════════╗
║              Test Execution Summary                ║
╚════════════════════════════════════════════════════╝
Total attempts: 16
Successful:     16
Failed:         0
```

### 2. Advanced Scenarios Testing (`test_advanced_scenarios.sh`)

Tests edge cases and advanced features like URL canonicalization, special characters, and complex tag combinations.

```bash
# Run from project root
./scripts/test_advanced_scenarios.sh

# Dry-run mode
./scripts/test_advanced_scenarios.sh --dry-run
```

**Scenarios Covered:**
- ✓ URL tracking parameter removal (`utm_*`, `fbclid`)
- ✓ Very long titles (80+ characters)
- ✓ Special characters in titles (emojis, &, /, etc.)
- ✓ Multiple articles from the same domain
- ✓ Complex tag combinations (7+ tags)
- ✓ Tags with hyphens and underscores
- ✓ URLs with query strings and anchors
- ✓ GitHub raw content URLs
- ✓ Tags with numbers and dots
- ✓ Short tags (2 characters)
- ✓ Whitespace handling in titles
- ✓ Mixed case tags
- ✓ `www` prefix canonicalization

**Example Output:**
```
╔════════════════════════════════════════════════════╗
║   Stash Advanced Scenarios Testing Script         ║
╚════════════════════════════════════════════════════╝

[ADVANCED-1] URL with tracking parameters
  URL: https://blog.rust-lang.org/2024/01/09/rust-1.75.0.html?utm_source=test&...
  Tags: rust,release-notes,test-canonicalization
  ✓ Added successfully

...

Edge cases tested:
  ✓ Tracking parameter removal
  ✓ Long titles and URLs
  ✓ Special characters in titles and tags
  ✓ Multiple articles from same domain
  ✓ Complex tag combinations
  ✓ URL anchors and query strings
  ✓ Whitespace handling
  ✓ www prefix canonicalization
```

### 3. Cleanup Test Data (`cleanup_test_data.sh`)

Utility script to help identify and remove test articles.

```bash
# List test articles without removing
./scripts/cleanup_test_data.sh --list-only

# Interactive cleanup (default)
./scripts/cleanup_test_data.sh

# View help
./scripts/cleanup_test_data.sh --help
```

**Options:**
```
--list-only       Only list test articles without removing
--by-domain       Remove all articles from test domains
--by-tags         Remove articles with test-related tags
--all             Remove all articles (USE WITH CAUTION)
--force           Skip confirmation prompts
--help, -h        Show help message
```

**Example Output:**
```
╔════════════════════════════════════════════════════╗
║         Stash Test Data Cleanup Utility            ║
╚════════════════════════════════════════════════════╝

Listing articles that appear to be from test scripts...

Found 3 article(s) with tag: test-canonicalization
Found 15 article(s) from domain: doc.rust-lang.org
Found 2 article(s) from domain: github.com

Summary:
  Articles with test tags: 3
  Articles from test domains: 48
```

## Verification

After running test scripts, verify the results:

```bash
# View all articles
stash list --all

# View in table format with details
stash list --all --format table

# View as JSON for programmatic analysis
stash list --all --format json

# Count total articles
stash list --all --format ids | wc -l

# Launch interactive TUI
stash tui
```

## Test Scenarios Matrix

### Basic Script Coverage

| Feature | Scenario | Expected Behavior |
|---------|----------|-------------------|
| Basic Add | Rust docs URL | Extracts title, tags applied |
| GitHub URL | Repository link | Recognizes GitHub, adds repo info |
| Custom Title | User-provided title | Overrides auto-extracted title |
| Multiple Tags | 4+ tags | All tags stored correctly |
| No Tags | URL without tags | Article added, no tags |
| Duplicate | Same URL twice | Second attempt shows duplicate warning |
| Various Domains | 10+ different sites | All processed correctly |

### Advanced Script Coverage

| Feature | Scenario | Expected Behavior |
|---------|----------|-------------------|
| URL Canonicalization | Tracking params | Params stripped, canonical URL saved |
| Long Content | 80+ char title | Full title stored |
| Special Characters | Emojis, &, / | Handled correctly |
| Same Domain | 3+ articles | All stored separately |
| Complex Tags | 7+ tags | All tags preserved |
| Tag Formats | Hyphens, underscores | Stored as-is |
| Query Strings | ?search=term | Full URL preserved |
| Anchors | #section | Anchor preserved or handled |
| Whitespace | Leading/trailing spaces | Trimmed correctly |
| www Prefix | www.example.com | Canonicalized |

## Performance Benchmarking

Use these scripts to benchmark stash performance:

```bash
# Time the bulk add operation
time ./scripts/test_bulk_add.sh

# Typical results:
# real    0m25.123s  (~1.5s per article with network fetch)
# user    0m2.456s
# sys     0m1.234s

# Benchmark query performance
time stash list --all
# Target: <100ms
```

## Troubleshooting

### Script Can't Find Stash Binary

```bash
# Make sure stash is built
cargo build --release

# Or add to PATH
export PATH="$PWD/target/release:$PATH"

# Or specify explicitly in script
STASH="./target/release/stash" ./scripts/test_bulk_add.sh
```

### Network Timeouts

If you experience network timeouts:

1. Check your internet connection
2. Some URLs may be temporarily unavailable
3. The script will continue even if individual articles fail
4. Failed articles are counted in the summary

### Permission Denied

```bash
# Make scripts executable
chmod +x scripts/*.sh

# Or run with bash explicitly
bash scripts/test_bulk_add.sh
```

### Database Locked

If you get "database is locked" errors:

1. Close any other stash processes
2. Close the TUI if running
3. Wait a moment and retry

## Customization

### Adding Custom Test URLs

Edit the scripts to add your own test URLs:

```bash
# Add to test_bulk_add.sh
add_article \
    "https://your-custom-url.com/article" \
    "custom,tags" \
    "Optional Custom Title" \
    "Description for test output"
```

### Creating Domain-Specific Tests

Create a new script for your domain:

```bash
cp scripts/test_bulk_add.sh scripts/test_my_domain.sh
# Edit to include only your domain's URLs
```

## Integration with CI/CD

These scripts can be integrated into automated testing:

```bash
# In your CI pipeline
cargo build --release
./scripts/test_bulk_add.sh
stash list --all --format json > test_output.json
# Verify expected count, structure, etc.
```

## Contributing

When adding new test scenarios:

1. Use real, publicly accessible URLs
2. Include diverse content types
3. Document the scenario being tested
4. Update this README with new scenarios

## License

These scripts are part of the Stash project and follow the same MIT license.

---

**Questions or Issues?** [Open an issue](https://github.com/ayushman-git/stash/issues) or refer to the main [README](../README.md).

