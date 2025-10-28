#!/bin/bash
# test_bulk_add.sh - Comprehensive bulk testing script for stash
# Adds ~15 articles covering various scenarios with real URLs

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if stash binary exists
if ! command -v stash &> /dev/null; then
    if [ -f "./target/release/stash" ]; then
        STASH="./target/release/stash"
    elif [ -f "./target/debug/stash" ]; then
        STASH="./target/debug/stash"
    else
        echo -e "${RED}Error: stash binary not found${NC}"
        echo "Please build the project first: cargo build --release"
        exit 1
    fi
else
    STASH="stash"
fi

DRY_RUN=false
if [ "$1" == "--dry-run" ]; then
    DRY_RUN=true
    echo -e "${YELLOW}Running in dry-run mode (no articles will be added)${NC}\n"
fi

COUNTER=0
SUCCESS=0
FAILED=0

add_article() {
    local url="$1"
    local tags="$2"
    local title="$3"
    local description="$4"
    
    COUNTER=$((COUNTER + 1))
    echo -e "\n${BLUE}[$COUNTER]${NC} $description"
    echo -e "  URL: $url"
    
    if [ -n "$tags" ]; then
        echo -e "  Tags: ${YELLOW}$tags${NC}"
    fi
    
    if [ -n "$title" ]; then
        echo -e "  Title: $title"
    fi
    
    if [ "$DRY_RUN" = true ]; then
        echo -e "  ${YELLOW}[DRY-RUN] Would add article${NC}"
        SUCCESS=$((SUCCESS + 1))
        return
    fi
    
    local cmd="$STASH add \"$url\""
    
    if [ -n "$tags" ]; then
        cmd="$cmd --tags $tags"
    fi
    
    if [ -n "$title" ]; then
        cmd="$cmd --title \"$title\""
    fi
    
    if eval $cmd 2>&1; then
        echo -e "  ${GREEN}✓ Added successfully${NC}"
        SUCCESS=$((SUCCESS + 1))
    else
        local exit_code=$?
        if [ $exit_code -eq 3 ]; then
            echo -e "  ${YELLOW}⚠ Already exists (duplicate)${NC}"
            SUCCESS=$((SUCCESS + 1))
        else
            echo -e "  ${RED}✗ Failed${NC}"
            FAILED=$((FAILED + 1))
        fi
    fi
}

echo -e "${GREEN}╔════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║   Stash Bulk Testing Script - Comprehensive       ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════╝${NC}"
echo ""
echo "This script will add ~15 test articles with various scenarios"
echo ""

# Scenario 1: Basic Rust documentation
add_article \
    "https://doc.rust-lang.org/book/ch01-00-getting-started.html" \
    "rust,tutorial,documentation" \
    "" \
    "Basic Rust documentation article"

# Scenario 2: GitHub repository
add_article \
    "https://github.com/BurntSushi/ripgrep" \
    "rust,cli,tools" \
    "" \
    "Popular Rust CLI tool repository"

# Scenario 3: Rust blog with custom title
add_article \
    "https://blog.rust-lang.org/2024/02/08/Rust-1.76.0.html" \
    "rust,release-notes" \
    "Rust 1.76.0 Release" \
    "Blog post with custom title"

# Scenario 4: Multiple tags, complex categorization
add_article \
    "https://sqlite.org/quirks.html" \
    "database,sqlite,documentation,reference" \
    "" \
    "SQLite documentation with multiple tags"

# Scenario 5: CLI tool documentation
add_article \
    "https://doc.rust-lang.org/cargo/guide/" \
    "rust,cargo,build-tools" \
    "" \
    "Cargo guide documentation"

# Scenario 6: Blog post about performance
add_article \
    "https://nnethercote.github.io/2024/03/06/how-to-speed-up-the-rust-compiler-in-march-2024.html" \
    "rust,performance,compiler" \
    "" \
    "Technical blog about Rust compiler"

# Scenario 7: Database best practices
add_article \
    "https://www.sqlite.org/bestpractice.html" \
    "database,sqlite,best-practices" \
    "" \
    "SQLite best practices"

# Scenario 8: Testing tutorial
add_article \
    "https://doc.rust-lang.org/book/ch11-00-testing.html" \
    "rust,testing,tutorial" \
    "Testing in Rust" \
    "Rust testing chapter with custom title"

# Scenario 9: Article without tags
add_article \
    "https://fasterthanli.me/articles/aiming-for-correctness-with-types" \
    "" \
    "" \
    "Article without tags to test default behavior"

# Scenario 10: CLI design article
add_article \
    "https://clig.dev/" \
    "cli,design,best-practices" \
    "Command Line Interface Guidelines" \
    "CLI design guidelines"

# Scenario 11: Rust error handling
add_article \
    "https://doc.rust-lang.org/book/ch09-00-error-handling.html" \
    "rust,error-handling,tutorial" \
    "" \
    "Error handling in Rust"

# Scenario 12: GitHub discussion/issue
add_article \
    "https://github.com/rust-lang/rust/issues/44265" \
    "rust,async,futures" \
    "" \
    "GitHub issue for testing"

# Scenario 13: Rust patterns and idioms
add_article \
    "https://rust-unofficial.github.io/patterns/" \
    "rust,patterns,best-practices,reference" \
    "" \
    "Rust design patterns book"

# Scenario 14: Terminal UI library
add_article \
    "https://ratatui.rs/introduction/" \
    "rust,tui,ui-library" \
    "" \
    "Ratatui TUI library documentation"

# Scenario 15: Duplicate URL (intentional)
add_article \
    "https://doc.rust-lang.org/book/ch01-00-getting-started.html" \
    "duplicate-test" \
    "This should be detected as duplicate" \
    "Intentional duplicate to test deduplication"

# Scenario 16: Article from tech blog
add_article \
    "https://without.boats/blog/pin/" \
    "rust,advanced,async" \
    "" \
    "Advanced Rust concepts blog"

echo ""
echo -e "${GREEN}╔════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║              Test Execution Summary                ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════╝${NC}"
echo -e "Total attempts: ${BLUE}$COUNTER${NC}"
echo -e "Successful:     ${GREEN}$SUCCESS${NC}"
echo -e "Failed:         ${RED}$FAILED${NC}"
echo ""

if [ "$DRY_RUN" = false ]; then
    echo -e "${BLUE}To view added articles, run:${NC}"
    echo "  $STASH list --all"
    echo ""
    echo -e "${BLUE}To clean up test articles, run:${NC}"
    echo "  ./scripts/cleanup_test_data.sh"
fi

echo ""
exit 0

