# Comprehensive Task List

Date: 2026-02-03

This task list consolidates all features and improvements identified in:
- `docs/PROJECT_REVIEW.md`
- `docs/PROJECT_CONTEXT.md`
- `N8N_USAGE.md`
- `STATUS.md` / `NEXT_STEPS.md`

It is organized by milestones and includes implementation, testing, and documentation tasks.

## Milestone 0: Baseline Hygiene
- [ ] Fix test warning in `tests/integration.rs` (`drop(stdin)` on reference)
- [ ] Run `cargo fmt` and `cargo clippy` to keep baseline clean
- [ ] Ensure `README.md` and `docs/PROJECT_CONTEXT.md` match current behavior

## Milestone 1: Markdown Correctness & Spec Alignment

### 1.1 Ordered and Unordered Lists
- [ ] Track list type from `Tag::List(Option<u64>)` and apply:
  - Unordered: prefix `• `
  - Ordered: preserve numbering (increment counter per item)
- [ ] Add unit tests for ordered list numbering and mixed list types
- [ ] Add integration test for ordered lists in CLI output

### 1.2 Bold + Italic Support
- [ ] Use `to_bold_italic` when both styles are active
- [ ] Add tests for nested formatting (`***bold italic***` and bold+italic nested)
- [ ] Update docs to reflect bold-italic behavior

### 1.3 Blockquote and Paragraph Spacing
- [ ] Normalize whitespace to avoid extra blank lines around blockquotes, lists, and paragraphs
- [ ] Add tests for nested blockquote + list + paragraph sequences
- [ ] Document spacing rules in `docs/PROJECT_CONTEXT.md`

### 1.4 Links in Context
- [ ] Add tests for links inside headings, bold, and italics
- [ ] Ensure link formatting is consistent across contexts

## Milestone 2: Code Block Handling and Carbon

### 2.1 Define Code Block Policy
- [ ] Decide on default behavior: omit, keep as plain text, or generate Carbon URLs
- [ ] Update `README.md` and `docs/PROJECT_CONTEXT.md` to match decision

### 2.2 Carbon URL Generation
- [ ] Implement real Carbon URL generation (encode code + settings)
- [ ] Add tests covering `--carbon` flag output
- [ ] Consider language detection from fenced code block info
- [ ] Add docs describing Carbon behavior and limitations

### 2.3 Optional Code Block Retention
- [ ] Add `--code-blocks <mode>` with modes: `omit` (default), `text`, `carbon`
- [ ] Update CLI help and documentation
- [ ] Add tests for each mode

## Milestone 3: CLI Enhancements
- [ ] Add `--max-chars <N>` to customize warning threshold
- [ ] Add `--no-trim` to preserve trailing newlines
- [ ] Add `--bullet <char>` to customize bullet character
- [ ] Add `--plain` (or similar) to disable Unicode mapping and keep plain text
- [ ] Update CLI help, `README.md`, and docs
- [ ] Add integration tests for each flag

## Milestone 4: Converter Performance & Quality
- [ ] Reduce allocations in `converter::convert` (reuse buffers, avoid `format!` where possible)
- [ ] Minimize string copies when applying styles
- [ ] Add benchmarks or simple perf tests for large inputs
- [ ] Add snapshot/fixture tests for common Markdown samples

## Milestone 5: Feature Parity with n8n Notes
- [ ] Ensure `***bold italic***` matches n8n behavior
- [ ] Support custom warning limit (parity with `warn_limit` in n8n docs)
- [ ] Document differences between Rust CLI and Python/n8n versions

## Milestone 6: REST API (Future Phase)
- [ ] Define API spec for `POST /convert` (input and response format)
- [ ] Select framework (`axum` or `actix-web`) and set up server crate/module
- [ ] Implement conversion endpoint using existing converter
- [ ] Add request size limits and basic validation
- [ ] Add tests for API behavior
- [ ] Document API usage and deployment

## Milestone 7: Documentation & Release Updates
- [ ] Update `README.md` with new flags and behaviors
- [ ] Update `docs/PROJECT_CONTEXT.md` with revised rules
- [ ] Update `docs/ARCHITECTURE.md` if module responsibilities change
- [ ] Add a CHANGELOG entry for new features
- [ ] Update release checklist in `RELEASE.md` if needed

## Optional Enhancements (Backlog)
- [ ] Inline image handling: `![alt](url)` → `alt (url)`
- [ ] Preserve Markdown segments that LinkedIn supports (plain text mode per section)
- [ ] Configurable warning output format (JSON for automation)
- [ ] Add examples folder with input/output fixtures
- [ ] Provide a library crate to use the converter directly in other Rust projects

## Validation Checklist (Per Change Set)
- [ ] `cargo fmt`
- [ ] `cargo clippy`
- [ ] `cargo test`
- [ ] Update or add tests for any changed behavior
