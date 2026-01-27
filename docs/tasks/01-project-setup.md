# Task 1: Project Setup

**Phase:** 1  
**Estimated Effort:** 30 minutes  
**Dependencies:** None (starting point)

---

## Context

This task initializes the Rust project with all necessary dependencies and configuration. The result is a buildable skeleton.

## Goal

Create a new Rust CLI project with `clap` and `pulldown-cmark` dependencies, ready for feature development.

---

## Implementation Steps

### Step 1.1: Create Project

```bash
cargo new mdtolinkedin
cd mdtolinkedin
```

### Step 1.2: Add Dependencies

**File:** `Cargo.toml`

```toml
[package]
name = "mdtolinkedin"
version = "0.1.0"
edition = "2021"
description = "Convert Markdown to LinkedIn-compatible text"
license = "MIT"
repository = "https://github.com/<username>/mdtolinkedin"

[dependencies]
clap = { version = "4", features = ["derive"] }
pulldown-cmark = "0.10"
```

### Step 1.3: Create Module Structure

Create empty module files:

```bash
touch src/cli.rs src/io.rs src/converter.rs src/unicode.rs
```

**File:** `src/main.rs`

```rust
mod cli;
mod io;
mod converter;
mod unicode;

fn main() {
    println!("mdtolinkedin - coming soon!");
}
```

### Step 1.4: Verify Build

```bash
cargo build
cargo run
```

Expected output: `mdtolinkedin - coming soon!`

### Step 1.5: Initialize Git

```bash
git init
echo "target/" > .gitignore
git add .
git commit -m "chore: initial project setup"
```

### Step 1.6: Add README

**File:** `README.md`

```markdown
# mdtolinkedin

Convert Markdown to LinkedIn-compatible text using Unicode formatting.

## Installation

```bash
cargo install mdtolinkedin
```

## Usage

```bash
mdtolinkedin input.md
cat input.md | mdtolinkedin
mdtolinkedin input.md -o output.txt
```

## License

MIT
```

---

## Definition of Done

- [ ] `cargo build` succeeds without errors.
- [ ] `cargo run` prints placeholder message.
- [ ] All module files exist (`cli.rs`, `io.rs`, `converter.rs`, `unicode.rs`).
- [ ] Git repository initialized with `.gitignore`.
- [ ] `README.md` exists with basic usage info.

---

## Files Created

| File | Description |
|------|-------------|
| `Cargo.toml` | Project manifest with dependencies |
| `src/main.rs` | Entry point |
| `src/cli.rs` | CLI module (empty) |
| `src/io.rs` | I/O module (empty) |
| `src/converter.rs` | Converter module (empty) |
| `src/unicode.rs` | Unicode module (empty) |
| `README.md` | Project readme |
| `.gitignore` | Git ignore file |
