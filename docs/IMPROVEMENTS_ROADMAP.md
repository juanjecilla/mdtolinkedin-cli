# Improvements Roadmap

This document proposes a concrete, prioritized list of improvements to implement
in the project. It complements `IMPROVEMENTS.md` with a more execution-focused
plan.

## P0 (Release Readiness)

1. Ensure CI is green on all PRs (fmt, clippy, tests).
2. Verify `v0.1.0` release artifacts are attached to GitHub Releases.
3. Publish the Homebrew tap formula in `juanjecilla/homebrew-tap`.
4. Capture SHA256 checksums for release artifacts and update the formula.

## P1 (Distribution)

1. Publish to crates.io (`cargo publish`).
2. Automate Homebrew formula updates after each release.
3. Add Linux packages (`cargo-deb`, `cargo-rpm`).
4. Add Windows distribution via Scoop or winget.

## P2 (CI/CD Improvements)

1. Add caching for Cargo registry and target directory to speed CI.
2. Expand CI matrix to macOS and Windows for test parity.
3. Add `cargo audit` or `cargo deny` for dependency security checks.
4. Add release checksums and signature verification (optional GPG/Sigstore).

## P3 (Quality and Testing)

1. Add tests for ordered list numbering and mixed list types.
2. Add tests for bold+italic combinations (`***text***`).
3. Add fixture-based regression tests for common Markdown examples.
4. Add tests for JSON output and `--plain` mode.
5. Add tests for code image error handling (missing fonts, invalid themes).

## P4 (Performance and DX)

1. Reduce allocations in `converter::convert` (reuse buffers, avoid `format!`).
2. Add benchmarks for large inputs (criterion or custom harness).
3. Add `just` or `make` tasks to standardize common workflows.

## P5 (Documentation and UX)

1. Add a full CLI reference page and example outputs in `docs/`.
2. Document code-block handling modes with screenshots.
3. Add a CONTRIBUTING guide (workflow, testing, release steps).
4. Add a CHANGELOG policy (keep a consistent format for every release).
