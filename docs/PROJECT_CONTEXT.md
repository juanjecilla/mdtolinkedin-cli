# Project Context: mdtolinkedin CLI

## Problem Statement

LinkedIn does not support Markdown formatting. To use **bold** or *italic* text, users must manually copy-paste special Unicode characters (Mathematical Alphanumeric Symbols). This CLI automates that conversion.

## Reference Implementation

The web app [devlimelabs/markdowntolinkedin](https://github.com/devlimelabs/markdowntolinkedin) provides similar functionality in JavaScript. Key takeaways:

- Uses Unicode Mathematical Bold (U+1D400–U+1D433) for bold.
- Uses Unicode Mathematical Italic (U+1D434–U+1D467) for italic.
- Converts `# Header` to bold text.
- Converts `- item` to `• item`.
- Converts `> quote` to italic.
- Converts `[text](url)` to `text (url)`.

## Technical Requirements

### Language & Dependencies

| Dependency | Purpose | Crate |
|------------|---------|-------|
| Rust | Language | - |
| clap | CLI argument parsing | `clap = { version = "4", features = ["derive"] }` |
| pulldown-cmark | Markdown parsing | `pulldown-cmark = "0.10"` |

### I/O Modes

```bash
# File → stdout
mdtolinkedin input.md

# File → file
mdtolinkedin input.md -o output.txt

# stdin → stdout
cat input.md | mdtolinkedin

# stdin → file
echo "**bold**" | mdtolinkedin -o out.txt
```

### CLI Flags

| Flag | Description |
|------|-------------|
| `[INPUT]` | Input file (optional, stdin if omitted) |
| `-o, --output <FILE>` | Output file (optional, stdout if omitted) |
| `--carbon` | Generate Carbon.now.sh URLs for code blocks (legacy) |
| `--code-blocks <MODE>` | Code block handling: omit, text, or carbon |
| `--no-warn` | Suppress character limit warning |
| `--max-chars <N>` | Custom character limit for warnings |
| `--bullet <CHAR>` | Custom bullet character for unordered lists |
| `--no-trim` | Preserve trailing newlines in output |
| `--plain` | Disable Unicode mapping (plain text output) |
| `--format <FORMAT>` | Output format: text or json |
| `--code-image-dir <DIR>` | Output directory for code images |
| `--code-image-theme <NAME>` | Code image theme (syntect preset) |
| `--code-image-font <PATH>` | Code image font path (TTF/OTF) |
| `--code-image-font-size <PX>` | Code image font size |
| `--code-image-bg <HEX>` | Code image background color |
| `--code-image-padding <PX>` | Code image padding |

### Character Limit

LinkedIn posts max at 3000 characters. The CLI should:
1. Count output characters.
2. Print warning to stderr if >3000.

## Formatting Rules

### Bold (Mathematical Bold)

```
ASCII A-Z → U+1D400 to U+1D419
ASCII a-z → U+1D41A to U+1D433
```

Example: `**Hello**` → `𝗛𝗲𝗹𝗹𝗼`

### Italic (Mathematical Italic)

```
ASCII A-Z → U+1D434 to U+1D44D
ASCII a-z → U+1D44E to U+1D467
```

Example: `*Hello*` → `𝘏𝘦𝘭𝘭𝘰`

### Bold Italic (Mathematical Bold Italic)

```
ASCII A-Z → U+1D468 to U+1D481
ASCII a-z → U+1D482 to U+1D49B
```

Example: `***Hi***` → `𝒉𝒊`

### Other Transformations

| Markdown | Output |
|----------|--------|
| `# Header` | Bold header text |
| `## Subheader` | Bold subheader text |
| `- item` | `• item` |
| `1. item` | `1. item` (keep numbered) |
| `***bold italic***` | Bold-italic Unicode characters |
| `> quote` | Italic quote text |
| `[text](url)` | `text (url)` |
| `![alt](url)` | `alt (url)` |
| `` `code` `` | Remove backticks, keep text |
| ```` ```code``` ```` | Omit, keep as text, or Carbon URL (via `--code-blocks`) |
| ```` ```code``` ```` | Image generation via `--code-blocks image` (PNG + SVG) |
| Emoji | Preserve unchanged |

### Spacing Rules

- Headings end with a blank line.
- Paragraphs end with a blank line.
- List items end with a newline; lists end with a blank line.
- Blockquotes end with a newline.
- Code block output (text or Carbon URL) ends with a blank line.

### List Formatting Notes

- Ordered lists preserve numbering at the top level.
- Nested list indentation is not preserved in output; all nested items are flattened.
- Unordered list bullets can be customized with `--bullet <CHAR>`.

### Formatting Examples

| Markdown | Output |
|----------|--------|
| `**Bold [Link](https://example.com)**` | `𝐁𝐨𝐥𝐝 𝐋𝐢𝐧𝐤 (https://example.com)` |
| `*Italic [Link](https://example.com)*` | `𝑰𝑡𝑎𝑙𝑖𝑐 𝑳𝒊𝒏𝒌 (https://example.com)` |

### Fixture Regeneration

Use `scripts/update_fixtures.py` to regenerate expected outputs in `tests/fixtures/`.

### JSON Output Notes

- JSON output uses escaped control characters (`\\n`, `\\t`, `\\r`) and quotes.

## Distribution

1. **GitHub Releases**: Cross-compiled binaries for Linux, macOS (Intel/ARM), Windows.
2. **Homebrew**: Formula in a tap repository.

## Future: REST API

Phase 4 (not in initial scope) will add an HTTP API:

```
POST /convert
Content-Type: text/plain

**bold** text

---

Response: 𝗯𝗼𝗹𝗱 text
```

Framework: `axum` or `actix-web`.
