#!/bin/bash
# Release preparation script for mdtolinkedin v0.1.0

set -e

echo "🚀 Preparing release v0.1.0..."

# Step 1: Set up Rust toolchain (if needed)
echo ""
echo "Step 1: Setting up Rust toolchain..."
if ! rustup default stable 2>/dev/null; then
    echo "⚠️  Warning: Could not set default toolchain. Please run manually:"
    echo "   rustup default stable"
    read -p "Press Enter to continue anyway, or Ctrl+C to abort..."
fi

# Step 2: Verify code quality
echo ""
echo "Step 2: Verifying code quality..."
echo "Running: cargo check"
cargo check || { echo "❌ cargo check failed"; exit 1; }

echo "Running: cargo test"
cargo test || { echo "❌ cargo test failed"; exit 1; }

echo "Running: cargo fmt --check"
cargo fmt --check || {
    echo "⚠️  Code formatting issues found. Running cargo fmt..."
    cargo fmt
}

echo "Running: cargo clippy"
cargo clippy -- -D warnings || { echo "❌ cargo clippy failed"; exit 1; }

# Step 3: Stage files for commit
echo ""
echo "Step 3: Staging files for commit..."
git add src/ tests/ .github/ PROJECT_README.md AGENTS.md Cargo.toml Cargo.lock .gitignore RELEASE.md STATUS.md

# Show what will be committed
echo ""
echo "Files staged for commit:"
git status --short

# Step 4: Create commit
echo ""
read -p "Create commit? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    git commit -m "feat: initial release v0.1.0

- Implement Unicode Bold/Italic conversion
- Add Markdown to LinkedIn converter
- Add CLI with file/stdin I/O support
- Add character limit warning
- Add integration tests
- Add CI/CD workflows
- Update documentation"
    echo "✅ Commit created"
else
    echo "⏭️  Skipped commit"
fi

# Step 5: Create tag
echo ""
read -p "Create version tag v0.1.0? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    git tag -a v0.1.0 -m "Release v0.1.0 - Initial release"
    echo "✅ Tag v0.1.0 created"
else
    echo "⏭️  Skipped tag creation"
fi

# Step 6: Push instructions
echo ""
echo "📤 Next steps:"
echo "1. Push commits:    git push origin master"
echo "2. Push tag:        git push origin v0.1.0"
echo ""
echo "After pushing the tag, GitHub Actions will automatically:"
echo "- Build binaries for Linux, macOS (Intel + ARM), and Windows"
echo "- Create a GitHub Release with all artifacts"
echo ""
echo "Then update the Homebrew formula with SHA256 hashes from the release."
echo "See RELEASE.md for details."
