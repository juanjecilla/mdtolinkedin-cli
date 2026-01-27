mod cli;
mod io;
mod converter;
mod unicode;

use clap::Parser;
use cli::Cli;
use converter::ConvertOptions;

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
    let options = ConvertOptions {
        use_carbon: args.carbon,
    };
    let output = converter::convert(&input, &options);

    // Character count warning
    let char_count = output.chars().count();
    if !args.no_warn && char_count > 3000 {
        eprintln!(
            "⚠️  Warning: Output is {} characters (LinkedIn limit: 3000)",
            char_count
        );
    }

    // Write output
    if let Err(e) = io::write_output(args.output.as_ref(), &output) {
        eprintln!("Error writing output: {}", e);
        std::process::exit(1);
    }
}
