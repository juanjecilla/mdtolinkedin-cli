# Task 5: Release and Distribution

**Phase:** 4  
**Estimated Effort:** 2 hours  
**Dependencies:** Task 4 (CLI and I/O)

---

## Context

This task sets up GitHub Actions for CI/CD and creates a Homebrew formula for distribution.

## Goal

1. Automated testing on PRs.
2. Cross-compiled binaries on release.
3. Homebrew tap for easy installation.

---

## Implementation Steps

### Step 5.1: Create CI Workflow

**File:** `.github/workflows/ci.yml`

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Run tests
        run: cargo test --verbose
      - name: Check formatting
        run: cargo fmt --check
      - name: Clippy
        run: cargo clippy -- -D warnings
```

### Step 5.2: Create Release Workflow

**File:** `.github/workflows/release.yml`

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

permissions:
  contents: write

jobs:
  build:
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
            name: linux-x86_64
          - target: x86_64-apple-darwin
            os: macos-latest
            name: macos-x86_64
          - target: aarch64-apple-darwin
            os: macos-latest
            name: macos-aarch64
          - target: x86_64-pc-windows-msvc
            os: windows-latest
            name: windows-x86_64

    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Build
        run: cargo build --release --target ${{ matrix.target }}

      - name: Package (Unix)
        if: runner.os != 'Windows'
        run: |
          cd target/${{ matrix.target }}/release
          tar -czvf ../../../mdtolinkedin-${{ matrix.name }}.tar.gz mdtolinkedin
          cd ../../..

      - name: Package (Windows)
        if: runner.os == 'Windows'
        run: |
          cd target/${{ matrix.target }}/release
          7z a ../../../mdtolinkedin-${{ matrix.name }}.zip mdtolinkedin.exe
          cd ../../..

      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: mdtolinkedin-${{ matrix.name }}
          path: mdtolinkedin-${{ matrix.name }}.*

  release:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/download-artifact@v4
        with:
          path: artifacts
          merge-multiple: true

      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: artifacts/*
          generate_release_notes: true
```

### Step 5.3: Create Homebrew Tap Repository

Create a new GitHub repository: `<username>/homebrew-tap`

**File:** `Formula/mdtolinkedin.rb`

```ruby
class Mdtolinkedin < Formula
  desc "Convert Markdown to LinkedIn-compatible text"
  homepage "https://github.com/<username>/mdtolinkedin"
  version "0.1.0"

  on_macos do
    on_arm do
      url "https://github.com/<username>/mdtolinkedin/releases/download/v0.1.0/mdtolinkedin-macos-aarch64.tar.gz"
      sha256 "REPLACE_WITH_SHA256"
    end
    on_intel do
      url "https://github.com/<username>/mdtolinkedin/releases/download/v0.1.0/mdtolinkedin-macos-x86_64.tar.gz"
      sha256 "REPLACE_WITH_SHA256"
    end
  end

  on_linux do
    url "https://github.com/<username>/mdtolinkedin/releases/download/v0.1.0/mdtolinkedin-linux-x86_64.tar.gz"
    sha256 "REPLACE_WITH_SHA256"
  end

  def install
    bin.install "mdtolinkedin"
  end

  test do
    assert_match "mdtolinkedin", shell_output("#{bin}/mdtolinkedin --version")
  end
end
```

### Step 5.4: Update README with Installation

**File:** `README.md` (update)

```markdown
## Installation

### Homebrew (macOS/Linux)

```bash
brew tap <username>/tap
brew install mdtolinkedin
```

### Download Binary

Download from [GitHub Releases](https://github.com/<username>/mdtolinkedin/releases).

### Build from Source

```bash
cargo install --git https://github.com/<username>/mdtolinkedin
```
```

### Step 5.5: Create First Release

```bash
# Ensure tests pass
cargo test

# Tag release
git tag -a v0.1.0 -m "Initial release"
git push origin v0.1.0
```

Wait for GitHub Actions to build and publish release.

### Step 5.6: Update Homebrew Formula

After release is published:

1. Download each tarball.
2. Calculate SHA256: `shasum -a 256 mdtolinkedin-*.tar.gz`
3. Update `sha256` values in formula.
4. Push to homebrew-tap repository.

---

## Definition of Done

- [ ] CI workflow runs tests on PRs.
- [ ] Release workflow triggers on tags.
- [ ] Binaries published for Linux, macOS (Intel + ARM), Windows.
- [ ] Homebrew formula works: `brew install <user>/tap/mdtolinkedin`.
- [ ] README includes installation instructions.
- [ ] `mdtolinkedin --version` shows correct version.

---

## Files Changed

| File | Change |
|------|--------|
| `.github/workflows/ci.yml` | Created |
| `.github/workflows/release.yml` | Created |
| `README.md` | Updated |
| (external) `homebrew-tap/Formula/mdtolinkedin.rb` | Created |
