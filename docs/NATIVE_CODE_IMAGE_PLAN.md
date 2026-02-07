# Native Code Image Rendering in CLI (No Carbon)

Date: 2026-02-03

This document outlines native, CLI‑only approaches to generate code screenshots without external services.

## Goals

- Generate PNG images from code blocks inside the CLI
- No external API calls or browser dependencies
- Cross‑platform, deterministic output
- Configurable theme, font, padding, background

## Option 1: Pure Rust pipeline (recommended)

**Flow**
1. Syntax highlight using `syntect` (Sublime grammars) to produce per‑token styles.
2. Convert highlighted spans to SVG (each span with fill color + font attributes).
3. Render SVG to a pixmap using `resvg` (pure Rust, uses tiny‑skia).
4. Encode as PNG via `image` crate and write to disk.

Why this works
- `syntect` provides high‑quality syntax highlighting in Rust. citeturn1search1
- `resvg` renders SVG in pure Rust and uses tiny‑skia, without system libraries; output is deterministic. citeturn1search11
- `image` provides PNG encoding in Rust. citeturn0search0

Tradeoffs
- Need to embed or load fonts (TTF/OTF) for consistent rendering.
- SVG text rendering is adequate but not “native” OS text layout; complex scripts may be limited (resvg docs note lack of native text rendering). citeturn1search11

## Option 2: Skia via `skia-safe` (high quality, heavier)

**Flow**
1. Syntax highlight using `syntect`.
2. Draw text directly to a Skia surface via `skia-safe`.
3. Encode to PNG.

Pros
- Excellent text shaping and rendering fidelity.
- Direct raster path (no SVG intermediate).

Cons
- Large binary and build complexity (Skia C++ deps).
- Heavier CI requirements.

`skia-safe` provides Rust bindings to Skia. citeturn1search5

## Recommendation

If “native CLI only” means **no external services and minimal OS deps**, use **Option 1** (syntect → SVG → resvg → PNG). It is pure Rust, deterministic, and relatively lightweight. If you need top‑tier text rendering and are OK with heavier builds, use Skia.

## Proposed CLI Additions (draft)

- `--code-blocks image` (generate PNGs for code blocks)
- `--code-image-dir <path>` (where to write images)
- `--code-image-theme <name>` (maps to syntect theme)
- `--code-image-font <path>` or `--code-image-font-name <name>`
- `--code-image-bg <hex>`
- `--code-image-padding <px>`

## Open Decisions

1. **Font handling**: bundle a default font in the binary vs. require a path.
2. **Output format**: PNG only vs. optional SVG output.
3. **Theme list**: fixed presets vs. allow custom theme files.
