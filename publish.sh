#!/bin/bash
# Publish all UCP crates to crates.io in dependency order
# Usage: ./publish.sh [--dry-run]

set -e

DRY_RUN=""
if [ "$1" == "--dry-run" ]; then
    DRY_RUN="--dry-run"
    echo "Running in dry-run mode (no actual publishing)"
fi

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Crates in dependency order (from least to most dependent)
CRATES=(
    "crates/ucm-core"
    "crates/ucp-observe"
    "crates/translators/markdown"
    "crates/translators/html"
    "crates/ucm-engine"
    "crates/ucl-parser"
    "crates/ucp-llm"
    "crates/ucp-agent"
    "crates/ucp-cli"
)

echo "================================"
echo "UCP Crates Publishing Script"
echo "================================"
echo ""

# Check if logged in to crates.io
echo "Checking crates.io login status..."
if ! cargo whoami &> /dev/null; then
    echo -e "${RED}Error: Not logged in to crates.io${NC}"
    echo "Run: cargo login <YOUR_CRATES_IO_TOKEN>"
    exit 1
fi

echo -e "${GREEN}Logged in as: $(cargo whoami)${NC}"
echo ""

# Verify clean git state
if [ -n "$(git status --porcelain)" ]; then
    echo -e "${YELLOW}Warning: Uncommitted changes detected${NC}"
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Verify on correct branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "prepare-cratesio-publish" ] && [ "$CURRENT_BRANCH" != "main" ]; then
    echo -e "${YELLOW}Warning: Not on expected branch (current: $CURRENT_BRANCH)${NC}"
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Publish each crate
echo "Publishing crates in order..."
echo ""

for crate in "${CRATES[@]}"; do
    crate_name=$(basename "$crate")
    echo -e "${YELLOW}Publishing $crate_name...${NC}"
    
    if [ -n "$DRY_RUN" ]; then
        echo "  Running: cargo publish -p $crate_name --dry-run"
        if cargo publish -p "$crate_name" --dry-run; then
            echo -e "  ${GREEN}✓ $crate_name dry-run successful${NC}"
        else
            echo -e "  ${RED}✗ $crate_name dry-run failed${NC}"
            exit 1
        fi
    else
        echo "  Running: cargo publish -p $crate_name"
        if cargo publish -p "$crate_name"; then
            echo -e "  ${GREEN}✓ $crate_name published successfully${NC}"
        else
            echo -e "  ${RED}✗ $crate_name failed to publish${NC}"
            exit 1
        fi
        
        # Wait a bit between publishes to let crates.io index update
        echo "  Waiting 10 seconds for crates.io to index..."
        sleep 10
    fi
    echo ""
done

echo "================================"
if [ -n "$DRY_RUN" ]; then
    echo -e "${GREEN}All dry-runs passed!${NC}"
    echo "Run without --dry-run to actually publish."
else
    echo -e "${GREEN}All crates published successfully!${NC}"
fi
echo "================================"
