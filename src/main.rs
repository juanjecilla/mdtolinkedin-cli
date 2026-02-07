use clap::Parser;
use mdtolinkedin::cli::{Cli, CodeBlocksArg, OutputFormat};
use mdtolinkedin::code_image::CodeImageOptions;
use mdtolinkedin::converter::{self, CodeBlockMode, ConvertOptions};
use mdtolinkedin::{io, json_output};

fn main() {
    let args = Cli::parse();

    // Read input
    let input = match io::read_input(args.input.as_ref()) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    // Convert
    let code_block_mode = if let Some(mode) = args.code_blocks {
        match mode {
            CodeBlocksArg::Omit => CodeBlockMode::Omit,
            CodeBlocksArg::Text => CodeBlockMode::Text,
            CodeBlocksArg::Carbon => CodeBlockMode::Carbon,
            CodeBlocksArg::Image => CodeBlockMode::Image,
        }
    } else if args.carbon {
        CodeBlockMode::Carbon
    } else {
        CodeBlockMode::Omit
    };

    let code_image_options = if code_block_mode == CodeBlockMode::Image {
        Some(CodeImageOptions {
            output_dir: args.code_image_dir.clone().into(),
            theme: args.code_image_theme.clone(),
            font_path: args.code_image_font.clone(),
            font_size: args.code_image_font_size,
            background: args.code_image_bg.clone(),
            padding: args.code_image_padding,
        })
    } else {
        None
    };

    let options = ConvertOptions {
        code_block_mode,
        bullet: args.bullet.unwrap_or_else(|| "•".to_string()),
        trim_output: !args.no_trim,
        plain: args.plain,
        code_image: code_image_options,
    };
    let output = converter::convert(&input, &options);

    // Character count warning
    let char_count = output.chars().count();
    let warn_limit = args.max_chars.unwrap_or(3000);
    if !args.no_warn && char_count > warn_limit {
        eprintln!(
            "⚠️  Warning: Output is {} characters (limit: {})",
            char_count, warn_limit
        );
    }

    let formatted_output = match args.format {
        OutputFormat::Text => output,
        OutputFormat::Json => json_output::format_json(&output, char_count, warn_limit),
    };

    let final_output = if args.format == OutputFormat::Text && args.no_trim {
        if formatted_output.ends_with('\n') {
            formatted_output
        } else {
            format!("{}\n", formatted_output)
        }
    } else {
        formatted_output
    };

    // Write output
    if let Err(e) = io::write_output(args.output.as_ref(), &final_output) {
        eprintln!("Error writing output: {}", e);
        std::process::exit(1);
    }
}
