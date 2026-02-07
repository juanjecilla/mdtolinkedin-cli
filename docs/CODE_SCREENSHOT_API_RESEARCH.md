# Code Screenshot API Research (Carbon Replacement)

Date: 2026-02-03

This document surveys practical ways to generate code screenshots via API without relying on Carbon. It emphasizes options that are stable, automatable, and easy to integrate into a CLI or service.

## Goals

- Generate code images programmatically (PNG/SVG)
- Simple API integration (HTTP)
- Control over themes, fonts, padding, and background
- No dependence on carbon.now.sh
- Reliable for automation (CI, n8n, release pipelines)

## Option A: Code‑specific image APIs (SaaS)

### Pictify (code-to-image + HTML-to-image)
- Provides a code-to-image tool and an HTML-to-image API; suitable for rendering code cards via HTTP.
- Useful if you want a managed service with templates and a straightforward API.

Pros
- Minimal infra (no headless browsers to manage)
- Handles scaling and rendering

Cons
- Vendor lock‑in
- Template customization is limited to their API model

## Option B: General screenshot/HTML‑to‑image APIs (SaaS)

These services accept a URL (or HTML) and return a screenshot image. You can render a code card in HTML and ask the API to screenshot it.

Examples:
- CaptureAPI (URL → screenshot REST API)
- ScreenshotAPI (URL → screenshot REST API)

Pros
- Simple integration
- Scales without owning browser infra

Cons
- You must host an HTML code renderer (or send HTML)
- Extra latency (HTTP + render + screenshot)
- Potential privacy concerns if code leaves your system

## Option C: Self‑hosted rendering service (recommended for control)

### Architecture
1. Server accepts code, language, theme settings
2. Syntax highlighting via a library (server‑side)
3. Render HTML (or SVG) template
4. Headless browser generates PNG/SVG
5. Return image bytes or URL

### Building blocks
- **Syntax highlighting**: starry-night (GitHub‑style), lowlight (highlight.js), or similar
- **Rendering**: Playwright or Puppeteer to screenshot a prepared HTML page

Pros
- Full control of appearance and data handling
- No third‑party dependency at runtime
- Easy to add presets and versioned themes

Cons
- Requires owning browser infra (Playwright/Chromium)
- More operational complexity

## Option D: Self‑host ray.so UI + headless capture

Ray.so is open source and can be self‑hosted. It’s a full UI with code image rendering. A common approach is to host it internally and use a headless browser to render to PNG via API.

Pros
- High‑quality aesthetic out of the box
- Reduced design work

Cons
- Heavier runtime (Next.js app + headless capture)
- You still need a screenshot service

## Recommendation (for this repo)

If you want to fully remove Carbon and keep control, **Option C** is the most robust. It lets you ship a small internal API (or sidecar service) that is stable and tailored to LinkedIn posts. It also avoids vendor lock‑in and keeps code private.

Suggested stack:
- starry-night for syntax highlighting (GitHub‑like)
- HTML template + CSS tokens for theme control
- Playwright for server‑side rendering and screenshots
- Optional caching by hash of (code + options)

## Implementation Steps (high‑level)

1. Define request schema: code, language, theme, font, padding, background, output size
2. Implement syntax highlighting pipeline
3. Generate deterministic HTML (single template + inline CSS)
4. Render + screenshot with Playwright
5. Add caching and limits (rate, payload size)
6. Expose via HTTP endpoint (e.g., `/render`)
7. Add CLI flag to call the API and return image URL or save to file

## Open Questions

- Do we want PNG only or SVG too?
- Should we expose presets (themes/fonts) or allow arbitrary CSS values?
- Should the API accept raw HTML to keep it flexible?
- Where do we host (local, internal service, cloud)?

## Sources

- Pictify code-to-image + HTML‑to‑image API: [pictify.io/tools/code-to-image](https://pictify.io/tools/code-to-image)
- Playwright screenshots API: [playwright.dev/docs/screenshots](https://playwright.dev/docs/screenshots)
- starry‑night (GitHub‑style highlighting): [github.com/wooorm/starry-night](https://github.com/wooorm/starry-night)
- lowlight (highlight.js based): [github.com/wooorm/lowlight](https://github.com/wooorm/lowlight)
- ray.so open source repo: [github.com/raycast/ray-so](https://github.com/raycast/ray-so)
- CaptureAPI screenshot API: [captureapi.net](https://captureapi.net/)
- ScreenshotAPI docs: [screenshotapi.net/docs](https://www.screenshotapi.net/docs)
