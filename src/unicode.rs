/// Convert ASCII letters to Mathematical Bold Unicode.
/// 
/// # Example
/// ```
/// use mdtolinkedin::unicode::to_bold;
/// assert_eq!(to_bold("Hello"), "𝐇𝐞𝐥𝐥𝐨");
/// ```
pub fn to_bold(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            'A'..='Z' => char::from_u32(0x1D400 + (c as u32 - 'A' as u32)).unwrap_or(c),
            'a'..='z' => char::from_u32(0x1D41A + (c as u32 - 'a' as u32)).unwrap_or(c),
            _ => c,
        })
        .collect()
}

/// Convert ASCII letters to Mathematical Italic Unicode.
/// 
/// # Example
/// ```
/// use mdtolinkedin::unicode::to_italic;
/// assert_eq!(to_italic("Hello"), "𝐻𝑒𝑙𝑙𝑜");
/// ```
pub fn to_italic(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            'A'..='Z' => char::from_u32(0x1D434 + (c as u32 - 'A' as u32)).unwrap_or(c),
            'a'..='z' => char::from_u32(0x1D44E + (c as u32 - 'a' as u32)).unwrap_or(c),
            _ => c,
        })
        .collect()
}

/// Convert ASCII letters to Mathematical Bold Italic Unicode.
#[allow(dead_code)]
pub fn to_bold_italic(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            'A'..='Z' => char::from_u32(0x1D468 + (c as u32 - 'A' as u32)).unwrap_or(c),
            'a'..='z' => char::from_u32(0x1D482 + (c as u32 - 'a' as u32)).unwrap_or(c),
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bold_uppercase() {
        assert_eq!(to_bold("ABC"), "𝐀𝐁𝐂");
    }

    #[test]
    fn test_bold_lowercase() {
        assert_eq!(to_bold("abc"), "𝐚𝐛𝐜");
    }

    #[test]
    fn test_bold_mixed() {
        assert_eq!(to_bold("Hello World!"), "𝐇𝐞𝐥𝐥𝐨 𝐖𝐨𝐫𝐥𝐝!");
    }

    #[test]
    fn test_bold_numbers_unchanged() {
        assert_eq!(to_bold("Test123"), "𝐓𝐞𝐬𝐭123");
    }

    #[test]
    fn test_italic_lowercase() {
        assert_eq!(to_italic("hello"), "\u{1D455}\u{1D452}\u{1D459}\u{1D459}\u{1D45C}");
    }

    #[test]
    fn test_italic_uppercase() {
        assert_eq!(to_italic("HELLO"), "𝐻𝐸𝐿𝐿𝑂");
    }

    #[test]
    fn test_preserves_emoji() {
        assert_eq!(to_bold("Hello 🚀"), "𝐇𝐞𝐥𝐥𝐨 🚀");
    }

    #[test]
    fn test_preserves_punctuation() {
        assert_eq!(to_bold("Hello, World!"), "𝐇𝐞𝐥𝐥𝐨, 𝐖𝐨𝐫𝐥𝐝!");
    }
}
