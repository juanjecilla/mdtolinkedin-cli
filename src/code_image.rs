use std::fs;
use std::path::{Path, PathBuf};

use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

use resvg::tiny_skia;
use resvg::usvg;
use usvg::{fontdb, TreeParsing, TreeTextToPath};

#[derive(Clone, Debug)]
pub struct CodeImageOptions {
    pub output_dir: PathBuf,
    pub theme: String,
    pub font_path: Option<PathBuf>,
    pub font_size: f32,
    pub background: String,
    pub padding: u32,
}

#[derive(Debug)]
pub struct CodeImagePaths {
    pub png: PathBuf,
    pub svg: PathBuf,
}

#[derive(Debug)]
pub struct CodeImageError(String);

impl std::fmt::Display for CodeImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CodeImageError {}

pub fn render_code_image(
    code: &str,
    language: Option<&str>,
    index: usize,
    options: &CodeImageOptions,
) -> Result<CodeImagePaths, CodeImageError> {
    fs::create_dir_all(&options.output_dir)
        .map_err(|err| CodeImageError(format!("create output dir failed: {}", err)))?;

    let base = sanitize_filename(language.unwrap_or("code"));
    let stem = format!("{}-{:03}", base, index + 1);
    let svg_path = options.output_dir.join(format!("{}.svg", stem));
    let png_path = options.output_dir.join(format!("{}.png", stem));

    let svg = render_svg(code, language, options)?;
    fs::write(&svg_path, svg.as_bytes())
        .map_err(|err| CodeImageError(format!("write svg failed: {}", err)))?;

    render_png(&svg, &png_path, options)?;

    Ok(CodeImagePaths {
        png: png_path,
        svg: svg_path,
    })
}

fn render_svg(
    code: &str,
    language: Option<&str>,
    options: &CodeImageOptions,
) -> Result<String, CodeImageError> {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = pick_theme(&ts, &options.theme)?;

    let syntax = language
        .and_then(|lang| ps.find_syntax_by_token(lang))
        .unwrap_or_else(|| ps.find_syntax_plain_text());

    let mut highlighter = HighlightLines::new(syntax, theme);

    let font_size = options.font_size;
    let char_width = font_size * 0.6;
    let line_height = font_size * 1.4;

    let mut lines: Vec<Vec<(String, String)>> = Vec::new();
    let mut max_cols: usize = 0;

    for line in LinesWithEndings::from(code) {
        let ranges = highlighter
            .highlight_line(line, &ps)
            .map_err(|err| CodeImageError(format!("highlight failed: {}", err)))?;

        let mut tokens: Vec<(String, String)> = Vec::new();
        let mut cols: usize = 0;

        for (style, text) in ranges {
            let text = text.trim_end_matches('\n');
            if text.is_empty() {
                continue;
            }
            let expanded = expand_tabs(text, 4);
            cols += expanded.chars().count();
            let color = format!(
                "#{:02X}{:02X}{:02X}",
                style.foreground.r, style.foreground.g, style.foreground.b
            );
            tokens.push((color, expanded));
        }

        max_cols = max_cols.max(cols);
        lines.push(tokens);
    }

    if lines.is_empty() {
        lines.push(Vec::new());
    }

    let width = (max_cols as f32 * char_width + (options.padding * 2) as f32).ceil() as u32;
    let height = ((lines.len() as f32) * line_height + (options.padding * 2) as f32).ceil() as u32;

    let font_family = options
        .font_path
        .as_ref()
        .and_then(|p| p.file_stem().map(|s| s.to_string_lossy().to_string()))
        .unwrap_or_else(|| "monospace".to_string());

    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">",
        width, height, width, height
    ));
    svg.push_str(&format!(
        "<rect width=\"100%\" height=\"100%\" fill=\"{}\" rx=\"6\" />",
        options.background
    ));

    let mut y = options.padding as f32 + font_size;
    for line_tokens in lines {
        let mut x = options.padding as f32;
        for (color, text) in line_tokens {
            let escaped = escape_xml(&text);
            svg.push_str(&format!(
                "<text x=\"{:.1}\" y=\"{:.1}\" fill=\"{}\" font-family=\"{}\" font-size=\"{}\" xml:space=\"preserve\">{}</text>",
                x,
                y,
                color,
                font_family,
                font_size,
                escaped
            ));
            let token_cols = text.chars().count() as f32;
            x += token_cols * char_width;
        }
        y += line_height;
    }

    svg.push_str("</svg>");
    Ok(svg)
}

fn render_png(
    svg: &str,
    out_path: &Path,
    options: &CodeImageOptions,
) -> Result<(), CodeImageError> {
    let opt = usvg::Options::default();

    let mut fontdb = fontdb::Database::new();
    if let Some(font_path) = &options.font_path {
        fontdb
            .load_font_file(font_path)
            .map_err(|err| CodeImageError(format!("load font failed: {}", err)))?;
    } else {
        fontdb.load_system_fonts();
    }

    let mut tree = usvg::Tree::from_data(svg.as_bytes(), &opt)
        .map_err(|err| CodeImageError(format!("parse svg failed: {}", err)))?;
    tree.convert_text(&fontdb);

    let rtree = resvg::Tree::from_usvg(&tree);
    let size = rtree.size.to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(size.width(), size.height())
        .ok_or_else(|| CodeImageError("create pixmap failed".to_string()))?;

    rtree.render(tiny_skia::Transform::default(), &mut pixmap.as_mut());

    pixmap
        .save_png(out_path)
        .map_err(|err| CodeImageError(format!("save png failed: {}", err)))?;

    Ok(())
}

fn pick_theme<'a>(ts: &'a ThemeSet, name: &str) -> Result<&'a Theme, CodeImageError> {
    if let Some(theme) = ts.themes.get(name) {
        return Ok(theme);
    }
    ts.themes
        .values()
        .next()
        .ok_or_else(|| CodeImageError("no themes available".to_string()))
}

/// Sanitize a string for use as a filename stem (e.g. language name).
fn sanitize_filename(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
            out.push(ch);
        } else if ch == ' ' || ch == '.' {
            out.push('_');
        }
        // Skip other characters to avoid invalid filenames
    }
    if out.is_empty() {
        out.push_str("code");
    }
    out
}

fn expand_tabs(input: &str, tab_width: usize) -> String {
    let mut out = String::new();
    let mut col = 0usize;
    for ch in input.chars() {
        if ch == '\t' {
            let spaces = tab_width - (col % tab_width);
            out.extend(std::iter::repeat_n(' ', spaces));
            col += spaces;
        } else {
            out.push(ch);
            col += 1;
        }
    }
    out
}

fn escape_xml(input: &str) -> String {
    let mut out = String::new();
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(ch),
        }
    }
    out
}
