# Task 3: Markdown Converter Module

**Phase:** 2  
**Estimated Effort:** 2 hours  
**Dependencies:** Task 2 (Unicode Module)

---

## Context

This module uses `pulldown_cmark` to parse Markdown and applies Unicode transformations based on the element type.

### Parsing Strategy

`pulldown_cmark` emits events as it parses:
- `Event::Start(Tag::...)` - Opening tag
- `Event::Text(...)` - Text content
- `Event::End(Tag::...)` - Closing tag

We track context (bold, italic, etc.) using a state stack.

## Goal

Implement `src/converter.rs` to transform Markdown to LinkedIn-formatted text.

---

## Implementation Steps

### Step 3.1: Define Conversion Context

**File:** `src/converter.rs`

```rust
use pulldown_cmark::{Event, Parser, Tag, TagEnd, HeadingLevel};
use crate::unicode::{to_bold, to_italic};

#[derive(Debug, Clone, Copy, PartialEq)]
enum TextStyle {
    Normal,
    Bold,
    Italic,
    BoldItalic,
}

pub struct ConvertOptions {
    pub use_carbon: bool,
}

impl Default for ConvertOptions {
    fn default() -> Self {
        Self { use_carbon: false }
    }
}
```

### Step 3.2: Implement Main Converter

```rust
pub fn convert(markdown: &str, options: &ConvertOptions) -> String {
    let parser = Parser::new(markdown);
    let mut output = String::new();
    let mut style_stack: Vec<TextStyle> = vec![TextStyle::Normal];
    let mut pending_link_url: Option<String> = None;
    let mut in_code_block = false;
    let mut code_block_content = String::new();

    for event in parser {
        match event {
            // Headings → Bold
            Event::Start(Tag::Heading { .. }) => {
                style_stack.push(TextStyle::Bold);
            }
            Event::End(TagEnd::Heading(_)) => {
                style_stack.pop();
                output.push_str("\n\n");
            }

            // Strong → Bold
            Event::Start(Tag::Strong) => {
                let current = *style_stack.last().unwrap_or(&TextStyle::Normal);
                let new_style = match current {
                    TextStyle::Italic => TextStyle::BoldItalic,
                    _ => TextStyle::Bold,
                };
                style_stack.push(new_style);
            }
            Event::End(TagEnd::Strong) => {
                style_stack.pop();
            }

            // Emphasis → Italic
            Event::Start(Tag::Emphasis) => {
                let current = *style_stack.last().unwrap_or(&TextStyle::Normal);
                let new_style = match current {
                    TextStyle::Bold => TextStyle::BoldItalic,
                    _ => TextStyle::Italic,
                };
                style_stack.push(new_style);
            }
            Event::End(TagEnd::Emphasis) => {
                style_stack.pop();
            }

            // Lists → Bullet points
            Event::Start(Tag::Item) => {
                output.push_str("• ");
            }
            Event::End(TagEnd::Item) => {
                output.push('\n');
            }

            // Blockquotes → Italic
            Event::Start(Tag::BlockQuote(_)) => {
                style_stack.push(TextStyle::Italic);
            }
            Event::End(TagEnd::BlockQuote(_)) => {
                style_stack.pop();
                output.push('\n');
            }

            // Links → text (url)
            Event::Start(Tag::Link { dest_url, .. }) => {
                pending_link_url = Some(dest_url.to_string());
            }
            Event::End(TagEnd::Link) => {
                if let Some(url) = pending_link_url.take() {
                    output.push_str(&format!(" ({})", url));
                }
            }

            // Code blocks → Remove or Carbon URL
            Event::Start(Tag::CodeBlock(_)) => {
                in_code_block = true;
                code_block_content.clear();
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                if options.use_carbon {
                    // Generate Carbon URL (simplified - see Task 5)
                    output.push_str("[Code image: carbon.now.sh]\n");
                }
                // Otherwise: skip code block entirely
            }

            // Inline code → Just text
            Event::Code(code) => {
                output.push_str(&code);
            }

            // Text → Apply current style
            Event::Text(text) => {
                if in_code_block {
                    code_block_content.push_str(&text);
                } else {
                    let styled = apply_style(&text, *style_stack.last().unwrap_or(&TextStyle::Normal));
                    output.push_str(&styled);
                }
            }

            // Soft/Hard breaks
            Event::SoftBreak | Event::HardBreak => {
                output.push('\n');
            }

            // Paragraphs
            Event::End(TagEnd::Paragraph) => {
                output.push_str("\n\n");
            }

            _ => {}
        }
    }

    output.trim().to_string()
}

fn apply_style(text: &str, style: TextStyle) -> String {
    match style {
        TextStyle::Normal => text.to_string(),
        TextStyle::Bold => to_bold(text),
        TextStyle::Italic => to_italic(text),
        TextStyle::BoldItalic => {
            // For simplicity, just use bold
            // Could implement to_bold_italic if needed
            to_bold(text)
        }
    }
}
```

### Step 3.3: Add Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn opts() -> ConvertOptions {
        ConvertOptions::default()
    }

    #[test]
    fn test_bold() {
        let result = convert("**hello**", &opts());
        assert_eq!(result, "𝗵𝗲𝗹𝗹𝗼");
    }

    #[test]
    fn test_italic() {
        let result = convert("*hello*", &opts());
        assert_eq!(result, "𝘩𝘦𝘭𝘭𝘰");
    }

    #[test]
    fn test_header() {
        let result = convert("# Header", &opts());
        assert_eq!(result, "𝗛𝗲𝗮𝗱𝗲𝗿");
    }

    #[test]
    fn test_list() {
        let result = convert("- item one\n- item two", &opts());
        assert!(result.contains("• item one"));
        assert!(result.contains("• item two"));
    }

    #[test]
    fn test_link() {
        let result = convert("[click here](https://example.com)", &opts());
        assert_eq!(result, "click here (https://example.com)");
    }

    #[test]
    fn test_blockquote() {
        let result = convert("> quoted text", &opts());
        assert_eq!(result, "𝘲𝘶𝘰𝘵𝘦𝘥 𝘵𝘦𝘹𝘵");
    }

    #[test]
    fn test_inline_code() {
        let result = convert("Use `println!` macro", &opts());
        assert!(result.contains("println!"));
    }

    #[test]
    fn test_code_block_removed() {
        let result = convert("```rust\nfn main() {}\n```", &opts());
        assert!(!result.contains("fn main"));
    }

    #[test]
    fn test_preserves_emoji() {
        let result = convert("Hello 🚀 world", &opts());
        assert!(result.contains("🚀"));
    }
}
```

### Step 3.4: Run Tests

```bash
cargo test converter
```

---

## Definition of Done

- [ ] Headers convert to bold text.
- [ ] `**text**` converts to bold Unicode.
- [ ] `*text*` converts to italic Unicode.
- [ ] Lists use `•` bullet character.
- [ ] Blockquotes convert to italic.
- [ ] Links become `text (url)` format.
- [ ] Inline code removes backticks.
- [ ] Code blocks are removed (or Carbon URL if flag set).
- [ ] All unit tests pass.

---

## Files Changed

| File | Change |
|------|--------|
| `src/converter.rs` | Implemented |
