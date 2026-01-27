# Task 4: CLI and I/O Module

**Phase:** 3  
**Estimated Effort:** 1 hour  
**Dependencies:** Task 3 (Markdown Parser)

---

## Context

This task wires together the CLI interface using `clap` and the I/O handling (stdin/file input, stdout/file output).

## Goal

Implement `src/cli.rs`, `src/io.rs`, and wire everything in `src/main.rs`.

---

## Implementation Steps

### Step 4.1: Implement CLI Module

**File:** `src/cli.rs`

```rust
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "mdtolinkedin")]
#[command(version, about = "Convert Markdown to LinkedIn-compatible text")]
pub struct Cli {
    /// Input Markdown file (reads from stdin if omitted)
    pub input: Option<PathBuf>,

    /// Output file (writes to stdout if omitted)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Generate Carbon.now.sh URLs for code blocks
    #[arg(long)]
    pub carbon: bool,

    /// Suppress character limit warning
    #[arg(long)]
    pub no_warn: bool,
}
```

### Step 4.2: Implement I/O Module

**File:** `src/io.rs`

```rust
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

/// Read input from file or stdin.
pub fn read_input(path: Option<&PathBuf>) -> io::Result<String> {
    match path {
        Some(p) => fs::read_to_string(p),
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        }
    }
}

/// Write output to file or stdout.
pub fn write_output(path: Option<&PathBuf>, content: &str) -> io::Result<()> {
    match path {
        Some(p) => fs::write(p, content),
        None => {
            io::stdout().write_all(content.as_bytes())?;
            io::stdout().write_all(b"\n")?;
            Ok(())
        }
    }
}
```

### Step 4.3: Wire Main Entry Point

**File:** `src/main.rs`

```rust
mod cli;
mod io;
mod converter;
mod unicode;

use clap::Parser;
use cli::Cli;
use converter::ConvertOptions;

fn main() {
    let args = Cli::parse();

    // Read input
    let input = match io::read_input(args.input.as_ref()) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    // Convert
    let options = ConvertOptions {
        use_carbon: args.carbon,
    };
    let output = converter::convert(&input, &options);

    // Character count warning
    let char_count = output.chars().count();
    if !args.no_warn && char_count > 3000 {
        eprintln!(
            "⚠️  Warning: Output is {} characters (LinkedIn limit: 3000)",
            char_count
        );
    }

    // Write output
    if let Err(e) = io::write_output(args.output.as_ref(), &output) {
        eprintln!("Error writing output: {}", e);
        std::process::exit(1);
    }
}
```

### Step 4.4: Add Integration Tests

**File:** `tests/integration.rs`

```rust
use std::process::Command;
use std::io::Write;

#[test]
fn test_file_input_stdout() {
    // Create temp file
    let input = "**bold** text";
    let temp_path = "/tmp/mdtolinkedin_test.md";
    std::fs::write(temp_path, input).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_path])
        .output()
        .expect("Failed to run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("𝗯𝗼𝗹𝗱"));

    std::fs::remove_file(temp_path).ok();
}

#[test]
fn test_stdin_input() {
    use std::process::Stdio;

    let mut child = Command::new("cargo")
        .args(["run", "--"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn");

    let stdin = child.stdin.as_mut().unwrap();
    stdin.write_all(b"*italic*").unwrap();
    drop(stdin);

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("𝘪𝘵𝘢𝘭𝘪𝘤"));
}

#[test]
fn test_file_output() {
    let input = "# Header";
    let input_path = "/tmp/mdtolinkedin_in.md";
    let output_path = "/tmp/mdtolinkedin_out.txt";
    std::fs::write(input_path, input).unwrap();

    Command::new("cargo")
        .args(["run", "--", input_path, "-o", output_path])
        .output()
        .expect("Failed to run");

    let result = std::fs::read_to_string(output_path).unwrap();
    assert!(result.contains("𝗛𝗲𝗮𝗱𝗲𝗿"));

    std::fs::remove_file(input_path).ok();
    std::fs::remove_file(output_path).ok();
}

#[test]
fn test_character_warning() {
    // Create a long input
    let input = "a".repeat(3001);
    let temp_path = "/tmp/mdtolinkedin_long.md";
    std::fs::write(temp_path, &input).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_path])
        .output()
        .expect("Failed to run");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Warning"));
    assert!(stderr.contains("3001"));

    std::fs::remove_file(temp_path).ok();
}

#[test]
fn test_no_warn_flag() {
    let input = "a".repeat(3001);
    let temp_path = "/tmp/mdtolinkedin_nowarn.md";
    std::fs::write(temp_path, &input).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_path, "--no-warn"])
        .output()
        .expect("Failed to run");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains("Warning"));

    std::fs::remove_file(temp_path).ok();
}
```

### Step 4.5: Test All Modes

```bash
# Build release
cargo build --release

# Test file → stdout
echo "**bold**" > test.md
./target/release/mdtolinkedin test.md

# Test stdin → stdout
echo "*italic*" | ./target/release/mdtolinkedin

# Test file → file
./target/release/mdtolinkedin test.md -o out.txt
cat out.txt

# Cleanup
rm test.md out.txt
```

---

## Definition of Done

- [ ] `mdtolinkedin input.md` reads file, outputs to stdout.
- [ ] `cat file.md | mdtolinkedin` reads stdin, outputs to stdout.
- [ ] `mdtolinkedin input.md -o output.txt` writes to file.
- [ ] `--no-warn` suppresses character limit warning.
- [ ] Warning appears on stderr when output > 3000 chars.
- [ ] All integration tests pass.
- [ ] `cargo build --release` succeeds.

---

## Files Changed

| File | Change |
|------|--------|
| `src/cli.rs` | Implemented |
| `src/io.rs` | Implemented |
| `src/main.rs` | Updated |
| `tests/integration.rs` | Created |
