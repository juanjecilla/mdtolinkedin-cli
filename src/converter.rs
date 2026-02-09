use crate::carbon::carbon_url;
use crate::code_image::{render_code_image, CodeImageOptions};
use crate::unicode::{to_bold, to_bold_italic, to_italic};
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};
use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq)]
enum TextStyle {
    Normal,
    Bold,
    Italic,
    BoldItalic,
}

pub struct ConvertOptions {
    pub code_block_mode: CodeBlockMode,
    pub bullet: String,
    pub trim_output: bool,
    pub plain: bool,
    pub code_image: Option<CodeImageOptions>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeBlockMode {
    Omit,
    Text,
    Carbon,
    Image,
}

#[derive(Debug, Clone)]
struct ListContext {
    ordered: bool,
    next_index: u64,
}

impl Default for ConvertOptions {
    fn default() -> Self {
        Self {
            code_block_mode: CodeBlockMode::Omit,
            bullet: "•".to_string(),
            trim_output: true,
            plain: false,
            code_image: None,
        }
    }
}

pub fn convert(markdown: &str, options: &ConvertOptions) -> String {
    let parser = Parser::new(markdown);
    let mut output = String::with_capacity(markdown.len());
    let mut style_stack: Vec<TextStyle> = Vec::with_capacity(8);
    style_stack.push(TextStyle::Normal);
    let mut pending_link_url: Option<String> = None;
    let mut in_code_block = false;
    let mut code_block_content = String::with_capacity(256);
    let mut code_block_language: Option<String> = None;
    let mut code_block_index: usize = 0;
    let mut list_stack: Vec<ListContext> = Vec::with_capacity(8);

    for event in parser {
        match event {
            // Headings → Bold
            Event::Start(Tag::Heading { .. }) => {
                style_stack.push(TextStyle::Bold);
            }
            Event::End(TagEnd::Heading(_)) => {
                style_stack.pop();
                ensure_blank_line(&mut output);
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
            Event::Start(Tag::List(start)) => {
                let ctx = ListContext {
                    ordered: start.is_some(),
                    next_index: start.unwrap_or(1),
                };
                list_stack.push(ctx);
            }
            Event::End(TagEnd::List(_)) => {
                list_stack.pop();
                ensure_blank_line(&mut output);
            }
            Event::Start(Tag::Item) => {
                if !output.is_empty() && !output.ends_with('\n') {
                    output.push('\n');
                }
                if let Some(ctx) = list_stack.last_mut() {
                    if ctx.ordered {
                        let _ = write!(output, "{}. ", ctx.next_index);
                        ctx.next_index += 1;
                    } else {
                        output.push_str(&options.bullet);
                        output.push(' ');
                    }
                } else {
                    output.push_str(&options.bullet);
                    output.push(' ');
                }
            }
            Event::End(TagEnd::Item) => {
                ensure_newline(&mut output);
            }

            // Blockquotes → Italic
            Event::Start(Tag::BlockQuote) => {
                style_stack.push(TextStyle::Italic);
            }
            Event::End(TagEnd::BlockQuote) => {
                style_stack.pop();
                ensure_newline(&mut output);
            }

            // Links → text (url)
            Event::Start(Tag::Link { dest_url, .. }) => {
                pending_link_url = Some(dest_url.to_string());
            }
            Event::End(TagEnd::Link) => {
                if let Some(url) = pending_link_url.take() {
                    output.push_str(" (");
                    output.push_str(&url);
                    output.push(')');
                }
            }
            // Images → alt (url)
            Event::Start(Tag::Image { dest_url, .. }) => {
                pending_link_url = Some(dest_url.to_string());
            }
            Event::End(TagEnd::Image) => {
                if let Some(url) = pending_link_url.take() {
                    output.push_str(" (");
                    output.push_str(&url);
                    output.push(')');
                }
            }

            // Code blocks → Omit, text, or Carbon URL
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                code_block_content.clear();
                code_block_language = match kind {
                    CodeBlockKind::Fenced(info) => {
                        info.split_whitespace().next().map(|s| s.to_string())
                    }
                    CodeBlockKind::Indented => None,
                };
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                match options.code_block_mode {
                    CodeBlockMode::Omit => {}
                    CodeBlockMode::Text => {
                        output.push_str(code_block_content.trim_end());
                        output.push_str("\n\n");
                    }
                    CodeBlockMode::Carbon => {
                        let url = carbon_url(&code_block_content, code_block_language.as_deref());
                        output.push_str(&url);
                        output.push_str("\n\n");
                    }
                    CodeBlockMode::Image => {
                        if let Some(code_image_options) = &options.code_image {
                            match render_code_image(
                                &code_block_content,
                                code_block_language.as_deref(),
                                code_block_index,
                                code_image_options,
                            ) {
                                Ok(paths) => {
                                    let _ = write!(
                                        output,
                                        "Code image (png): {}\n",
                                        paths.png.display()
                                    );
                                    let _ = write!(
                                        output,
                                        "Code image (svg): {}\n\n",
                                        paths.svg.display()
                                    );
                                }
                                Err(err) => {
                                    eprintln!("Error rendering code image: {}", err);
                                }
                            }
                        }
                    }
                }
                code_block_language = None;
                code_block_index += 1;
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
                    let style = *style_stack.last().unwrap_or(&TextStyle::Normal);
                    if options.plain || style == TextStyle::Normal {
                        output.push_str(&text);
                    } else {
                        let styled = apply_style(&text, style, false);
                        output.push_str(&styled);
                    }
                }
            }

            // Soft/Hard breaks
            Event::SoftBreak => {
                if in_code_block {
                    code_block_content.push('\n');
                } else {
                    ensure_newline(&mut output);
                }
            }
            Event::HardBreak => {
                if in_code_block {
                    code_block_content.push('\n');
                } else {
                    ensure_newline(&mut output);
                }
            }

            // Paragraphs
            Event::End(TagEnd::Paragraph) => {
                ensure_blank_line(&mut output);
            }

            _ => {}
        }
    }

    if options.trim_output {
        output.trim().to_string()
    } else {
        output
    }
}

fn apply_style(text: &str, style: TextStyle, plain: bool) -> String {
    if plain {
        return text.to_string();
    }

    match style {
        TextStyle::Normal => text.to_string(),
        TextStyle::Bold => to_bold(text),
        TextStyle::Italic => to_italic(text),
        TextStyle::BoldItalic => to_bold_italic(text),
    }
}

fn ensure_newline(output: &mut String) {
    if !output.ends_with('\n') {
        output.push('\n');
    }
}

fn ensure_blank_line(output: &mut String) {
    if output.ends_with("\n\n") {
        return;
    }

    if output.ends_with('\n') {
        output.push('\n');
    } else {
        output.push_str("\n\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unicode::{to_bold, to_bold_italic};

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
    fn test_ordered_list() {
        let result = convert("1. first\n2. second", &opts());
        assert!(result.contains("1. first"));
        assert!(result.contains("2. second"));
    }

    #[test]
    fn test_link() {
        let result = convert("[click here](https://example.com)", &opts());
        assert_eq!(result, "click here (https://example.com)");
    }

    #[test]
    fn test_link_in_heading() {
        let result = convert("# [Title](https://example.com)", &opts());
        assert!(result.contains("𝐓𝐢𝐭𝐥𝐞"));
        assert!(result.contains("(https://example.com)"));
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
    fn test_bold_italic() {
        let result = convert("***hi***", &opts());
        assert_eq!(result, "\u{1D489}\u{1D48A}");
    }

    #[test]
    fn test_bold_with_italic_nested() {
        let result = convert("**A *B* C**", &opts());
        let expected = format!("{}{}{}", to_bold("A "), to_bold_italic("B"), to_bold(" C"));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_code_block_removed() {
        let result = convert("```rust\nfn main() {}\n```", &opts());
        assert!(!result.contains("fn main"));
    }

    #[test]
    fn test_code_block_text_mode() {
        let mut options = opts();
        options.code_block_mode = CodeBlockMode::Text;
        let result = convert("```rust\nfn main() {}\n```", &options);
        assert!(result.contains("fn main() {}"));
    }

    #[test]
    fn test_code_block_carbon_mode() {
        let mut options = opts();
        options.code_block_mode = CodeBlockMode::Carbon;
        let result = convert("```rust\nfn main() {}\n```", &options);
        assert!(result.contains("https://carbon.now.sh/"));
        assert!(result.contains("code="));
    }

    #[test]
    fn test_preserves_emoji() {
        let result = convert("Hello 🚀 world", &opts());
        assert!(result.contains("🚀"));
    }

    #[test]
    fn test_fixture_whitespace() {
        let input = std::fs::read_to_string("tests/fixtures/whitespace.md").unwrap();
        let expected = std::fs::read_to_string("tests/fixtures/whitespace.txt").unwrap();
        let result = convert(&input, &opts());
        assert_eq!(result, expected.trim_end_matches('\n'));
    }

    #[test]
    fn test_fixture_nested() {
        let input = std::fs::read_to_string("tests/fixtures/nested.md").unwrap();
        let expected = std::fs::read_to_string("tests/fixtures/nested.txt").unwrap();
        let result = convert(&input, &opts());
        assert_eq!(result, expected.trim_end_matches('\n'));
    }

    #[test]
    fn test_fixture_links_and_styles() {
        let input = std::fs::read_to_string("tests/fixtures/links_and_styles.md").unwrap();
        let expected = std::fs::read_to_string("tests/fixtures/links_and_styles.txt").unwrap();
        let result = convert(&input, &opts());
        assert_eq!(result, expected.trim_end_matches('\n'));
    }

    #[test]
    fn test_fixture_mixed_lists() {
        let input = std::fs::read_to_string("tests/fixtures/mixed_lists.md").unwrap();
        let expected = std::fs::read_to_string("tests/fixtures/mixed_lists.txt").unwrap();
        let result = convert(&input, &opts());
        assert_eq!(result, expected.trim_end_matches('\n'));
    }

    #[test]
    fn test_fixture_code_blocks_omit() {
        let input = std::fs::read_to_string("tests/fixtures/code_blocks.md").unwrap();
        let expected = std::fs::read_to_string("tests/fixtures/code_blocks_omit.txt").unwrap();
        let result = convert(&input, &opts());
        assert_eq!(result, expected.trim_end_matches('\n'));
    }

    #[test]
    fn test_fixture_code_blocks_text() {
        let input = std::fs::read_to_string("tests/fixtures/code_blocks.md").unwrap();
        let expected = std::fs::read_to_string("tests/fixtures/code_blocks_text.txt").unwrap();
        let mut options = opts();
        options.code_block_mode = CodeBlockMode::Text;
        let result = convert(&input, &options);
        assert_eq!(result, expected.trim_end_matches('\n'));
    }

    #[test]
    fn test_fixture_code_blocks_carbon() {
        let input = std::fs::read_to_string("tests/fixtures/code_blocks.md").unwrap();
        let expected = std::fs::read_to_string("tests/fixtures/code_blocks_carbon.txt").unwrap();
        let mut options = opts();
        options.code_block_mode = CodeBlockMode::Carbon;
        let result = convert(&input, &options);
        assert_eq!(result, expected.trim_end_matches('\n'));
    }

    #[test]
    fn test_fixture_nested_lists() {
        let input = std::fs::read_to_string("tests/fixtures/nested_lists.md").unwrap();
        let expected = std::fs::read_to_string("tests/fixtures/nested_lists.txt").unwrap();
        let result = convert(&input, &opts());
        assert_eq!(result, expected.trim_end_matches('\n'));
    }

    #[test]
    fn test_fixture_ordered_list_start() {
        let input = std::fs::read_to_string("tests/fixtures/ordered_list_start.md").unwrap();
        let expected = std::fs::read_to_string("tests/fixtures/ordered_list_start.txt").unwrap();
        let result = convert(&input, &opts());
        assert_eq!(result, expected.trim_end_matches('\n'));
    }

    #[test]
    fn test_fixture_whitespace_no_trim() {
        let input = std::fs::read_to_string("tests/fixtures/whitespace.md").unwrap();
        let expected = std::fs::read_to_string("tests/fixtures/whitespace_notrim.txt").unwrap();
        let mut options = opts();
        options.trim_output = false;
        let result = convert(&input, &options);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_fixture_plain_mode() {
        let input = std::fs::read_to_string("tests/fixtures/plain.md").unwrap();
        let expected = std::fs::read_to_string("tests/fixtures/plain.txt").unwrap();
        let mut options = opts();
        options.plain = true;
        let result = convert(&input, &options);
        assert_eq!(result, expected.trim_end_matches('\n'));
    }

    #[test]
    fn test_fixture_images() {
        let input = std::fs::read_to_string("tests/fixtures/images.md").unwrap();
        let expected = std::fs::read_to_string("tests/fixtures/images.txt").unwrap();
        let result = convert(&input, &opts());
        assert_eq!(result, expected.trim_end_matches('\n'));
    }

    #[test]
    fn test_fixture_common() {
        let input = std::fs::read_to_string("tests/fixtures/common.md").unwrap();
        let expected = std::fs::read_to_string("tests/fixtures/common.txt").unwrap();
        let result = convert(&input, &opts());
        assert_eq!(result, expected.trim_end_matches('\n'));
    }
}
