# Release Notes - v0.1.0

## Highlights

- Markdown to LinkedIn-friendly Unicode conversion (bold/italic/headers).
- File and stdin/stdout support for easy piping.
- Code block handling modes: omit, text, Carbon URL, or rendered images.
- Optional JSON output for automation workflows.

## Installation

```bash
brew tap juanjecilla/tap
brew install mdtolinkedin
```

Or download binaries from GitHub Releases.

## Usage

```bash
mdtolinkedin input.md
cat input.md | mdtolinkedin
mdtolinkedin input.md -o output.txt
mdtolinkedin post.md --format json
```

## Checksums

Release artifacts:

- `mdtolinkedin-macos-aarch64.tar.gz`
- `mdtolinkedin-macos-x86_64.tar.gz`
- `mdtolinkedin-linux-x86_64.tar.gz`
- `mdtolinkedin-windows-x86_64.zip`

SHA256 values should be recorded in the Homebrew formula after the artifacts are published.
