#!/usr/bin/env python3
"""
Release preparation script for mdtolinkedin v0.1.0
Standalone Python script that can run without dependencies
"""

import subprocess
import sys
import os
from pathlib import Path

# Colors for terminal output
class Colors:
    GREEN = '\033[92m'
    YELLOW = '\033[93m'
    RED = '\033[91m'
    BLUE = '\033[94m'
    RESET = '\033[0m'
    BOLD = '\033[1m'

def print_step(step_num, message):
    """Print a formatted step message"""
    print(f"\n{Colors.BOLD}{Colors.BLUE}Step {step_num}:{Colors.RESET} {message}")

def print_success(message):
    """Print a success message"""
    print(f"{Colors.GREEN}✅ {message}{Colors.RESET}")

def print_warning(message):
    """Print a warning message"""
    print(f"{Colors.YELLOW}⚠️  {message}{Colors.RESET}")

def print_error(message):
    """Print an error message"""
    print(f"{Colors.RED}❌ {message}{Colors.RESET}")

def run_command(cmd, check=True, capture_output=False):
    """Run a shell command and return the result"""
    try:
        if isinstance(cmd, str):
            cmd = cmd.split()
        result = subprocess.run(
            cmd,
            check=check,
            capture_output=capture_output,
            text=True
        )
        return result
    except subprocess.CalledProcessError as e:
        if check:
            raise
        return e
    except FileNotFoundError:
        return None

def check_rust_installed():
    """Check if Rust is installed"""
    result = run_command("rustc --version", check=False, capture_output=True)
    return result is not None and result.returncode == 0

def check_cargo_installed():
    """Check if Cargo is installed"""
    result = run_command("cargo --version", check=False, capture_output=True)
    return result is not None and result.returncode == 0

def setup_rust_toolchain():
    """Set up Rust stable toolchain"""
    print_step(1, "Setting up Rust toolchain...")
    
    if not check_rust_installed():
        print_error("Rust is not installed. Please install Rust first:")
        print("  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh")
        return False
    
    if not check_cargo_installed():
        print_error("Cargo is not installed. Please install Rust toolchain.")
        return False
    
    print("Running: rustup default stable")
    result = run_command("rustup default stable", check=False, capture_output=True)
    
    if result and result.returncode == 0:
        print_success("Rust toolchain configured")
        return True
    else:
        print_warning("Could not set default toolchain automatically.")
        print("You may need to run manually: rustup default stable")
        response = input("Continue anyway? (y/n): ").strip().lower()
        return response == 'y'

def verify_code_quality():
    """Run all code quality checks"""
    print_step(2, "Verifying code quality...")
    
    checks = [
        ("cargo check", "Checking compilation"),
        ("cargo test", "Running tests"),
        ("cargo fmt --check", "Checking formatting"),
        ("cargo clippy -- -D warnings", "Running clippy linter"),
    ]
    
    for cmd, description in checks:
        print(f"\nRunning: {cmd}")
        print(f"  {description}...")
        result = run_command(cmd, check=False, capture_output=True)
        
        if result is None:
            print_error(f"Command not found: {cmd.split()[0]}")
            return False
        
        if result.returncode != 0:
            if "fmt --check" in cmd:
                print_warning("Formatting issues found. Running cargo fmt...")
                fmt_result = run_command("cargo fmt", check=False)
                if fmt_result and fmt_result.returncode == 0:
                    print_success("Code formatted")
                    continue
            print_error(f"{description} failed")
            if result.stderr:
                print(result.stderr)
            return False
        
        print_success(f"{description} passed")
    
    return True

def stage_files():
    """Stage files for commit"""
    print_step(3, "Staging files for commit...")
    
    files_to_add = [
        "src/",
        "tests/",
        ".github/",
        "README.md",
        "AGENTS.md",
        "Cargo.toml",
        "Cargo.lock",
        ".gitignore",
        "RELEASE.md",
        "IMPROVEMENTS.md",
        "STATUS.md",
        "NEXT_STEPS.md",
    ]
    
    # Check which files exist
    existing_files = []
    for file_path in files_to_add:
        if Path(file_path).exists():
            existing_files.append(file_path)
        else:
            print_warning(f"File not found: {file_path}")
    
    if not existing_files:
        print_error("No files to stage")
        return False
    
    print(f"Staging {len(existing_files)} files/directories...")
    result = run_command(["git", "add"] + existing_files, check=False, capture_output=True)
    
    if result and result.returncode == 0:
        print_success("Files staged")
        
        # Show what will be committed
        print("\nFiles staged for commit:")
        status_result = run_command("git status --short", check=False, capture_output=True)
        if status_result and status_result.stdout:
            print(status_result.stdout)
        return True
    else:
        print_error("Failed to stage files")
        if result and result.stderr:
            print(result.stderr)
        return False

def create_commit():
    """Create git commit"""
    print_step(4, "Creating commit...")
    
    commit_message = """feat: initial release v0.1.0

- Implement Unicode Bold/Italic conversion
- Add Markdown to LinkedIn converter
- Add CLI with file/stdin I/O support
- Add character limit warning
- Add integration tests
- Add CI/CD workflows
- Update documentation"""
    
    print("\nCommit message:")
    print("-" * 50)
    print(commit_message)
    print("-" * 50)
    
    response = input("\nCreate commit? (y/n): ").strip().lower()
    if response != 'y':
        print("⏭️  Skipped commit")
        return False
    
    result = run_command(
        ["git", "commit", "-m", commit_message],
        check=False,
        capture_output=True
    )
    
    if result and result.returncode == 0:
        print_success("Commit created")
        if result.stdout:
            print(result.stdout)
        return True
    else:
        print_error("Failed to create commit")
        if result and result.stderr:
            print(result.stderr)
        return False

def create_tag():
    """Create version tag"""
    print_step(5, "Creating version tag...")
    
    version = "v0.1.0"
    tag_message = "Release v0.1.0 - Initial release"
    
    print(f"\nTag: {version}")
    print(f"Message: {tag_message}")
    
    response = input(f"\nCreate tag {version}? (y/n): ").strip().lower()
    if response != 'y':
        print("⏭️  Skipped tag creation")
        return False
    
    result = run_command(
        ["git", "tag", "-a", version, "-m", tag_message],
        check=False,
        capture_output=True
    )
    
    if result and result.returncode == 0:
        print_success(f"Tag {version} created")
        return True
    else:
        print_error(f"Failed to create tag {version}")
        if result and result.stderr:
            print(result.stderr)
        return False

def show_next_steps():
    """Show instructions for next steps"""
    print(f"\n{Colors.BOLD}{Colors.GREEN}📤 Next Steps:{Colors.RESET}")
    print("\n1. Push commits:")
    print("   git push origin main")
    print("\n2. Push tag:")
    print("   git push origin v0.1.0")
    print("\nAfter pushing the tag, GitHub Actions will automatically:")
    print("- Build binaries for Linux, macOS (Intel + ARM), and Windows")
    print("- Create a GitHub Release with all artifacts")
    print("\nThen update the Homebrew formula with SHA256 hashes from the release.")
    print("See RELEASE.md for details.")

def main():
    """Main execution function"""
    print(f"{Colors.BOLD}{Colors.BLUE}🚀 Preparing release v0.1.0...{Colors.RESET}")
    
    # Change to script directory
    script_dir = Path(__file__).parent
    os.chdir(script_dir)
    
    # Check if we're in a git repository
    if not Path(".git").exists():
        print_error("Not in a git repository")
        print("Please run this script from the project root directory")
        sys.exit(1)
    
    try:
        # Step 1: Setup Rust
        if not setup_rust_toolchain():
            print_error("Failed to set up Rust toolchain")
            sys.exit(1)
        
        # Step 2: Verify code quality
        if not verify_code_quality():
            print_error("Code quality checks failed")
            sys.exit(1)
        
        # Step 3: Stage files
        if not stage_files():
            print_error("Failed to stage files")
            sys.exit(1)
        
        # Step 4: Create commit
        create_commit()
        
        # Step 5: Create tag
        create_tag()
        
        # Step 6: Show next steps
        show_next_steps()
        
        print(f"\n{Colors.BOLD}{Colors.GREEN}✅ Release preparation complete!{Colors.RESET}")
        
    except KeyboardInterrupt:
        print(f"\n{Colors.YELLOW}\n⚠️  Interrupted by user{Colors.RESET}")
        sys.exit(1)
    except Exception as e:
        print_error(f"Unexpected error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
