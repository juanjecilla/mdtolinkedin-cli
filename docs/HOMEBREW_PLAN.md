# Homebrew Setup Plan

This plan matches the README instruction:

```
brew tap juanjecilla/tap
brew install mdtolinkedin
```

Homebrew maps `juanjecilla/tap` to the GitHub repo `juanjecilla/homebrew-tap` by default, so the tap repo should use that name.

## 1. Create the tap repository

1. Create a GitHub repo named `homebrew-tap` under the `juanjecilla` account.
2. Add a `Formula/` directory at the repo root.
3. Add a `README.md` explaining how to use the tap:
   - `brew tap juanjecilla/tap`
   - `brew install mdtolinkedin`

## 2. Build a Homebrew formula

1. Copy the formula template from `RELEASE.md`.
2. Set:
   - `homepage` to `https://github.com/juanjecilla/mdtolinkedin-cli`
   - `version` to `0.1.0`
   - URLs to the GitHub Release artifacts produced by the release workflow.
3. After the release exists, compute SHA256 for each artifact and fill the values.

## 3. Release artifact expectations

The release workflow should upload these artifacts (already configured in `.github/workflows/release.yml`):

- `mdtolinkedin-macos-aarch64.tar.gz`
- `mdtolinkedin-macos-x86_64.tar.gz`
- `mdtolinkedin-linux-x86_64.tar.gz`
- `mdtolinkedin-windows-x86_64.zip`

Only the macOS and Linux tarballs are used in the Homebrew formula. Windows is optional for Homebrew.

## 4. Publish and validate

1. Commit the formula to `juanjecilla/homebrew-tap`.
2. Run:
   ```bash
   brew tap juanjecilla/tap
   brew install mdtolinkedin
   mdtolinkedin --version
   ```
3. If Homebrew caches old artifacts, run:
   ```bash
   brew update
   brew reinstall mdtolinkedin
   ```

## 5. Optional automation (later)

- Add a GitHub Action in the tap repo to auto-update the formula when a new release is published (using `brew bump-formula-pr` or a custom script).
- Add a checksum generation step in the release workflow to make copy/paste easier.
