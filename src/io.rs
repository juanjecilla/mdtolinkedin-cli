use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

/// Read input from file or stdin.
pub fn read_input(path: Option<&PathBuf>) -> io::Result<String> {
    match path {
        Some(p) => fs::read_to_string(p),
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        }
    }
}

/// Write output to file or stdout.
pub fn write_output(path: Option<&PathBuf>, content: &str) -> io::Result<()> {
    match path {
        Some(p) => fs::write(p, content),
        None => {
            io::stdout().write_all(content.as_bytes())?;
            io::stdout().write_all(b"\n")?;
            Ok(())
        }
    }
}