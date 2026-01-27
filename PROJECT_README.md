# mdtolinkedin

Convert Markdown to LinkedIn-compatible text using Unicode formatting.

LinkedIn doesn't support Markdown natively, so this tool transforms bold/italic formatting into Unicode Mathematical Alphanumeric Symbols that LinkedIn accepts.

## Features

- ✅ Converts Markdown to LinkedIn-compatible text
- ✅ Supports bold (`**text**`) and italic (`*text*`) formatting
- ✅ Transforms headers, lists, links, blockquotes, and more
- ✅ Warns when output exceeds LinkedIn's 3000 character limit
- ✅ Supports stdin/stdout and file I/O
- ✅ Optional Carbon.now.sh URL generation for code blocks

## Installation

### Homebrew (macOS/Linux)

```bash
brew tap juanje/tap
brew install mdtolinkedin
```

### Download Binary

Download pre-built binaries from [GitHub Releases](https://github.com/juanje/mdtolinkedin/releases).

### Build from Source

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build and install
cargo install --git https://github.com/juanje/mdtolinkedin

# Or build locally
git clone https://github.com/juanje/mdtolinkedin.git
cd mdtolinkedin
cargo build --release
```

## Usage

### Basic Usage

```bash
# Read from file, output to stdout
mdtolinkedin input.md

# Read from stdin
cat input.md | mdtolinkedin

# Read from file, write to file
mdtolinkedin input.md -o output.txt
```

### Command-Line Options

```bash
mdtolinkedin [OPTIONS] [INPUT_FILE]

Arguments:
  [INPUT_FILE]    Input Markdown file (reads from stdin if omitted)

Options:
  -o, --output <OUTPUT>    Output file (writes to stdout if omitted)
      --carbon             Generate Carbon.now.sh URLs for code blocks
      --no-warn           Suppress character limit warning
  -h, --help               Print help
  -V, --version            Print version
```

### Examples

**Convert a Markdown file:**
```bash
mdtolinkedin post.md
```

**Convert with output file:**
```bash
mdtolinkedin post.md -o linkedin_post.txt
```

**Pipe from stdin:**
```bash
echo "**Hello** *world*" | mdtolinkedin
```

**Suppress character limit warning:**
```bash
mdtolinkedin long_post.md --no-warn
```

**Generate Carbon URLs for code blocks:**
```bash
mdtolinkedin post.md --carbon
```

## Markdown Transformation Rules

| Markdown | LinkedIn Output |
|----------|----------------|
| `# Header` | Bold text (no # symbol) |
| `**bold**` | Unicode bold characters |
| `*italic*` | Unicode italic characters |
| `- item` | `• item` (bullet symbol) |
| `1. item` | `1. item` (preserved) |
| `> quote` | Italic text |
| `[text](url)` | `text (url)` |
| `` `code` `` | Remove backticks, plain text |
| ``````` code ``````` | Omit OR generate Carbon.now.sh URL (with `--carbon` flag) |

## Character Limit

LinkedIn posts have a 3000 character limit. The CLI automatically:
- Counts output characters
- Prints a warning to stderr if output exceeds 3000 characters
- Warnings can be suppressed with `--no-warn` flag

## Development

### Build

```bash
cargo build
cargo build --release  # Optimized release build
```

### Test

```bash
cargo test              # Run all tests
cargo test --verbose    # Run with output
```

### Code Quality

```bash
cargo fmt              # Format code
cargo fmt --check       # Check formatting
cargo clippy           # Run linter
```

## License

MIT

## Repository

https://github.com/juanje/mdtolinkedin
