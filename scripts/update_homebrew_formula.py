#!/usr/bin/env python3
from __future__ import annotations

import argparse
from pathlib import Path
import sys

ASSETS = {
    "macos_arm": "mdtolinkedin-macos-aarch64.tar.gz",
    "macos_x86": "mdtolinkedin-macos-x86_64.tar.gz",
    "linux_x86": "mdtolinkedin-linux-x86_64.tar.gz",
}


def normalize_tag(tag: str) -> str:
    return tag if tag.startswith("v") else f"v{tag}"


def normalize_version(tag: str) -> str:
    return tag[1:] if tag.startswith("v") else tag


def read_checksums(path: Path) -> dict[str, str]:
    checksums: dict[str, str] = {}
    for line in path.read_text().splitlines():
        line = line.strip()
        if not line:
            continue
        parts = line.split()
        if len(parts) < 2:
            continue
        checksum, filename = parts[0], parts[-1]
        checksums[filename] = checksum
    return checksums


def build_formula(version: str, tag: str, checksums: dict[str, str]) -> str:
    def sha(asset: str) -> str:
        if asset not in checksums:
            raise KeyError(f"missing checksum for {asset}")
        return checksums[asset]

    return f"""class Mdtolinkedin < Formula
  desc \"Convert Markdown to LinkedIn-compatible text\"
  homepage \"https://github.com/juanjecilla/mdtolinkedin-cli\"
  version \"{version}\"

  on_macos do
    on_arm do
      url \"https://github.com/juanjecilla/mdtolinkedin-cli/releases/download/{tag}/{ASSETS['macos_arm']}\"
      sha256 \"{sha(ASSETS['macos_arm'])}\"
    end
    on_intel do
      url \"https://github.com/juanjecilla/mdtolinkedin-cli/releases/download/{tag}/{ASSETS['macos_x86']}\"
      sha256 \"{sha(ASSETS['macos_x86'])}\"
    end
  end

  on_linux do
    url \"https://github.com/juanjecilla/mdtolinkedin-cli/releases/download/{tag}/{ASSETS['linux_x86']}\"
    sha256 \"{sha(ASSETS['linux_x86'])}\"
  end

  def install
    bin.install \"mdtolinkedin\"
  end

  test do
    assert_match \"mdtolinkedin\", shell_output(\"#{bin}/mdtolinkedin --version\")
  end
end
"""


def main() -> int:
    parser = argparse.ArgumentParser(description="Update Homebrew formula for mdtolinkedin")
    parser.add_argument("--version", required=True, help="Release tag or version (e.g. v0.1.0)")
    parser.add_argument("--checksums", required=True, type=Path, help="Path to SHA256SUMS.txt")
    parser.add_argument("--tap-dir", required=True, type=Path, help="Path to homebrew-tap repo")
    parser.add_argument("--stdout", action="store_true", help="Print formula instead of writing")
    args = parser.parse_args()

    tag = normalize_tag(args.version)
    version = normalize_version(tag)
    checksums = read_checksums(args.checksums)

    try:
        formula = build_formula(version, tag, checksums)
    except KeyError as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1

    if args.stdout:
        print(formula)
        return 0

    formula_path = args.tap_dir / "Formula" / "mdtolinkedin.rb"
    formula_path.parent.mkdir(parents=True, exist_ok=True)
    formula_path.write_text(formula)
    print(f"Updated {formula_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
