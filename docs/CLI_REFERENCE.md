# CLI Reference

## Usage

```bash
mdtolinkedin [OPTIONS] [INPUT]
```

If `INPUT` is omitted, the CLI reads from stdin. If `--output` is omitted, it writes to stdout.

## Full Help Output

```text
Convert Markdown to LinkedIn-compatible text

Usage: mdtolinkedin [OPTIONS] [INPUT]

Arguments:
  [INPUT]  Input Markdown file (reads from stdin if omitted)

Options:
  -o, --output <OUTPUT>            Output file (writes to stdout if omitted)
      --carbon                     Generate Carbon.now.sh URLs for code blocks
      --code-blocks <MODE>         Code block handling: omit, text, or carbon [possible values: omit, text, carbon, image]
      --no-warn                    Suppress character limit warning
      --max-chars <N>              Custom character limit for warnings
      --bullet <CHAR>              Custom bullet character for unordered lists
      --no-trim                    Preserve trailing newlines in output
      --plain                      Disable Unicode mapping (plain text output)
      --format <FORMAT>            Output format: text or json [default: text] [possible values: text, json]
      --code-image-dir <DIR>       Code image output directory (for --code-blocks image) [default: code-images]
      --code-image-theme <NAME>    Code image theme (syntect preset) [default: InspiredGitHub]
      --code-image-font <PATH>     Code image font path (TTF/OTF)
      --code-image-font-size <PX>  Code image font size (px) [default: 16]
      --code-image-bg <HEX>        Code image background color (hex) [default: #ffffff]
      --code-image-padding <PX>    Code image padding (px) [default: 24]
  -h, --help                       Print help
  -V, --version                    Print version
```

## Examples

### Basic Conversion

Input:

```markdown
# Hello

**Bold** and *italic* with [link](https://example.com).

- Item one
```

Output:

```text
𝐇𝐞𝐥𝐥𝐨

𝐁𝐨𝐥𝐝 and 𝑖𝑡𝑎𝑙𝑖𝑐 with link (https://example.com).

• Item one
```

### JSON Output

```bash
echo "**Bold**" | mdtolinkedin --format json
```

```json
{"text":"𝐁𝐨𝐥𝐝","char_count":4,"limit":3000,"limit_exceeded":false}
```
