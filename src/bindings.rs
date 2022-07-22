extern "C" {
    pub fn simdutf_validate_ascii(src: *const u8, len: usize) -> bool;

    pub fn simdutf_validate_utf8(src: *const u8, len: usize) -> bool;
    pub fn simdutf_validate_utf16(src: *const u16, len: usize) -> bool;
    pub fn simdutf_validate_utf32(src: *const u32, len: usize) -> bool;

    pub fn simdutf_convert_utf8_to_utf16(src: *const u8, len: usize, dst: *mut u16) -> usize;
    pub fn simdutf_convert_utf8_to_utf32(src: *const u8, len: usize, dst: *mut u32) -> usize;
    pub fn simdutf_convert_utf16_to_utf8(src: *const u16, len: usize, dst: *mut u8) -> usize;
    pub fn simdutf_convert_utf16_to_utf32(src: *const u16, len: usize, dst: *mut u32) -> usize;
    pub fn simdutf_convert_utf32_to_utf8(src: *const u32, len: usize, dst: *mut u8) -> usize;
    pub fn simdutf_convert_utf32_to_utf16(src: *const u32, len: usize, dst: *mut u16) -> usize;

    pub fn simdutf_convert_valid_utf8_to_utf16(src: *const u8, len: usize, dst: *mut u16) -> usize;
    pub fn simdutf_convert_valid_utf8_to_utf32(src: *const u8, len: usize, dst: *mut u32) -> usize;
    pub fn simdutf_convert_valid_utf16_to_utf8(src: *const u16, len: usize, dst: *mut u8) -> usize;
    pub fn simdutf_convert_valid_utf16_to_utf32(src: *const u16, len: usize, dst: *mut u32) -> usize;
    pub fn simdutf_convert_valid_utf32_to_utf8(src: *const u32, len: usize, dst: *mut u8) -> usize;
    pub fn simdutf_convert_valid_utf32_to_utf16(src: *const u32, len: usize, dst: *mut u16) -> usize;

    pub fn simdutf_utf8_length_from_utf16(src: *const u16, len: usize) -> usize;
    pub fn simdutf_utf16_length_from_utf8(src: *const u8, len: usize) -> usize;
    pub fn simdutf_utf32_length_from_utf8(src: *const u8, len: usize) -> usize;
    pub fn simdutf_utf8_length_from_utf32(src: *const u32, len: usize) -> usize;
    pub fn simdutf_utf16_length_from_utf32(src: *const u32, len: usize) -> usize;
    pub fn simdutf_utf32_length_from_utf16(src: *const u16, len: usize) -> usize;
}
