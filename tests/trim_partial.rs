#[test]
fn trim_partial_utf8_complete() {
    // Complete UTF-8 string: "Hello" (all ASCII, no partial characters)
    let input = b"Hello";
    assert_eq!(simdutf::trim_partial_utf8(input), input.len());
}

#[test]
fn trim_partial_utf8_empty() {
    assert_eq!(simdutf::trim_partial_utf8(b""), 0);
}

#[test]
fn trim_partial_utf8_truncated_2byte() {
    // U+00E9 (é) is encoded as [0xC3, 0xA9] in UTF-8.
    // Truncate to just the leading byte.
    let input: &[u8] = &[0xC3];
    assert_eq!(simdutf::trim_partial_utf8(input), 0);
}

#[test]
fn trim_partial_utf8_truncated_3byte() {
    // U+4E16 (世) is encoded as [0xE4, 0xB8, 0x96] in UTF-8.
    // Truncate to 2 bytes.
    let input: &[u8] = &[0xE4, 0xB8];
    assert_eq!(simdutf::trim_partial_utf8(input), 0);
}

#[test]
fn trim_partial_utf8_truncated_4byte() {
    // U+1F600 (😀) is encoded as [0xF0, 0x9F, 0x98, 0x80] in UTF-8.
    // Truncate to 3 bytes.
    let input: &[u8] = &[0xF0, 0x9F, 0x98];
    assert_eq!(simdutf::trim_partial_utf8(input), 0);
}

#[test]
fn trim_partial_utf8_valid_then_truncated() {
    // "A" followed by a truncated 2-byte sequence
    let input: &[u8] = &[b'A', 0xC3];
    assert_eq!(simdutf::trim_partial_utf8(input), 1);
}

#[test]
fn trim_partial_utf16_complete() {
    // Complete UTF-16 string: just BMP characters
    let input: &[u16] = &[0x0048, 0x0065, 0x006C, 0x006C, 0x006F]; // "Hello"
    assert_eq!(simdutf::trim_partial_utf16(input), input.len());
}

#[test]
fn trim_partial_utf16_empty() {
    let input: &[u16] = &[];
    assert_eq!(simdutf::trim_partial_utf16(input), 0);
}

#[test]
fn trim_partial_utf16_truncated_surrogate() {
    // A high surrogate without a following low surrogate (truncated)
    // U+1F600 in UTF-16 is [0xD83D, 0xDE00]
    // Native endianness
    let input: &[u16] = &[0x0041, 0xD83D]; // 'A' followed by lone high surrogate
    assert_eq!(simdutf::trim_partial_utf16(input), 1);
}

#[test]
fn trim_partial_utf16be_complete() {
    let input: &[u16] = &[
        0x0048u16.to_be(),
        0x0065u16.to_be(),
        0x006Cu16.to_be(),
        0x006Cu16.to_be(),
        0x006Fu16.to_be(),
    ]; // "Hello"
    assert_eq!(simdutf::trim_partial_utf16be(input), input.len());
}

#[test]
fn trim_partial_utf16be_truncated_surrogate() {
    // 'A' followed by a lone high surrogate for U+1F600 (😀): [0xD83D, 0xDE00]
    let input: &[u16] = &[0x0041u16.to_be(), 0xD83Du16.to_be()];
    assert_eq!(simdutf::trim_partial_utf16be(input), 1);
}

#[test]
fn trim_partial_utf16le_complete() {
    let input: &[u16] = &[
        0x0048u16.to_le(),
        0x0065u16.to_le(),
        0x006Cu16.to_le(),
        0x006Cu16.to_le(),
        0x006Fu16.to_le(),
    ]; // "Hello"
    assert_eq!(simdutf::trim_partial_utf16le(input), input.len());
}

#[test]
fn trim_partial_utf16le_truncated_surrogate() {
    // 'A' followed by a lone high surrogate for U+1F600 (😀): [0xD83D, 0xDE00]
    let input: &[u16] = &[0x0041u16.to_le(), 0xD83Du16.to_le()];
    assert_eq!(simdutf::trim_partial_utf16le(input), 1);
}
