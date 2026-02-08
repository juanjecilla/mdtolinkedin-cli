#!/usr/bin/env bash
set -euo pipefail

if ! command -v gh >/dev/null 2>&1; then
  echo "error: gh CLI is required" >&2
  exit 1
fi

if [[ $# -lt 1 ]]; then
  echo "usage: $0 <tag> [--require-signature] [--repo <owner/repo>]" >&2
  exit 1
fi

tag="$1"
shift
require_signature="false"
repo="${REPO:-juanjecilla/mdtolinkedin-cli}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --require-signature)
      require_signature="true"
      shift
      ;;
    --repo)
      repo="$2"
      shift 2
      ;;
    *)
      echo "unknown argument: $1" >&2
      exit 1
      ;;
  esac
done

assets=$(gh release view "$tag" --repo "$repo" --json assets --jq '.assets[].name')

expected=(
  "mdtolinkedin-linux-x86_64.tar.gz"
  "mdtolinkedin-macos-x86_64.tar.gz"
  "mdtolinkedin-macos-aarch64.tar.gz"
  "mdtolinkedin-windows-x86_64.zip"
  "mdtolinkedin-linux-x86_64.deb"
  "mdtolinkedin-linux-x86_64.rpm"
  "SHA256SUMS.txt"
)

if [[ "$require_signature" == "true" ]]; then
  expected+=("SHA256SUMS.txt.asc")
fi

missing=0
for item in "${expected[@]}"; do
  if ! grep -qx "$item" <<<"$assets"; then
    echo "missing: $item"
    missing=1
  fi
done

if [[ $missing -ne 0 ]]; then
  echo "release $tag is missing required assets" >&2
  exit 1
fi

echo "release $tag has all required assets"
