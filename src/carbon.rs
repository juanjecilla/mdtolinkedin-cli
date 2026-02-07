pub fn carbon_url(code: &str, language: Option<&str>) -> String {
    let encoded_code = percent_encode(code);
    let mut url = format!("https://carbon.now.sh/?code={}", encoded_code);

    if let Some(lang) = language {
        if !lang.is_empty() {
            let encoded_lang = percent_encode(lang);
            url.push_str("&l=");
            url.push_str(&encoded_lang);
        }
    }

    url
}

fn percent_encode(input: &str) -> String {
    let mut out = String::new();

    for b in input.as_bytes() {
        match *b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*b as char)
            }
            _ => {
                out.push('%');
                out.push(hex_digit(b >> 4));
                out.push(hex_digit(b & 0x0F));
            }
        }
    }

    out
}

fn hex_digit(n: u8) -> char {
    match n {
        0..=9 => (b'0' + n) as char,
        10..=15 => (b'A' + (n - 10)) as char,
        _ => '0',
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_carbon_url_basic() {
        let url = carbon_url("fn main() {}", Some("rust"));
        assert!(url.contains("https://carbon.now.sh/?code="));
        assert!(url.contains("&l=rust"));
    }

    #[test]
    fn test_percent_encode_space() {
        let encoded = percent_encode("a b");
        assert_eq!(encoded, "a%20b");
    }
}
