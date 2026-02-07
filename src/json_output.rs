pub fn format_json(text: &str, char_count: usize, limit: usize) -> String {
    let escaped = json_escape(text);
    format!(
        "{{\"text\":\"{}\",\"char_count\":{},\"limit\":{},\"limit_exceeded\":{}}}",
        escaped,
        char_count,
        limit,
        char_count > limit
    )
}

pub fn json_escape(input: &str) -> String {
    let mut out = String::new();
    for ch in input.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(ch),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::converter::{convert, ConvertOptions};

    #[test]
    fn test_fixture_json_format() {
        let input = std::fs::read_to_string("tests/fixtures/json.md").unwrap();
        let expected = std::fs::read_to_string("tests/fixtures/json.txt").unwrap();
        let options = ConvertOptions::default();
        let converted = convert(&input, &options);
        let char_count = converted.chars().count();
        let output = format_json(&converted, char_count, 3000);
        assert_eq!(output, expected.trim_end_matches('\n'));
    }

    #[test]
    fn test_json_escape_quotes() {
        let escaped = json_escape("a\"b");
        assert_eq!(escaped, "a\\\"b");
    }
}
