#!/bin/bash
# cleanup_test_data.sh - Clean up test articles added by testing scripts
# Provides safe removal of test data with confirmation prompts

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

echo -e "${YELLOW}╔════════════════════════════════════════════════════╗${NC}"
echo -e "${YELLOW}║         Stash Test Data Cleanup Utility            ║${NC}"
echo -e "${YELLOW}╚════════════════════════════════════════════════════╝${NC}"
echo ""

# Show help if requested
if [ "$1" == "--help" ] || [ "$1" == "-h" ]; then
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --list-only       Only list test articles without removing"
    echo "  --by-domain       Remove all articles from test domains"
    echo "  --by-tags         Remove articles with test-related tags"
    echo "  --all             Remove all articles (USE WITH CAUTION)"
    echo "  --force           Skip confirmation prompts"
    echo "  --help, -h        Show this help message"
    echo ""
    echo "Default behavior (no options): Interactive cleanup by tags"
    exit 0
fi

LIST_ONLY=false
BY_DOMAIN=false
BY_TAGS=true
REMOVE_ALL=false
FORCE=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --list-only)
            LIST_ONLY=true
            shift
            ;;
        --by-domain)
            BY_DOMAIN=true
            BY_TAGS=false
            shift
            ;;
        --by-tags)
            BY_TAGS=true
            BY_DOMAIN=false
            shift
            ;;
        --all)
            REMOVE_ALL=true
            shift
            ;;
        --force)
            FORCE=true
            shift
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            echo "Run with --help for usage information"
            exit 1
            ;;
    esac
done

# Test-related tags used in our testing scripts
TEST_TAGS=(
    "test-canonicalization"
    "test-www"
    "duplicate-test"
    "search-test"
)

# Test domains used in testing scripts
TEST_DOMAINS=(
    "rust-lang.org"
    "doc.rust-lang.org"
    "blog.rust-lang.org"
    "github.com"
    "sqlite.org"
    "fasterthanli.me"
    "nnethercote.github.io"
    "clig.dev"
    "rust-unofficial.github.io"
    "ratatui.rs"
    "without.boats"
    "tokio.rs"
    "lib.rs"
    "docs.rs"
    "rustup.rs"
    "cheats.rs"
)

list_test_articles() {
    echo -e "${BLUE}Listing articles that appear to be from test scripts...${NC}"
    echo ""
    
    # Get all articles in JSON format
    local articles=$($STASH list --all --format json 2>/dev/null || echo "[]")
    
    if [ "$articles" == "[]" ]; then
        echo -e "${YELLOW}No articles found in stash${NC}"
        return 1
    fi
    
    # Count articles for different criteria
    local tag_matches=0
    local domain_matches=0
    
    # Check by tags
    for tag in "${TEST_TAGS[@]}"; do
        local count=$(echo "$articles" | grep -c "\"$tag\"" || true)
        if [ $count -gt 0 ]; then
            echo -e "${YELLOW}Found $count article(s) with tag: $tag${NC}"
            tag_matches=$((tag_matches + count))
        fi
    done
    
    # Check by domains
    for domain in "${TEST_DOMAINS[@]}"; do
        local count=$(echo "$articles" | grep -c "\"$domain\"" || true)
        if [ $count -gt 0 ]; then
            echo -e "${BLUE}Found $count article(s) from domain: $domain${NC}"
            domain_matches=$((domain_matches + count))
        fi
    done
    
    echo ""
    echo -e "${GREEN}Summary:${NC}"
    echo -e "  Articles with test tags: ${YELLOW}$tag_matches${NC}"
    echo -e "  Articles from test domains: ${BLUE}$domain_matches${NC}"
    
    return 0
}

confirm_action() {
    local message="$1"
    
    if [ "$FORCE" = true ]; then
        return 0
    fi
    
    echo -e "${YELLOW}$message${NC}"
    read -p "Continue? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo -e "${RED}Cancelled${NC}"
        exit 1
    fi
}

remove_all_articles() {
    echo -e "${RED}WARNING: This will remove ALL articles from stash!${NC}"
    confirm_action "Are you absolutely sure?"
    
    # Get all article IDs
    local ids=$($STASH list --all --format ids 2>/dev/null || echo "")
    
    if [ -z "$ids" ]; then
        echo -e "${YELLOW}No articles to remove${NC}"
        return
    fi
    
    # Convert to array
    IFS=$'\n' read -rd '' -a id_array <<<"$ids" || true
    
    local count=0
    for id in "${id_array[@]}"; do
        if [ -n "$id" ]; then
            $STASH rm "$id" --force &>/dev/null && count=$((count + 1))
        fi
    done
    
    echo -e "${GREEN}Removed $count article(s)${NC}"
}

# Main logic
if [ "$REMOVE_ALL" = true ]; then
    remove_all_articles
    exit 0
fi

# List articles
list_test_articles

if [ "$LIST_ONLY" = true ]; then
    exit 0
fi

echo ""
echo -e "${BLUE}Cleanup Options:${NC}"
echo "  1) Remove articles with test-specific tags"
echo "  2) Show articles from test domains (for manual review)"
echo "  3) Remove ALL articles (dangerous!)"
echo "  4) Cancel"
echo ""

if [ "$FORCE" = false ]; then
    read -p "Choose option (1-4): " -n 1 -r option
    echo
else
    option="1"
fi

case $option in
    1)
        echo -e "${YELLOW}Removing articles with test tags...${NC}"
        # This would require implementing tag-based filtering in stash
        echo -e "${YELLOW}Note: Tag-based removal requires querying by tag${NC}"
        echo -e "${YELLOW}For now, please use 'stash list' to identify and manually remove test articles${NC}"
        ;;
    2)
        echo -e "${BLUE}Articles from test domains:${NC}"
        $STASH list --all --format table | grep -E "$(IFS='|'; echo "${TEST_DOMAINS[*]}")" || echo "No matches found"
        ;;
    3)
        remove_all_articles
        ;;
    4|*)
        echo -e "${YELLOW}Cancelled${NC}"
        exit 0
        ;;
esac

echo ""
echo -e "${GREEN}Cleanup utility finished${NC}"
echo -e "${BLUE}Tip: Use '$STASH list --all' to see all articles${NC}"
echo ""
exit 0

