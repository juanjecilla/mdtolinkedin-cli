use clap::Parser;
use std::path::PathBuf;

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
    #[arg(long)]
    pub carbon: bool,

    /// Suppress character limit warning
    #[arg(long)]
    pub no_warn: bool,
}