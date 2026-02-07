use std::process::Command;
use std::io::Write;

#[test]
fn test_file_input_stdout() {
    // Create temp file
    let input = "**bold** text";
    let temp_path = "/tmp/mdtolinkedin_test.md";
    std::fs::write(temp_path, input).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_path])
        .output()
        .expect("Failed to run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("𝐛𝐨𝐥𝐝"));

    std::fs::remove_file(temp_path).ok();
}

#[test]
fn test_stdin_input() {
    use std::process::Stdio;

    let mut child = Command::new("cargo")
        .args(["run", "--"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn");

    let stdin = child.stdin.as_mut().unwrap();
    stdin.write_all(b"*italic*").unwrap();
    drop(child.stdin.take());

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("𝑖𝑡𝑎𝑙𝑖𝑐"));
}

#[test]
fn test_file_output() {
    let input = "# Header";
    let input_path = "/tmp/mdtolinkedin_in.md";
    let output_path = "/tmp/mdtolinkedin_out.txt";
    std::fs::write(input_path, input).unwrap();

    Command::new("cargo")
        .args(["run", "--", input_path, "-o", output_path])
        .output()
        .expect("Failed to run");

    let result = std::fs::read_to_string(output_path).unwrap();
    assert!(result.contains("𝐇𝐞𝐚𝐝𝐞𝐫"));

    std::fs::remove_file(input_path).ok();
    std::fs::remove_file(output_path).ok();
}

#[test]
fn test_character_warning() {
    // Create a long input
    let input = "a".repeat(3001);
    let temp_path = "/tmp/mdtolinkedin_long.md";
    std::fs::write(temp_path, &input).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_path])
        .output()
        .expect("Failed to run");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Warning"));
    assert!(stderr.contains("3001"));

    std::fs::remove_file(temp_path).ok();
}

#[test]
fn test_no_warn_flag() {
    let input = "a".repeat(3001);
    let temp_path = "/tmp/mdtolinkedin_nowarn.md";
    std::fs::write(temp_path, &input).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_path, "--no-warn"])
        .output()
        .expect("Failed to run");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains("Warning"));

    std::fs::remove_file(temp_path).ok();
}

#[test]
fn test_max_chars_flag() {
    let input = "a".repeat(11);
    let temp_path = "/tmp/mdtolinkedin_maxchars.md";
    std::fs::write(temp_path, &input).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_path, "--max-chars", "10"])
        .output()
        .expect("Failed to run");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Warning"));
    assert!(stderr.contains("limit: 10"));

    std::fs::remove_file(temp_path).ok();
}

#[test]
fn test_bullet_flag() {
    let input = "- item";
    let temp_path = "/tmp/mdtolinkedin_bullet.md";
    std::fs::write(temp_path, input).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_path, "--bullet", "-"])
        .output()
        .expect("Failed to run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("- item"));

    std::fs::remove_file(temp_path).ok();
}

#[test]
fn test_code_blocks_text_mode() {
    let input = "```\nfn main() {}\n```";
    let temp_path = "/tmp/mdtolinkedin_code_text.md";
    std::fs::write(temp_path, input).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_path, "--code-blocks", "text"])
        .output()
        .expect("Failed to run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("fn main() {}"));

    std::fs::remove_file(temp_path).ok();
}

#[test]
fn test_code_blocks_carbon_mode() {
    let input = "```rust\nfn main() {}\n```";
    let temp_path = "/tmp/mdtolinkedin_code_carbon.md";
    std::fs::write(temp_path, input).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_path, "--code-blocks", "carbon"])
        .output()
        .expect("Failed to run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("https://carbon.now.sh/"));

    std::fs::remove_file(temp_path).ok();
}

#[test]
fn test_bullet_flag_nested_lists() {
    let input = "1. first\n   - alpha\n2. second";
    let temp_path = "/tmp/mdtolinkedin_bullet_nested.md";
    std::fs::write(temp_path, input).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_path, "--bullet", "-"])
        .output()
        .expect("Failed to run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("- alpha"));

    std::fs::remove_file(temp_path).ok();
}

#[test]
fn test_no_trim_flag() {
    let input = "Hello";
    let temp_path = "/tmp/mdtolinkedin_notrim.md";
    std::fs::write(temp_path, input).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_path, "--no-trim"])
        .output()
        .expect("Failed to run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.ends_with('\n'));

    std::fs::remove_file(temp_path).ok();
}

#[test]
fn test_plain_flag() {
    let input = "**bold** and *italic*";
    let temp_path = "/tmp/mdtolinkedin_plain.md";
    std::fs::write(temp_path, input).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_path, "--plain"])
        .output()
        .expect("Failed to run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("bold and italic"));
    assert!(!stdout.contains("𝐛"));

    std::fs::remove_file(temp_path).ok();
}

#[test]
fn test_format_json() {
    let input = "**bold** text";
    let temp_path = "/tmp/mdtolinkedin_json.md";
    std::fs::write(temp_path, input).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", temp_path, "--format", "json"])
        .output()
        .expect("Failed to run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"text\""));
    assert!(stdout.contains("\"char_count\""));
    assert!(stdout.contains("\"limit\""));
    assert!(stdout.contains("\"limit_exceeded\""));
    assert!(stdout.contains("𝐛𝐨𝐥𝐝"));

    std::fs::remove_file(temp_path).ok();
}

#[test]
fn test_code_blocks_image_mode_creates_files() {
    let input = "```rust\nfn main() {}\n```";
    let temp_path = "/tmp/mdtolinkedin_code_image.md";
    let output_dir = "/tmp/mdtolinkedin_code_images";
    std::fs::write(temp_path, input).unwrap();
    std::fs::create_dir_all(output_dir).unwrap();

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            temp_path,
            "--code-blocks",
            "image",
            "--code-image-dir",
            output_dir,
        ])
        .output()
        .expect("Failed to run");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Code image (png):"));
    assert!(stdout.contains("Code image (svg):"));

    let png_exists = std::fs::read_dir(output_dir)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .any(|entry| entry.path().extension().map(|e| e == "png").unwrap_or(false));
    let svg_exists = std::fs::read_dir(output_dir)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .any(|entry| entry.path().extension().map(|e| e == "svg").unwrap_or(false));

    assert!(png_exists);
    assert!(svg_exists);

    std::fs::remove_file(temp_path).ok();
    let _ = std::fs::remove_dir_all(output_dir);
}
