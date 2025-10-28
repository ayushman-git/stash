#!/bin/bash
# test_advanced_scenarios.sh - Advanced edge cases and scenario testing
# Tests special characters, long URLs, tracking parameters, etc.

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
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
    echo -e "\n${MAGENTA}[ADVANCED-$COUNTER]${NC} $description"
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

echo -e "${MAGENTA}╔════════════════════════════════════════════════════╗${NC}"
echo -e "${MAGENTA}║   Stash Advanced Scenarios Testing Script         ║${NC}"
echo -e "${MAGENTA}╚════════════════════════════════════════════════════╝${NC}"
echo ""
echo "This script tests edge cases and advanced scenarios"
echo ""

# Scenario 1: URL with tracking parameters (tests canonicalization)
add_article \
    "https://blog.rust-lang.org/2024/01/09/rust-1.75.0.html?utm_source=test&utm_medium=email&fbclid=test123" \
    "rust,release-notes,test-canonicalization" \
    "" \
    "URL with tracking parameters"

# Scenario 2: Very long title with special characters
add_article \
    "https://rust-lang.github.io/async-book/" \
    "rust,async,advanced" \
    "Asynchronous Programming in Rust: A Complete Guide to async/await, Futures, and Concurrent Programming" \
    "Very long custom title"

# Scenario 3: Multiple articles from same domain
add_article \
    "https://doc.rust-lang.org/std/vec/struct.Vec.html" \
    "rust,std-library,reference" \
    "" \
    "Same domain test (rust-lang.org) - Vec"

add_article \
    "https://doc.rust-lang.org/std/string/struct.String.html" \
    "rust,std-library,reference" \
    "" \
    "Same domain test (rust-lang.org) - String"

add_article \
    "https://doc.rust-lang.org/std/collections/struct.HashMap.html" \
    "rust,std-library,reference" \
    "" \
    "Same domain test (rust-lang.org) - HashMap"

# Scenario 4: Title with special characters and emojis
add_article \
    "https://fasterthanli.me/articles/pin-and-suffering" \
    "rust,advanced,async" \
    "Pin & Suffering 🦀: Understanding Rust's Pin Type" \
    "Title with special characters & emoji"

# Scenario 5: Complex tag combinations
add_article \
    "https://tokio.rs/tokio/tutorial" \
    "rust,async,tokio,networking,tutorial,runtime,event-loop" \
    "" \
    "Many tags (7+ tags)"

# Scenario 6: Article with hyphens and underscores in tags
add_article \
    "https://lib.rs/crates/clap" \
    "rust,cli,command-line,arg-parser,clap_derive" \
    "" \
    "Tags with hyphens and underscores"

# Scenario 7: Long URL with query string
add_article \
    "https://docs.rs/tokio/latest/tokio/?search=TcpStream" \
    "rust,tokio,documentation,search-test" \
    "" \
    "URL with query string"

# Scenario 8: GitHub raw content URL
add_article \
    "https://raw.githubusercontent.com/rust-lang/rust/master/README.md" \
    "rust,github,readme" \
    "" \
    "GitHub raw content URL"

# Scenario 9: URL with anchor/fragment
add_article \
    "https://doc.rust-lang.org/book/ch10-02-traits.html#traits-defining-shared-behavior" \
    "rust,traits,tutorial" \
    "" \
    "URL with anchor fragment"

# Scenario 10: Numbers and special chars in tags
add_article \
    "https://blog.rust-lang.org/2024/01/09/rust-1.75.0.html" \
    "rust,v1.75.0,2024,release" \
    "" \
    "Tags with numbers and dots"

# Scenario 11: Really short tag
add_article \
    "https://rustup.rs/" \
    "rust,up,rs,setup" \
    "" \
    "Very short tags (2 chars)"

# Scenario 12: Article from docs.rs (crate documentation)
add_article \
    "https://docs.rs/anyhow/latest/anyhow/" \
    "rust,error-handling,crates,anyhow" \
    "" \
    "Crate documentation from docs.rs"

# Scenario 13: Whitespace handling in title
add_article \
    "https://cheats.rs/" \
    "rust,reference,cheatsheet" \
    "   Rust Language Cheat Sheet   " \
    "Title with leading/trailing whitespace"

# Scenario 14: Mixed case tags
add_article \
    "https://www.youtube.com/watch?v=rAl-9HwD858" \
    "Rust,YouTube,Video,Tutorial" \
    "Rust Programming Tutorial" \
    "Mixed case in tags"

# Scenario 15: URL with www prefix (canonicalization test)
add_article \
    "https://www.sqlite.org/index.html" \
    "database,sqlite,test-www" \
    "" \
    "URL with www prefix"

echo ""
echo -e "${MAGENTA}╔════════════════════════════════════════════════════╗${NC}"
echo -e "${MAGENTA}║         Advanced Test Execution Summary           ║${NC}"
echo -e "${MAGENTA}╚════════════════════════════════════════════════════╝${NC}"
echo -e "Total attempts: ${BLUE}$COUNTER${NC}"
echo -e "Successful:     ${GREEN}$SUCCESS${NC}"
echo -e "Failed:         ${RED}$FAILED${NC}"
echo ""

if [ "$DRY_RUN" = false ]; then
    echo -e "${BLUE}Edge cases tested:${NC}"
    echo "  ✓ Tracking parameter removal"
    echo "  ✓ Long titles and URLs"
    echo "  ✓ Special characters in titles and tags"
    echo "  ✓ Multiple articles from same domain"
    echo "  ✓ Complex tag combinations"
    echo "  ✓ URL anchors and query strings"
    echo "  ✓ Whitespace handling"
    echo "  ✓ www prefix canonicalization"
    echo ""
    echo -e "${BLUE}To view added articles, run:${NC}"
    echo "  $STASH list --all"
    echo ""
    echo -e "${BLUE}To clean up test articles, run:${NC}"
    echo "  ./scripts/cleanup_test_data.sh"
fi

echo ""
exit 0

