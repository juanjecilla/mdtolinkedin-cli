# Improvement Ideas

This list focuses on release readiness, CI/CD, distribution, and code quality.

## CI/CD

- Add Cargo cache for registry and build artifacts to speed up CI
- Expand the test matrix to macOS and Windows for parity with release builds
- Add an MSRV check (document the minimum supported Rust version)
- Add `cargo audit` or `cargo deny` to catch vulnerable or banned dependencies
- Publish code coverage reports (e.g., `grcov` or `tarpaulin`)
- Add release signing (checksums + optional GPG/Sigstore)

## CLI Distribution

- Publish to crates.io (`cargo install mdtolinkedin`)
- Automate Homebrew formula updates in a dedicated tap repo
- Add Linux packages (`cargo-deb` for `.deb`, `cargo-rpm` for `.rpm`)
- Provide Windows distribution via Scoop or winget
- Provide a one-line install script for Linux/macOS (curl + sh)

## Code Quality

- Add regression tests for ordered lists, nested styles, and code-block modes
- Add snapshot/fixture tests for common Markdown examples
- Add tests for JSON output and `--plain` mode
- Add lint gates for `cargo fmt` and `clippy` in pre-commit hooks
- Add fuzz or property-based tests around Markdown parsing edge cases
- Document and test error handling for code image rendering failures
