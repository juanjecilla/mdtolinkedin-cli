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
| `--carbon` | Generate Carbon.now.sh URLs for code blocks |
| `--no-warn` | Suppress character limit warning |

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

### Other Transformations

| Markdown | Output |
|----------|--------|
| `# Header` | Bold header text |
| `## Subheader` | Bold subheader text |
| `- item` | `• item` |
| `1. item` | `1. item` (keep numbered) |
| `> quote` | Italic quote text |
| `[text](url)` | `text (url)` |
| `` `code` `` | Remove backticks, keep text |
| ```` ```code``` ```` | Remove entirely OR Carbon URL |
| Emoji | Preserve unchanged |

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
