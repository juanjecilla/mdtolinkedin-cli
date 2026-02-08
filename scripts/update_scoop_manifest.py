#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path
import sys

ASSET = "mdtolinkedin-windows-x86_64.zip"


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


def main() -> int:
    parser = argparse.ArgumentParser(description="Update Scoop manifest for mdtolinkedin")
    parser.add_argument("--version", required=True, help="Release tag or version (e.g. v0.1.0)")
    parser.add_argument("--checksums", required=True, type=Path, help="Path to SHA256SUMS.txt")
    parser.add_argument(
        "--manifest",
        default=Path("packaging/scoop/mdtolinkedin.json"),
        type=Path,
        help="Path to Scoop manifest",
    )
    args = parser.parse_args()

    tag = normalize_tag(args.version)
    version = normalize_version(tag)
    checksums = read_checksums(args.checksums)

    if ASSET not in checksums:
        print(f"error: missing checksum for {ASSET}", file=sys.stderr)
        return 1

    manifest = {
        "version": version,
        "description": "Convert Markdown to LinkedIn-compatible text",
        "homepage": "https://github.com/juanjecilla/mdtolinkedin-cli",
        "license": "MIT",
        "architecture": {
            "64bit": {
                "url": (
                    "https://github.com/juanjecilla/mdtolinkedin-cli/releases/"
                    f"download/{tag}/{ASSET}"
                ),
                "hash": checksums[ASSET],
            }
        },
        "bin": "mdtolinkedin.exe",
    }

    args.manifest.parent.mkdir(parents=True, exist_ok=True)
    args.manifest.write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"Updated {args.manifest}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
