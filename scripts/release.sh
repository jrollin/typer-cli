#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if version argument provided
if [ -z "$1" ]; then
  echo -e "${RED}Error: Version number required${NC}"
  echo "Usage: $0 <version>"
  echo "Example: $0 0.8.0"
  exit 1
fi

VERSION=$1

# Validate version format (semver: X.Y.Z)
if ! echo "$VERSION" | grep -E '^[0-9]+\.[0-9]+\.[0-9]+$' > /dev/null; then
  echo -e "${RED}Error: Version must be in semver format (X.Y.Z)${NC}"
  echo "Example: 0.8.0, 1.0.0, 2.1.3"
  echo "Received: $VERSION"
  exit 1
fi

echo -e "${GREEN}✓${NC} Version format valid: $VERSION"

# Check if tag already exists
if git rev-parse "v$VERSION" >/dev/null 2>&1; then
  echo -e "${RED}Error: Tag v$VERSION already exists${NC}"
  echo "Existing tags:"
  git tag --list | tail -10
  exit 1
fi

echo -e "${GREEN}✓${NC} Tag v$VERSION does not exist"

# Check if working directory is clean
if ! git diff-index --quiet HEAD --; then
  echo -e "${YELLOW}Warning: You have uncommitted changes${NC}"
  echo "Please commit or stash your changes first"
  exit 1
fi

echo -e "${GREEN}✓${NC} Working directory is clean"

# Check required tools
if ! command -v cargo &> /dev/null; then
  echo -e "${RED}Error: cargo not found${NC}"
  exit 1
fi

if ! command -v git-cliff &> /dev/null; then
  echo -e "${YELLOW}git-cliff not found, installing...${NC}"
  cargo install git-cliff
fi

echo ""
echo "Updating version to $VERSION..."
echo ""

# Update Cargo.toml version
echo "→ Updating Cargo.toml"
cargo set-version "$VERSION"

# Update Cargo.lock
echo "→ Updating Cargo.lock"
cargo check --quiet

# Generate CHANGELOG.md
echo "→ Generating CHANGELOG.md"
git cliff -o CHANGELOG.md

# Show what changed
echo ""
echo "Files modified:"
git diff --stat Cargo.toml Cargo.lock CHANGELOG.md

# Commit changes
echo ""
echo "→ Committing changes"
git add Cargo.toml Cargo.lock CHANGELOG.md
git commit -m "chore(release): bump version to $VERSION"

# Create annotated tag
echo "→ Creating tag v$VERSION"
git tag -a "v$VERSION" -m "Release v$VERSION"

# Success message
echo ""
echo -e "${GREEN}🎉 Version bump complete!${NC}"
echo ""
echo "Summary:"
echo "  Version: $VERSION"
echo "  Commit: $(git rev-parse --short HEAD)"
echo "  Tag: v$VERSION"
echo ""
echo -e "${YELLOW}Next step:${NC}"
echo "  git push origin main && git push origin v$VERSION"
echo ""
echo "This will trigger the release workflow automatically."
