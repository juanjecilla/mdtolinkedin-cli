# mdtolinkedin CLI - AI Agent Task Documentation

This directory contains comprehensive documentation for an AI agent to implement the mdtolinkedin CLI tool.

## Project Overview

**Name:** mdtolinkedin  
**Type:** Command-line tool  
**Language:** Rust  
**Purpose:** Convert Markdown to LinkedIn-compatible text using Unicode formatting.

## Goals

Build a CLI that:
1. Parses Markdown (headers, bold, italic, lists, links, quotes).
2. Converts to LinkedIn format using Unicode math characters.
3. Supports multiple I/O modes (stdin, file, stdout).
4. Warns on character limit (>3000).
5. Optionally generates Carbon URLs for code blocks.

## Directory Structure

```
ai-agent-docs/mdtolinkedin-cli/
├── README.md                  # This file
├── PROJECT_CONTEXT.md         # Full technical context
├── ARCHITECTURE.md            # Module design
├── tasks/
│   ├── 01-project-setup.md    # Cargo init, deps, CI
│   ├── 02-unicode-module.md   # Unicode transformer
│   ├── 03-markdown-parser.md  # Converter module
│   ├── 04-cli-io.md           # CLI args and I/O
│   └── 05-release.md          # GitHub Actions, Homebrew
└── reference/
    └── unicode-mapping.md     # Character mapping table
```

## How to Use

1. Each task file contains:
   - **Context**: Background info needed.
   - **Goal**: What to achieve.
   - **Implementation Steps**: Detailed steps.
   - **Definition of Done**: Verification criteria.
   - **Example Code**: Snippets to guide implementation.
2. Tasks are ordered by dependency.
3. An agent should complete tasks sequentially.
