#!/bin/bash
# Release helper script for broken-link-checker

set -e

# Get the version from Cargo.toml
VERSION=$(grep "^version" Cargo.toml | head -1 | cut -d'"' -f2)

echo "Current version in Cargo.toml: $VERSION"
echo ""
echo "This will:"
echo "  1. Create a git tag v$VERSION"
echo "  2. Push the tag to GitHub"
echo "  3. Trigger the GitHub Actions workflow to build binaries"
echo ""
read -p "Continue? (y/n) " -n 1 -r
echo

if [[ $REPLY =~ ^[Yy]$ ]]
then
    # Make sure we're on main/master branch
    BRANCH=$(git rev-parse --abbrev-ref HEAD)
    if [[ "$BRANCH" != "main" && "$BRANCH" != "master" ]]; then
        echo "Warning: Not on main/master branch (currently on $BRANCH)"
        read -p "Continue anyway? (y/n) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi

    # Check if tag already exists
    if git rev-parse "v$VERSION" >/dev/null 2>&1; then
        echo "Error: Tag v$VERSION already exists!"
        echo "Please update the version in Cargo.toml"
        exit 1
    fi

    # Create and push tag
    git tag -a "v$VERSION" -m "Release v$VERSION"
    git push origin "v$VERSION"
    
    echo ""
    echo "✓ Tag v$VERSION created and pushed!"
    echo "✓ GitHub Actions will now build the binaries"
    echo "✓ Check: https://github.com/lohit-dev/broken-link-checker/actions"
    echo ""
    echo "The release will be available at:"
    echo "https://github.com/lohit-dev/broken-link-checker/releases/tag/v$VERSION"
fi