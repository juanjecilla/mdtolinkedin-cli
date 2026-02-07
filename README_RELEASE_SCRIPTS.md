# Release Scripts

This project includes two release preparation scripts that automate the release process:

## Available Scripts

### 1. `prepare-release.py` (Python - Recommended)

**Standalone Python script** that works on any platform with Python 3.6+.

**Features:**
- ✅ Cross-platform (Windows, macOS, Linux)
- ✅ No external dependencies (uses only Python standard library)
- ✅ Colored terminal output for better readability
- ✅ Interactive prompts for commit and tag creation
- ✅ Comprehensive error handling
- ✅ Checks Rust/Cargo installation
- ✅ Runs all code quality checks

**Usage:**
```bash
python3 prepare-release.py
# or
./prepare-release.py
```

**What it does:**
1. Sets up Rust toolchain (if needed)
2. Runs `cargo check`, `cargo test`, `cargo fmt`, `cargo clippy`
3. Stages all necessary files
4. Creates commit (with your approval)
5. Creates tag (with your approval)
6. Shows push instructions

### 2. `prepare-release.sh` (Bash)

**Bash script** for Unix-like systems (macOS, Linux).

**Features:**
- ✅ Simple and fast
- ✅ Uses standard bash commands
- ✅ Interactive prompts

**Usage:**
```bash
./prepare-release.sh
```

**Requirements:**
- Bash shell
- Unix-like system (macOS, Linux)

## Comparison

| Feature | Python Script | Bash Script |
|---------|--------------|-------------|
| Cross-platform | ✅ Yes | ❌ Unix only |
| Dependencies | ✅ None (stdlib) | ✅ Bash |
| Error handling | ✅ Comprehensive | ✅ Basic |
| Colored output | ✅ Yes | ❌ No |
| File validation | ✅ Yes | ❌ No |
| Git repo check | ✅ Yes | ❌ No |

## Recommendation

**Use `prepare-release.py`** if you:
- Want cross-platform compatibility
- Prefer better error messages and validation
- Want colored output for better readability
- Need more robust error handling

**Use `prepare-release.sh`** if you:
- Are on macOS/Linux
- Prefer simpler bash scripts
- Don't need extra features

## Both Scripts Do the Same Thing

Both scripts will:
1. Verify Rust toolchain is set up
2. Run code quality checks (`cargo check`, `cargo test`, `cargo fmt`, `cargo clippy`)
3. Stage files for commit
4. Ask for confirmation before creating commit
5. Ask for confirmation before creating tag
6. Show instructions for pushing to GitHub

## After Running Either Script

Once you've run either script and created the commit/tag:

```bash
# Push commits
git push origin main

# Push tag (triggers GitHub Actions release)
git push origin v0.1.0
```

GitHub Actions will then automatically:
- Build binaries for Linux, macOS (Intel + ARM), and Windows
- Create a GitHub Release with all artifacts

## Troubleshooting

### Python Script Issues

**"Command not found" errors:**
- Make sure Rust/Cargo is installed and in your PATH
- Run `rustup default stable` manually if needed

**Permission denied:**
```bash
chmod +x prepare-release.py
```

**Python version:**
- Requires Python 3.6 or higher
- Check with: `python3 --version`

### Bash Script Issues

**"Permission denied":**
```bash
chmod +x prepare-release.sh
```

**"Command not found":**
- Make sure you're in the project root directory
- Ensure Rust/Cargo is installed

## Manual Alternative

If you prefer to run commands manually, see `NEXT_STEPS.md` for detailed instructions.
