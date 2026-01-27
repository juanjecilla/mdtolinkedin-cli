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
    drop(stdin);

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
