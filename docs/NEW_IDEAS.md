# New Ideas Backlog

Date: 2026-02-03

This file captures new feature ideas and enhancements proposed during recent review.

## CLI and Output Options
- `--code-blocks text|omit|carbon` to formalize code block policy
- `--max-chars <N>` to customize warning thresholds
- `--bullet <char>` to customize bullet glyph
- `--plain` to disable Unicode mapping for plain-text output
- `--no-trim` to preserve trailing newlines
- Optional JSON output for automation pipelines

## Markdown Extensions
- Inline image conversion: `![alt](url)` → `alt (url)`
- Preserve Markdown segments that LinkedIn supports (plain text mode per section)

## Ecosystem
- Provide a library crate so other Rust projects can reuse the converter

## Carbon Enhancements
- Expose Carbon settings (theme, font, background) via CLI flags
- Option to emit Markdown-style link text for Carbon URLs

## Formatting Controls
- Ordered list start support (preserve Markdown start index)
- List indentation control for nested lists
