use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum CodeBlocksArg {
    Omit,
    Text,
    Carbon,
    Image,
}

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
}

#[derive(Parser, Debug)]
#[command(name = "mdtolinkedin")]
#[command(version, about = "Convert Markdown to LinkedIn-compatible text")]
pub struct Cli {
    /// Input Markdown file (reads from stdin if omitted)
    pub input: Option<PathBuf>,

    /// Output file (writes to stdout if omitted)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Generate Carbon.now.sh URLs for code blocks
    #[arg(long, conflicts_with = "code_blocks")]
    pub carbon: bool,

    /// Code block handling: omit, text, or carbon
    #[arg(long, value_enum, value_name = "MODE")]
    pub code_blocks: Option<CodeBlocksArg>,

    /// Suppress character limit warning
    #[arg(long)]
    pub no_warn: bool,

    /// Custom character limit for warnings
    #[arg(long, value_name = "N")]
    pub max_chars: Option<usize>,

    /// Custom bullet character for unordered lists
    #[arg(long, value_name = "CHAR")]
    pub bullet: Option<String>,

    /// Preserve trailing newlines in output
    #[arg(long)]
    pub no_trim: bool,

    /// Disable Unicode mapping (plain text output)
    #[arg(long)]
    pub plain: bool,

    /// Output format: text or json
    #[arg(long, value_enum, value_name = "FORMAT", default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,

    /// Code image output directory (for --code-blocks image)
    #[arg(long, value_name = "DIR", default_value = "code-images")]
    pub code_image_dir: String,

    /// Code image theme (syntect preset)
    #[arg(long, value_name = "NAME", default_value = "InspiredGitHub")]
    pub code_image_theme: String,

    /// Code image font path (TTF/OTF)
    #[arg(long, value_name = "PATH")]
    pub code_image_font: Option<PathBuf>,

    /// Code image font size (px)
    #[arg(long, value_name = "PX", default_value_t = 16.0)]
    pub code_image_font_size: f32,

    /// Code image background color (hex)
    #[arg(long, value_name = "HEX", default_value = "#ffffff")]
    pub code_image_bg: String,

    /// Code image padding (px)
    #[arg(long, value_name = "PX", default_value_t = 24)]
    pub code_image_padding: u32,
}
