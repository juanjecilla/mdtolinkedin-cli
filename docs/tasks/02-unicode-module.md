# Task 2: Unicode Transformer Module

**Phase:** 2  
**Estimated Effort:** 1 hour  
**Dependencies:** Task 1 (Project Setup)

---

## Context

LinkedIn displays Unicode Mathematical Alphanumeric Symbols as styled text. This module converts ASCII letters to their bold/italic Unicode equivalents.

### Unicode Ranges

| Style | Uppercase Range | Lowercase Range |
|-------|-----------------|-----------------|
| Bold | U+1D400 – U+1D419 | U+1D41A – U+1D433 |
| Italic | U+1D434 – U+1D44D | U+1D44E – U+1D467 |

**Note:** Numbers and special characters remain unchanged.

## Goal

Implement `src/unicode.rs` with functions to convert text to bold and italic Unicode.

---

## Implementation Steps

### Step 2.1: Implement Bold Conversion

**File:** `src/unicode.rs`

```rust
/// Convert ASCII letters to Mathematical Bold Unicode.
/// 
/// # Example
/// ```
/// assert_eq!(to_bold("Hello"), "𝗛𝗲𝗹𝗹𝗼");
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
```

### Step 2.2: Implement Italic Conversion

```rust
/// Convert ASCII letters to Mathematical Italic Unicode.
/// 
/// # Example
/// ```
/// assert_eq!(to_italic("Hello"), "𝘏𝘦𝘭𝘭𝘰");
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
```

### Step 2.3: Implement Bold-Italic (Optional)

For text that is both bold and italic (`***text***`):

```rust
/// Convert ASCII letters to Mathematical Bold Italic Unicode.
pub fn to_bold_italic(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            'A'..='Z' => char::from_u32(0x1D468 + (c as u32 - 'A' as u32)).unwrap_or(c),
            'a'..='z' => char::from_u32(0x1D482 + (c as u32 - 'a' as u32)).unwrap_or(c),
            _ => c,
        })
        .collect()
}
```

### Step 2.4: Add Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bold_uppercase() {
        assert_eq!(to_bold("ABC"), "𝗔𝗕𝗖");
    }

    #[test]
    fn test_bold_lowercase() {
        assert_eq!(to_bold("abc"), "𝗮𝗯𝗰");
    }

    #[test]
    fn test_bold_mixed() {
        assert_eq!(to_bold("Hello World!"), "𝗛𝗲𝗹𝗹𝗼 𝗪𝗼𝗿𝗹𝗱!");
    }

    #[test]
    fn test_bold_numbers_unchanged() {
        assert_eq!(to_bold("Test123"), "𝗧𝗲𝘀𝘁123");
    }

    #[test]
    fn test_italic_lowercase() {
        assert_eq!(to_italic("hello"), "𝘩𝘦𝘭𝘭𝘰");
    }

    #[test]
    fn test_italic_uppercase() {
        assert_eq!(to_italic("HELLO"), "𝘏𝘌𝘓𝘓𝘖");
    }

    #[test]
    fn test_preserves_emoji() {
        assert_eq!(to_bold("Hello 🚀"), "𝗛𝗲𝗹𝗹𝗼 🚀");
    }

    #[test]
    fn test_preserves_punctuation() {
        assert_eq!(to_bold("Hello, World!"), "𝗛𝗲𝗹𝗹𝗼, 𝗪𝗼𝗿𝗹𝗱!");
    }
}
```

### Step 2.5: Run Tests

```bash
cargo test unicode
```

---

## Definition of Done

- [ ] `to_bold()` converts A-Z and a-z to bold Unicode.
- [ ] `to_italic()` converts A-Z and a-z to italic Unicode.
- [ ] Numbers, punctuation, and emoji are preserved.
- [ ] All unit tests pass.
- [ ] `cargo test` succeeds.

---

## Files Changed

| File | Change |
|------|--------|
| `src/unicode.rs` | Implemented |

---

## Reference: Unicode Mapping Table

| ASCII | Bold | Italic |
|-------|------|--------|
| A | 𝗔 (U+1D400) | 𝘈 (U+1D434) |
| B | 𝗕 (U+1D401) | 𝘉 (U+1D435) |
| ... | ... | ... |
| Z | 𝗭 (U+1D419) | 𝘡 (U+1D44D) |
| a | 𝗮 (U+1D41A) | 𝘢 (U+1D44E) |
| b | 𝗯 (U+1D41B) | 𝘣 (U+1D44F) |
| ... | ... | ... |
| z | 𝘇 (U+1D433) | 𝘻 (U+1D467) |
