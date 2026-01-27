use pulldown_cmark::{Event, Parser, Tag, TagEnd};
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
            Event::Start(Tag::BlockQuote) => {
                style_stack.push(TextStyle::Italic);
            }
            Event::End(TagEnd::BlockQuote) => {
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
                    // Generate Carbon URL (simplified for now)
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
            Event::SoftBreak => {
                output.push('\n');
            }
            Event::HardBreak => {
                output.push_str("\n");
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

#[cfg(test)]
mod tests {
    use super::*;

    fn opts() -> ConvertOptions {
        ConvertOptions::default()
    }

    #[test]
    fn test_bold() {
        let result = convert("**hello**", &opts());
        assert_eq!(result, "𝐡𝐞𝐥𝐥𝐨");
    }

    #[test]
    fn test_italic() {
        let result = convert("*hello*", &opts());
        // Mathematical Italic: h=U+1D455, e=U+1D452, l=U+1D459, o=U+1D45C
        assert_eq!(result, "\u{1D455}\u{1D452}\u{1D459}\u{1D459}\u{1D45C}");
    }

    #[test]
    fn test_header() {
        let result = convert("# Header", &opts());
        assert_eq!(result, "𝐇𝐞𝐚𝐝𝐞𝐫");
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
        assert_eq!(result, "𝑞𝑢𝑜𝑡𝑒𝑑 𝑡𝑒𝑥𝑡");
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