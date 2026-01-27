# Unicode Mapping Reference

This document provides the complete mapping from ASCII letters to Unicode Mathematical Alphanumeric Symbols.

## Bold (Mathematical Bold)

| ASCII | Unicode | Code Point |
|-------|---------|------------|
| A | 𝗔 | U+1D400 |
| B | 𝗕 | U+1D401 |
| C | 𝗖 | U+1D402 |
| D | 𝗗 | U+1D403 |
| E | 𝗘 | U+1D404 |
| F | 𝗙 | U+1D405 |
| G | 𝗚 | U+1D406 |
| H | 𝗛 | U+1D407 |
| I | 𝗜 | U+1D408 |
| J | 𝗝 | U+1D409 |
| K | 𝗞 | U+1D40A |
| L | 𝗟 | U+1D40B |
| M | 𝗠 | U+1D40C |
| N | 𝗡 | U+1D40D |
| O | 𝗢 | U+1D40E |
| P | 𝗣 | U+1D40F |
| Q | 𝗤 | U+1D410 |
| R | 𝗥 | U+1D411 |
| S | 𝗦 | U+1D412 |
| T | 𝗧 | U+1D413 |
| U | 𝗨 | U+1D414 |
| V | 𝗩 | U+1D415 |
| W | 𝗪 | U+1D416 |
| X | 𝗫 | U+1D417 |
| Y | 𝗬 | U+1D418 |
| Z | 𝗭 | U+1D419 |
| a | 𝗮 | U+1D41A |
| b | 𝗯 | U+1D41B |
| c | 𝗰 | U+1D41C |
| d | 𝗱 | U+1D41D |
| e | 𝗲 | U+1D41E |
| f | 𝗳 | U+1D41F |
| g | 𝗴 | U+1D420 |
| h | 𝗵 | U+1D421 |
| i | 𝗶 | U+1D422 |
| j | 𝗷 | U+1D423 |
| k | 𝗸 | U+1D424 |
| l | 𝗹 | U+1D425 |
| m | 𝗺 | U+1D426 |
| n | 𝗻 | U+1D427 |
| o | 𝗼 | U+1D428 |
| p | 𝗽 | U+1D429 |
| q | 𝗾 | U+1D42A |
| r | 𝗿 | U+1D42B |
| s | 𝘀 | U+1D42C |
| t | 𝘁 | U+1D42D |
| u | 𝘂 | U+1D42E |
| v | 𝘃 | U+1D42F |
| w | 𝘄 | U+1D430 |
| x | 𝘅 | U+1D431 |
| y | 𝘆 | U+1D432 |
| z | 𝘇 | U+1D433 |

## Italic (Mathematical Italic)

| ASCII | Unicode | Code Point |
|-------|---------|------------|
| A | 𝘈 | U+1D434 |
| B | 𝘉 | U+1D435 |
| C | 𝘊 | U+1D436 |
| D | 𝘋 | U+1D437 |
| E | 𝘌 | U+1D438 |
| F | 𝘍 | U+1D439 |
| G | 𝘎 | U+1D43A |
| H | 𝘏 | U+1D43B |
| I | 𝘐 | U+1D43C |
| J | 𝘑 | U+1D43D |
| K | 𝘒 | U+1D43E |
| L | 𝘓 | U+1D43F |
| M | 𝘔 | U+1D440 |
| N | 𝘕 | U+1D441 |
| O | 𝘖 | U+1D442 |
| P | 𝘗 | U+1D443 |
| Q | 𝘘 | U+1D444 |
| R | 𝘙 | U+1D445 |
| S | 𝘚 | U+1D446 |
| T | 𝘛 | U+1D447 |
| U | 𝘜 | U+1D448 |
| V | 𝘝 | U+1D449 |
| W | 𝘞 | U+1D44A |
| X | 𝘟 | U+1D44B |
| Y | 𝘠 | U+1D44C |
| Z | 𝘡 | U+1D44D |
| a | 𝘢 | U+1D44E |
| b | 𝘣 | U+1D44F |
| c | 𝘤 | U+1D450 |
| d | 𝘥 | U+1D451 |
| e | 𝘦 | U+1D452 |
| f | 𝘧 | U+1D453 |
| g | 𝘨 | U+1D454 |
| h | 𝘩 | U+1D455 |
| i | 𝘪 | U+1D456 |
| j | 𝘫 | U+1D457 |
| k | 𝘬 | U+1D458 |
| l | 𝘭 | U+1D459 |
| m | 𝘮 | U+1D45A |
| n | 𝘯 | U+1D45B |
| o | 𝘰 | U+1D45C |
| p | 𝘱 | U+1D45D |
| q | 𝘲 | U+1D45E |
| r | 𝘳 | U+1D45F |
| s | 𝘴 | U+1D460 |
| t | 𝘵 | U+1D461 |
| u | 𝘶 | U+1D462 |
| v | 𝘷 | U+1D463 |
| w | 𝘸 | U+1D464 |
| x | 𝘹 | U+1D465 |
| y | 𝘺 | U+1D466 |
| z | 𝘻 | U+1D467 |

## Formula

```rust
// Bold
fn to_bold_char(c: char) -> char {
    match c {
        'A'..='Z' => char::from_u32(0x1D400 + (c as u32 - 'A' as u32)).unwrap_or(c),
        'a'..='z' => char::from_u32(0x1D41A + (c as u32 - 'a' as u32)).unwrap_or(c),
        _ => c,
    }
}

// Italic
fn to_italic_char(c: char) -> char {
    match c {
        'A'..='Z' => char::from_u32(0x1D434 + (c as u32 - 'A' as u32)).unwrap_or(c),
        'a'..='z' => char::from_u32(0x1D44E + (c as u32 - 'a' as u32)).unwrap_or(c),
        _ => c,
    }
}
```

## Notes

- Numbers (0-9) are NOT transformed.
- Punctuation and spaces are preserved.
- Emoji and other Unicode are passed through unchanged.
- Some fonts may not display these characters correctly.
