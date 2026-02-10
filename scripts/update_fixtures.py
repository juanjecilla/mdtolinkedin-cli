#!/usr/bin/env python3
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

FIXTURES = [
    ("tests/fixtures/whitespace.md", "tests/fixtures/whitespace.txt", []),
    ("tests/fixtures/nested.md", "tests/fixtures/nested.txt", []),
    ("tests/fixtures/links_and_styles.md", "tests/fixtures/links_and_styles.txt", []),
    ("tests/fixtures/mixed_lists.md", "tests/fixtures/mixed_lists.txt", []),
    ("tests/fixtures/nested_lists.md", "tests/fixtures/nested_lists.txt", []),
    ("tests/fixtures/ordered_list_start.md", "tests/fixtures/ordered_list_start.txt", []),
    ("tests/fixtures/common.md", "tests/fixtures/common.txt", []),
    ("tests/fixtures/code_blocks.md", "tests/fixtures/code_blocks_omit.txt", ["--code-blocks", "omit"]),
    ("tests/fixtures/code_blocks.md", "tests/fixtures/code_blocks_text.txt", ["--code-blocks", "text"]),
    ("tests/fixtures/code_blocks.md", "tests/fixtures/code_blocks_carbon.txt", ["--code-blocks", "carbon"]),
    ("tests/fixtures/whitespace.md", "tests/fixtures/whitespace_notrim.txt", ["--no-trim"]),
    ("tests/fixtures/plain.md", "tests/fixtures/plain.txt", ["--plain"]),
    ("tests/fixtures/images.md", "tests/fixtures/images.txt", []),
    ("tests/fixtures/json.md", "tests/fixtures/json.txt", ["--format", "json"]),
]


def run_cli(input_path: Path, args: list[str]) -> str:
    cmd = ["cargo", "run", "--quiet", "--", str(input_path)] + args
    result = subprocess.run(
        cmd,
        cwd=ROOT,
        check=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        text=True,
    )
    output = result.stdout
    if output.endswith("\n"):
        output = output[:-1]
    return output


def main() -> int:
    for input_rel, output_rel, args in FIXTURES:
        input_path = ROOT / input_rel
        output_path = ROOT / output_rel
        output = run_cli(input_path, args)
        output_path.write_text(output, encoding="utf-8")
        print(f"Wrote {output_rel}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
