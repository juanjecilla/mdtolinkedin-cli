# Architecture: mdtolinkedin CLI

## Module Overview

```
src/
├── lib.rs           # Library crate exports
├── main.rs          # Entry point, CLI parsing
├── cli.rs           # Clap CLI definition
├── io.rs            # Input/output handling
├── converter.rs     # Markdown → LinkedIn conversion
├── unicode.rs       # ASCII → Unicode math mappings
└── carbon.rs        # Carbon.now.sh URL generation
```

## Data Flow

```
┌─────────────┐     ┌──────────────┐     ┌──────────────┐     ┌─────────────┐
│   Input     │ ──▶ │   Parser     │ ──▶ │  Converter   │ ──▶ │   Output    │
│ (file/stdin)│     │ (pulldown)   │     │  (unicode)   │     │(file/stdout)│
└─────────────┘     └──────────────┘     └──────────────┘     └─────────────┘
```

## Module Details

### `cli.rs`

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "mdtolinkedin")]
#[command(about = "Convert Markdown to LinkedIn-compatible text")]
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

### `unicode.rs`

Core function to transform ASCII to Unicode math characters:

```rust
pub fn to_bold(text: &str) -> String {
    text.chars().map(|c| match c {
        'A'..='Z' => char::from_u32(0x1D400 + (c as u32 - 'A' as u32)).unwrap_or(c),
        'a'..='z' => char::from_u32(0x1D41A + (c as u32 - 'a' as u32)).unwrap_or(c),
        _ => c,
    }).collect()
}

pub fn to_italic(text: &str) -> String {
    text.chars().map(|c| match c {
        'A'..='Z' => char::from_u32(0x1D434 + (c as u32 - 'A' as u32)).unwrap_or(c),
        'a'..='z' => char::from_u32(0x1D44E + (c as u32 - 'a' as u32)).unwrap_or(c),
        _ => c,
    }).collect()
}
```

### `converter.rs`

Uses `pulldown_cmark` to parse and convert:

```rust
use pulldown_cmark::{Event, Parser, Tag};

pub fn convert(markdown: &str, use_carbon: bool) -> String {
    let parser = Parser::new(markdown);
    let mut output = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { .. }) => { /* start bold */ }
            Event::End(Tag::Heading { .. }) => { /* end bold, add newline */ }
            Event::Start(Tag::Strong) => { /* start bold */ }
            Event::End(Tag::Strong) => { /* end bold */ }
            Event::Start(Tag::Emphasis) => { /* start italic */ }
            Event::End(Tag::Emphasis) => { /* end italic */ }
            Event::Start(Tag::List(_)) => { /* prepare list */ }
            Event::Start(Tag::Item) => { output.push_str("• "); }
            Event::Start(Tag::BlockQuote(_)) => { /* start italic */ }
            Event::Text(text) => { /* transform based on context */ }
            Event::Code(code) => { /* inline code: just text */ }
            Event::Start(Tag::CodeBlock(_)) => { 
                if use_carbon { /* generate URL */ } 
                else { /* skip */ }
            }
            Event::Start(Tag::Link { dest_url, .. }) => { /* store URL */ }
            Event::End(Tag::Link { .. }) => { /* append (url) */ }
            _ => {}
        }
    }
    output
}
```

### `io.rs`

Handle input/output:

```rust
use std::io::{self, Read, Write};
use std::fs;
use std::path::PathBuf;

pub fn read_input(path: Option<&PathBuf>) -> io::Result<String> {
    match path {
        Some(p) => fs::read_to_string(p),
        None => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            Ok(buf)
        }
    }
}

pub fn write_output(path: Option<&PathBuf>, content: &str) -> io::Result<()> {
    match path {
        Some(p) => fs::write(p, content),
        None => {
            io::stdout().write_all(content.as_bytes())?;
            Ok(())
        }
    }
}
```

### `main.rs`

```rust
mod cli;
mod io;
mod converter;
mod unicode;

use clap::Parser;
use cli::Cli;

fn main() {
    let args = Cli::parse();

    let input = io::read_input(args.input.as_ref())
        .expect("Failed to read input");

    let output = converter::convert(&input, args.carbon);

    if !args.no_warn && output.chars().count() > 3000 {
        eprintln!("Warning: Output exceeds LinkedIn's 3000 character limit ({} chars)", 
                  output.chars().count());
    }

    io::write_output(args.output.as_ref(), &output)
        .expect("Failed to write output");
}
```

## Testing Strategy

| Module | Test Type | Focus |
|--------|-----------|-------|
| `unicode.rs` | Unit | Each letter maps correctly |
| `converter.rs` | Unit | Each Markdown element converts correctly |
| `main.rs` | Integration | CLI args, piping, file I/O |
