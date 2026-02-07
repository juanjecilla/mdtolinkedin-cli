# AGENTS.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

**mdtolinkedin** is a Rust CLI tool that converts Markdown to LinkedIn-compatible text using Unicode Mathematical Alphanumeric Symbols. LinkedIn doesn't support Markdown natively, so this tool transforms bold/italic formatting into Unicode equivalents that LinkedIn accepts.

## Common Commands

### Build and Run
```bash
# Build the project
cargo build

# Build optimized release binary
cargo build --release

# Run the CLI
cargo run -- [INPUT_FILE]
cargo run -- input.md -o output.txt

# Run with stdin
echo "**bold text**" | cargo run

# Install locally
cargo install --path .
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Code Quality
```bash
# Check code without building
cargo check

# Run linter
cargo clippy

# Format code
cargo fmt

# Check formatting without modifying
cargo fmt -- --check
```

## Architecture Overview

The codebase follows a modular pipeline architecture:

```
Input (file/stdin) → Parser (pulldown-cmark) → Converter (unicode mapping) → Output (file/stdout)
```

### Module Structure

- **`cli.rs`**: Clap-based CLI argument parsing. Defines input/output modes, optional Carbon URL generation, and warning suppression.
- **`cli.rs`**: Clap-based CLI argument parsing. Defines input/output modes, code block handling, optional Carbon URL generation, bullet customization, and warning suppression.
  
- **`io.rs`**: Abstracts I/O operations. Handles reading from stdin or file, writing to stdout or file. All file operations use `PathBuf` for cross-platform compatibility.

- **`converter.rs`**: Core conversion engine. Uses `pulldown_cmark` event stream to parse Markdown AST and transform elements to LinkedIn format. Maintains state for nested formatting contexts (e.g., bold within lists). Can trigger code image rendering when `--code-blocks image` is enabled.

- **`unicode.rs`**: ASCII-to-Unicode character mapping. Implements transformations using Mathematical Alphanumeric Symbols:
  - Bold: `A-Z` → U+1D400–U+1D419, `a-z` → U+1D41A–U+1D433
  - Italic: `A-Z` → U+1D434–U+1D44D, `a-z` → U+1D44E–U+1D467
  - Only letters transform; numbers, punctuation, and emoji preserve unchanged.

- **`lib.rs`**: Library crate exports for reuse in other Rust projects.
- **`main.rs`**: Entry point. Orchestrates CLI parsing, I/O, conversion, and character limit warning (3000 chars).

### Key Design Patterns

1. **Event-Driven Parsing**: Uses `pulldown_cmark::Parser` iterator to process Markdown as event stream (`Event::Start`, `Event::End`, `Event::Text`). This allows stateful transformation without building full AST.

2. **Context Stacking**: Converter maintains state stack to handle nested formatting (e.g., `**bold _italic_**`). Apply transformations based on active context when processing text events.

3. **Zero-Copy Where Possible**: Use `&str` slices and `String::push_str` to minimize allocations during conversion.

## Markdown Transformation Rules

| Markdown | LinkedIn Output |
|----------|----------------|
| `# Header` | Bold text (no # symbol) |
| `**bold**` | Unicode bold characters |
| `*italic*` | Unicode italic characters |
| `***bold italic***` | Unicode bold italic characters |
| `- item` | `• item` (bullet symbol) |
| `1. item` | `1. item` (preserved) |
| `> quote` | Italic text |
| `[text](url)` | `text (url)` |
| `` `code` `` | Remove backticks, plain text |
| ``````` code ``````` | Omit, keep as text, or generate Carbon URL (with `--code-blocks`) |

## Testing Strategy

- **Unit tests in `unicode.rs`**: Verify each ASCII character maps to correct Unicode codepoint.
- **Unit tests in `converter.rs`**: Test each Markdown element conversion independently (headers, bold, italic, lists, links, blockquotes).
- **Integration tests**: Validate CLI argument parsing, stdin/stdout piping, file I/O, and character limit warnings.
- **Fixture tests**: Snapshot-style fixtures in `tests/fixtures/` verified by converter unit tests.
  - Regenerate fixtures with `scripts/update_fixtures.py`.

When adding tests, place unit tests in the same file using `#[cfg(test)]` module. Integration tests go in `tests/` directory.

## Character Limit Handling

LinkedIn posts have a 3000 character limit. The CLI:
1. Counts output characters using `.chars().count()` (not bytes)
2. Prints warning to stderr if >3000
3. Suppresses warning with `--no-warn` flag
4. Warnings use stderr to avoid polluting piped output
5. Custom thresholds use `--max-chars <N>`

## Dependencies

- **clap v4**: CLI parsing with derive macros. Use `#[command]` and `#[arg]` attributes for configuration.
- **pulldown-cmark v0.10**: CommonMark-compliant Markdown parser. Event-based streaming API, not AST-based.
- **syntect v5**: Syntax highlighting for code images.
- **resvg v0.35**: SVG rendering to PNG for code images.

When adding dependencies, prefer stable crates with active maintenance and minimal transitive dependencies.

## File References

For implementation guidance, see:
- `docs/ARCHITECTURE.md`: Detailed module design with code snippets
- `docs/PROJECT_CONTEXT.md`: Unicode mappings and formatting rules
- `README.md`: Task breakdown for sequential implementation
