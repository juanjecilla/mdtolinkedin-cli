# Project Review

Date: 2026-02-03

## Scope
This document captures a quick analysis of the codebase, test status, known gaps, and a short list of technical improvements and feature ideas.

## Current Behavior Summary
- Converts Markdown to LinkedIn-compatible text using Unicode math letters for bold/italic.
- Handles headings, bold, italic, lists, links, blockquotes, and inline code.
- Skips code blocks by default and can emit a placeholder for Carbon when enabled.
- Supports stdin/file input and stdout/file output.
- Warns on output length greater than 3000 characters unless suppressed.

## Modules
- `src/cli.rs`: CLI options and input/output arguments.
- `src/io.rs`: Read from stdin or file; write to stdout or file.
- `src/unicode.rs`: ASCII-to-Unicode math mappings for bold/italic (and bold-italic).
- `src/converter.rs`: Markdown event stream conversion with a style stack.
- `src/main.rs`: Orchestrates CLI, I/O, conversion, warning, and errors.

## Tests Run
- `cargo test`

Results: All unit and integration tests passed. One warning reported in `tests/integration.rs` about `drop(stdin)` on a reference; this can be replaced with `let _ = stdin;` or just `drop(child.stdin.take())` to avoid the warning.

## Coverage Notes / Gaps
- Ordered lists are currently rendered as bullets because `Tag::Item` is always prefixed with `"• "` regardless of list type.
- Bold+italic combined style falls back to bold only; there is a `to_bold_italic` helper but it is unused.
- Links inside headings or other styles are not explicitly tested.
- Code block handling does not currently include the content in the output and the Carbon URL is a placeholder string.
- Paragraph spacing rules (extra newlines) could be tightened to avoid double-blank lines in some nested cases.

## Suggested Improvements (Technical)
1. Track list type (ordered/unordered) so ordered lists retain numbering and unordered lists use bullets.
2. Implement true bold-italic mapping using `to_bold_italic` (or document the deliberate fallback).
3. Replace the Carbon placeholder with a proper URL generator (or remove the flag if not supported).
4. Normalize whitespace handling to avoid extra newlines around lists and blockquotes.
5. Reduce allocations in `convert` by reusing buffers and minimizing `format!` calls.
6. Add a small snapshot-style test suite for common Markdown fixtures to detect regressions.

## Potential New Features
1. Optional output mode to preserve Markdown for sections that LinkedIn supports (e.g., plain text).
2. Configurable list bullet character (e.g., `•`, `-`, `*`).
3. Option to keep code blocks as text with backticks removed (instead of omitting).
4. A `--max-chars` flag to customize the warning threshold.
5. Support for inline images by converting to `alt text (url)`.
6. A `--no-trim` flag to preserve trailing newlines for exact output control.

## Recommended Next Actions
- Decide whether to fix ordered list behavior (would be a behavior change) and add a regression test.
- Replace the Carbon placeholder or update docs to clarify current behavior.
- Add a test for nested bold+italic formatting and blockquote+list interactions.
