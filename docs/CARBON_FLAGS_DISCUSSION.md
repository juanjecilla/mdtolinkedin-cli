# Carbon Settings Flags Discussion

Date: 2026-02-03

This document captures options and open questions for Carbon.now.sh settings flags.

## Proposed CLI Flags (Draft)

- `--carbon-theme <name>`
  - Example: `--carbon-theme dracula`
  - Default: Carbon default (leave unset)

- `--carbon-font <name>`
  - Example: `--carbon-font Fira%20Code`
  - Default: Carbon default

- `--carbon-bg <hex>`
  - Example: `--carbon-bg #282A36`
  - Default: Carbon default

- `--carbon-lang <lang>`
  - Example: `--carbon-lang rust`
  - Default: infer from fenced code block info when present

- `--carbon-padding <px>`
  - Example: `--carbon-padding 32`
  - Default: Carbon default

- `--carbon-line-numbers` / `--carbon-no-line-numbers`
  - Default: Carbon default

- `--carbon-window-controls <true|false>`
  - Default: Carbon default

## URL Mapping Notes

Carbon URLs accept query params like:
- `code=` (percent-encoded)
- `l=` language
- `t=` theme
- `fm=` font
- `bg=` background
- `pad=` padding
- `ln=` line numbers (true/false)
- `wc=` window controls (true/false)

## Open Questions

1. Should we validate theme/font names or pass through verbatim?
2. Do we want a single `--carbon` preset flag (e.g., `--carbon-preset <name>`) to bundle multiple settings?
3. Should these flags apply globally or allow per-code-block overrides based on fenced info?
4. Should we expose advanced options (line numbers, window controls) or keep it minimal?
5. Should `--code-blocks carbon` auto-enable a default `--carbon-lang` if none is present?

## Native CLI Alternative (No API)

See `docs/NATIVE_CODE_IMAGE_PLAN.md` for a pure Rust approach using syntect + resvg. This avoids services entirely and runs inside the CLI.
