# mdtolinkedin

Convert Markdown to LinkedIn-compatible text using Unicode formatting.

LinkedIn doesn't support Markdown natively, so this tool transforms bold/italic
formatting into Unicode Mathematical Alphanumeric Symbols while keeping the rest
of your content clean and readable.

## Features

- Converts Markdown headers, bold, italic, lists, links, images, and blockquotes
- Converts bold/italic text to Unicode math letters (LinkedIn-friendly)
- Supports stdin/stdout and file I/O
- Warns when output exceeds LinkedIn's 3000 character limit
- Optional JSON output for automation
- Optional code block handling (omit, text, Carbon URL, or rendered images)

## Installation

### crates.io

```bash
cargo install mdtolinkedin
```

### Homebrew (macOS/Linux)

```bash
brew tap juanjecilla/tap
brew install mdtolinkedin
```

Maintainers: configure `HOMEBREW_TAP_TOKEN` for release automation. See `docs/HOMEBREW_TAP_TOKEN.md`.

### GitHub Releases

Auto-release on merge to `main` creates a version tag from `Cargo.toml` and publishes a GitHub Release.

Download pre-built binaries from [GitHub Releases](https://github.com/juanjecilla/mdtolinkedin-cli/releases).

### Linux Packages (.deb/.rpm)

Linux packages are attached to each GitHub Release. Install with your distro tooling:

```bash
# Debian/Ubuntu
sudo dpkg -i mdtolinkedin-linux-x86_64.deb

# Fedora/RHEL
sudo rpm -i mdtolinkedin-linux-x86_64.rpm
```

### Windows (Scoop)

The Scoop manifest is maintained in `packaging/scoop`. Publish it to your
Scoop bucket to enable installs:

```powershell
scoop install mdtolinkedin
```

### Build from Source

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build and install
cargo install --git https://github.com/juanjecilla/mdtolinkedin-cli

# Or build locally
git clone https://github.com/juanjecilla/mdtolinkedin-cli.git
cd mdtolinkedin-cli
cargo build --release
```

## Usage

```bash
mdtolinkedin [OPTIONS] [INPUT_FILE]
```

If `INPUT_FILE` is omitted, the tool reads from stdin. If `--output` is omitted,
it writes to stdout.

### Common Options

- `-o, --output <OUTPUT>`: Write output to a file
- `--code-blocks <MODE>`: `omit` (default), `text`, `carbon`, or `image`
- `--max-chars <N>`: Custom character limit for warnings
- `--no-warn`: Suppress the character limit warning
- `--bullet <CHAR>`: Custom bullet character for unordered lists
- `--no-trim`: Preserve trailing newlines in output
- `--plain`: Disable Unicode mapping (plain text output)
- `--format <FORMAT>`: `text` (default) or `json`

Run `mdtolinkedin --help` for the full option list.

## Examples

```bash
# Read from file, output to stdout
mdtolinkedin input.md

# Read from stdin
cat input.md | mdtolinkedin

# Read from file, write to file
mdtolinkedin input.md -o output.txt

# JSON output (for automation)
mdtolinkedin post.md --format json
```

Example JSON output:

```json
{"text":"𝐇𝐞𝐥𝐥𝐨 world","char_count":11,"limit":3000,"limit_exceeded":false}
```

## Code Block Handling

Use `--code-blocks <mode>` to control how fenced code blocks are handled:

- `omit`: Drop code blocks from output (default)
- `text`: Include code blocks as plain text
- `carbon`: Insert a Carbon URL for each block
- `image`: Render PNG and SVG images for each block and insert file paths in the output

For `image`, you can tune output settings:

- `--code-image-dir <DIR>` (default: `code-images`)
- `--code-image-theme <NAME>` (default: `InspiredGitHub`)
- `--code-image-font <PATH>` (optional TTF/OTF)
- `--code-image-font-size <PX>`
- `--code-image-bg <HEX>`
- `--code-image-padding <PX>`

`--carbon` is a legacy alias for `--code-blocks carbon`.

## Markdown Transformation Rules

| Markdown | LinkedIn Output |
|----------|-----------------|
| `# Header` | Bold text (no `#`) |
| `**bold**` | Unicode bold characters |
| `*italic*` | Unicode italic characters |
| `***bold italic***` | Unicode bold italic characters |
| `- item` | `• item` (bullet symbol) |
| `1. item` | `1. item` (preserved) |
| `> quote` | Italic text |
| `[text](url)` | `text (url)` |
| `![alt](url)` | `alt (url)` |
| `` `code` `` | Remove backticks, plain text |
| fenced code blocks | Omit, keep as text, Carbon URL, or render images (via `--code-blocks`) |

## Character Limit

LinkedIn posts have a 3000 character limit. The CLI:

1. Counts output characters (not bytes)
2. Prints a warning to stderr if output exceeds the limit
3. Allows overrides via `--max-chars` and `--no-warn`

## Documentation

- `N8N_USAGE.md` - Automation and n8n usage notes
- `RELEASE.md` - Release checklist and Homebrew template
- `IMPROVEMENTS.md` - Improvement ideas (CI, distribution, quality)
- `docs/ARCHITECTURE.md` - Module design and data flow

## License

MIT
