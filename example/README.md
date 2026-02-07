# Example outputs

Generated from `sample.md` using the mdtolinkedin CLI with different options.

| File | Command |
|------|--------|
| `output.txt` | `mdtolinkedin sample.md -o output.txt --bullet "•" --no-trim --max-chars 3000` (default: code blocks omitted) |
| `output.json` | `mdtolinkedin sample.md -o output.json --format json --no-warn` |
| `output_plain.txt` | `mdtolinkedin sample.md -o output_plain.txt --plain` (no Unicode styling) |
| `output_code_text.txt` | `mdtolinkedin sample.md -o output_code_text.txt --code-blocks text` |
| `output_carbon.txt` | `mdtolinkedin sample.md -o output_carbon.txt --code-blocks carbon` |
| `output_image.txt` | `mdtolinkedin sample.md -o output_image.txt --code-blocks image --code-image-dir code-images --code-image-theme Monokai --code-image-font-size 14 --code-image-bg "#1e1e1e" --code-image-padding 16` |

The `code-images/` directory contains PNG and SVG files when using `--code-blocks image`.
