// Auto generated by codegen/src/generated.rs

/// Validate the ASCII string.
///
/// Returns [`true`] if and only if the string is valid ASCII.
#[inline]
#[must_use]
pub fn validate_ascii(src: &[u8]) -> bool {
    let len = src.len();
    let buf = src.as_ptr();
    unsafe { crate::bindings::simdutf_validate_ascii(buf, len) }
}

/// Validate the UTF-8 string.
///
/// Returns [`true`] if and only if the string is valid UTF-8.
#[inline]
#[must_use]
pub fn validate_utf8(src: &[u8]) -> bool {
    let len = src.len();
    let buf = src.as_ptr();
    unsafe { crate::bindings::simdutf_validate_utf8(buf, len) }
}

/// Validate the UTF-16 string.
///
/// This function uses native endianness.
///
/// This function is not BOM-aware.
///
/// Returns [`true`] if and only if the string is valid UTF-16.
#[inline]
#[must_use]
pub fn validate_utf16(src: &[u16]) -> bool {
    let len = src.len();
    let buf = src.as_ptr();
    unsafe { crate::bindings::simdutf_validate_utf16(buf, len) }
}

/// Validate the UTF-16BE string.
///
/// This function is not BOM-aware.
///
/// Returns [`true`] if and only if the string is valid UTF-16BE.
#[inline]
#[must_use]
pub fn validate_utf16be(src: &[u16]) -> bool {
    let len = src.len();
    let buf = src.as_ptr();
    unsafe { crate::bindings::simdutf_validate_utf16be(buf, len) }
}

/// Validate the UTF-16LE string.
///
/// This function is not BOM-aware.
///
/// Returns [`true`] if and only if the string is valid UTF-16LE.
#[inline]
#[must_use]
pub fn validate_utf16le(src: &[u16]) -> bool {
    let len = src.len();
    let buf = src.as_ptr();
    unsafe { crate::bindings::simdutf_validate_utf16le(buf, len) }
}

/// Validate the UTF-32 string.
///
/// This function uses native endianness.
///
/// Returns [`true`] if and only if the string is valid UTF-32.
#[inline]
#[must_use]
pub fn validate_utf32(src: &[u32]) -> bool {
    let len = src.len();
    let buf = src.as_ptr();
    unsafe { crate::bindings::simdutf_validate_utf32(buf, len) }
}

/// Count the number of code points in the UTF-8 string.
///
/// # Safety
/// + The input string must be valid UTF-8.
#[inline]
#[must_use]
pub unsafe fn count_utf8(src: &[u8]) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_count_utf8(buf, len)
}

/// Count the number of code points in the UTF-16 string.
///
/// This function uses native endianness.
///
/// This function is not BOM-aware.
///
/// # Safety
/// + The input string must be valid UTF-16.
#[inline]
#[must_use]
pub unsafe fn count_utf16(src: &[u16]) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_count_utf16(buf, len)
}

/// Count the number of code points in the UTF-16BE string.
///
/// This function is not BOM-aware.
///
/// # Safety
/// + The input string must be valid UTF-16BE.
#[inline]
#[must_use]
pub unsafe fn count_utf16be(src: &[u16]) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_count_utf16be(buf, len)
}

/// Count the number of code points in the UTF-16LE string.
///
/// This function is not BOM-aware.
///
/// # Safety
/// + The input string must be valid UTF-16LE.
#[inline]
#[must_use]
pub unsafe fn count_utf16le(src: &[u16]) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_count_utf16le(buf, len)
}

/// Count the number of code units that the UTF-16 string would require in UTF-8 format.
///
/// This function uses native endianness.
///
/// This function is not BOM-aware.
///
/// # Safety
/// + The input string must be valid UTF-16.
#[inline]
#[must_use]
pub unsafe fn utf8_length_from_utf16(src: &[u16]) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_utf8_length_from_utf16(buf, len)
}

/// Count the number of code units that the UTF-16BE string would require in UTF-8 format.
///
/// This function is not BOM-aware.
///
/// # Safety
/// + The input string must be valid UTF-16BE.
#[inline]
#[must_use]
pub unsafe fn utf8_length_from_utf16be(src: &[u16]) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_utf8_length_from_utf16be(buf, len)
}

/// Count the number of code units that the UTF-16LE string would require in UTF-8 format.
///
/// This function is not BOM-aware.
///
/// # Safety
/// + The input string must be valid UTF-16LE.
#[inline]
#[must_use]
pub unsafe fn utf8_length_from_utf16le(src: &[u16]) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_utf8_length_from_utf16le(buf, len)
}

/// Count the number of code units that the UTF-32 string would require in UTF-8 format.
///
/// This function uses native endianness.
///
/// # Safety
/// + The input string must be valid UTF-32.
#[inline]
#[must_use]
pub unsafe fn utf8_length_from_utf32(src: &[u32]) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_utf8_length_from_utf32(buf, len)
}

/// Count the number of code units that the UTF-8 string would require in UTF-16 format.
///
/// # Safety
/// + The input string must be valid UTF-8.
#[inline]
#[must_use]
pub unsafe fn utf16_length_from_utf8(src: &[u8]) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_utf16_length_from_utf8(buf, len)
}

/// Count the number of code units that the UTF-32 string would require in UTF-16 format.
///
/// This function uses native endianness.
///
/// # Safety
/// + The input string must be valid UTF-32.
#[inline]
#[must_use]
pub unsafe fn utf16_length_from_utf32(src: &[u32]) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_utf16_length_from_utf32(buf, len)
}

/// Count the number of code units that the UTF-8 string would require in UTF-32 format.
///
/// This function is equivalent to [`count_utf8`].
///
/// # Safety
/// + The input string must be valid UTF-8.
#[inline]
#[must_use]
pub unsafe fn utf32_length_from_utf8(src: &[u8]) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_utf32_length_from_utf8(buf, len)
}

/// Count the number of code units that the UTF-16 string would require in UTF-32 format.
///
/// This function is equivalent to [`count_utf16`].
///
/// This function uses native endianness.
///
/// This function is not BOM-aware.
///
/// # Safety
/// + The input string must be valid UTF-16.
#[inline]
#[must_use]
pub unsafe fn utf32_length_from_utf16(src: &[u16]) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_utf32_length_from_utf16(buf, len)
}

/// Count the number of code units that the UTF-16BE string would require in UTF-32 format.
///
/// This function is equivalent to [`count_utf16be`].
///
/// This function is not BOM-aware.
///
/// # Safety
/// + The input string must be valid UTF-16BE.
#[inline]
#[must_use]
pub unsafe fn utf32_length_from_utf16be(src: &[u16]) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_utf32_length_from_utf16be(buf, len)
}

/// Count the number of code units that the UTF-16LE string would require in UTF-32 format.
///
/// This function is equivalent to [`count_utf16le`].
///
/// This function is not BOM-aware.
///
/// # Safety
/// + The input string must be valid UTF-16LE.
#[inline]
#[must_use]
pub unsafe fn utf32_length_from_utf16le(src: &[u16]) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_utf32_length_from_utf16le(buf, len)
}

/// Convert possibly broken UTF-8 string into UTF-16 string.
///
/// During the conversion also validation of the input string is done.
/// This function is suitable to work with inputs from untrusted sources.
///
/// Returns the number of written code units; 0 if the input is not a valid UTF-8 string
///
/// # Safety
/// + `dst` must be non-null and properly aligned.
/// + `dst` must be valid for writes of `count * size_of::<u16>()` bytes, where the `count` is the number of code units ([`u16`]) after successful conversion.
#[inline]
#[must_use]
pub unsafe fn convert_utf8_to_utf16(src: &[u8], dst: *mut u16) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_convert_utf8_to_utf16(buf, len, dst)
}

/// Convert possibly broken UTF-8 string into UTF-16BE string.
///
/// During the conversion also validation of the input string is done.
/// This function is suitable to work with inputs from untrusted sources.
///
/// Returns the number of written code units; 0 if the input is not a valid UTF-8 string
///
/// # Safety
/// + `dst` must be non-null and properly aligned.
/// + `dst` must be valid for writes of `count * size_of::<u16>()` bytes, where the `count` is the number of code units ([`u16`]) after successful conversion.
#[inline]
#[must_use]
pub unsafe fn convert_utf8_to_utf16be(src: &[u8], dst: *mut u16) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_convert_utf8_to_utf16be(buf, len, dst)
}

/// Convert possibly broken UTF-8 string into UTF-16LE string.
///
/// During the conversion also validation of the input string is done.
/// This function is suitable to work with inputs from untrusted sources.
///
/// Returns the number of written code units; 0 if the input is not a valid UTF-8 string
///
/// # Safety
/// + `dst` must be non-null and properly aligned.
/// + `dst` must be valid for writes of `count * size_of::<u16>()` bytes, where the `count` is the number of code units ([`u16`]) after successful conversion.
#[inline]
#[must_use]
pub unsafe fn convert_utf8_to_utf16le(src: &[u8], dst: *mut u16) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_convert_utf8_to_utf16le(buf, len, dst)
}

/// Convert possibly broken UTF-8 string into UTF-32 string.
///
/// During the conversion also validation of the input string is done.
/// This function is suitable to work with inputs from untrusted sources.
///
/// Returns the number of written code units; 0 if the input is not a valid UTF-8 string
///
/// # Safety
/// + `dst` must be non-null and properly aligned.
/// + `dst` must be valid for writes of `count * size_of::<u32>()` bytes, where the `count` is the number of code units ([`u32`]) after successful conversion.
#[inline]
#[must_use]
pub unsafe fn convert_utf8_to_utf32(src: &[u8], dst: *mut u32) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_convert_utf8_to_utf32(buf, len, dst)
}

/// Convert possibly broken UTF-16 string into UTF-8 string.
///
/// During the conversion also validation of the input string is done.
/// This function is suitable to work with inputs from untrusted sources.
///
/// Returns the number of written code units; 0 if the input is not a valid UTF-16 string
///
/// This function uses native endianness.
///
/// This function is not BOM-aware.
///
/// # Safety
/// + `dst` must be non-null and properly aligned.
/// + `dst` must be valid for writes of `count * size_of::<u8>()` bytes, where the `count` is the number of code units ([`u8`]) after successful conversion.
#[inline]
#[must_use]
pub unsafe fn convert_utf16_to_utf8(src: &[u16], dst: *mut u8) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_convert_utf16_to_utf8(buf, len, dst)
}

/// Convert possibly broken UTF-16 string into UTF-32 string.
///
/// During the conversion also validation of the input string is done.
/// This function is suitable to work with inputs from untrusted sources.
///
/// Returns the number of written code units; 0 if the input is not a valid UTF-16 string
///
/// This function uses native endianness.
///
/// This function is not BOM-aware.
///
/// # Safety
/// + `dst` must be non-null and properly aligned.
/// + `dst` must be valid for writes of `count * size_of::<u32>()` bytes, where the `count` is the number of code units ([`u32`]) after successful conversion.
#[inline]
#[must_use]
pub unsafe fn convert_utf16_to_utf32(src: &[u16], dst: *mut u32) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_convert_utf16_to_utf32(buf, len, dst)
}

/// Convert possibly broken UTF-16BE string into UTF-8 string.
///
/// During the conversion also validation of the input string is done.
/// This function is suitable to work with inputs from untrusted sources.
///
/// Returns the number of written code units; 0 if the input is not a valid UTF-16BE string
///
/// This function is not BOM-aware.
///
/// # Safety
/// + `dst` must be non-null and properly aligned.
/// + `dst` must be valid for writes of `count * size_of::<u8>()` bytes, where the `count` is the number of code units ([`u8`]) after successful conversion.
#[inline]
#[must_use]
pub unsafe fn convert_utf16be_to_utf8(src: &[u16], dst: *mut u8) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_convert_utf16be_to_utf8(buf, len, dst)
}

/// Convert possibly broken UTF-16BE string into UTF-32 string.
///
/// During the conversion also validation of the input string is done.
/// This function is suitable to work with inputs from untrusted sources.
///
/// Returns the number of written code units; 0 if the input is not a valid UTF-16BE string
///
/// This function is not BOM-aware.
///
/// # Safety
/// + `dst` must be non-null and properly aligned.
/// + `dst` must be valid for writes of `count * size_of::<u32>()` bytes, where the `count` is the number of code units ([`u32`]) after successful conversion.
#[inline]
#[must_use]
pub unsafe fn convert_utf16be_to_utf32(src: &[u16], dst: *mut u32) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_convert_utf16be_to_utf32(buf, len, dst)
}

/// Convert possibly broken UTF-16LE string into UTF-8 string.
///
/// During the conversion also validation of the input string is done.
/// This function is suitable to work with inputs from untrusted sources.
///
/// Returns the number of written code units; 0 if the input is not a valid UTF-16LE string
///
/// This function is not BOM-aware.
///
/// # Safety
/// + `dst` must be non-null and properly aligned.
/// + `dst` must be valid for writes of `count * size_of::<u8>()` bytes, where the `count` is the number of code units ([`u8`]) after successful conversion.
#[inline]
#[must_use]
pub unsafe fn convert_utf16le_to_utf8(src: &[u16], dst: *mut u8) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_convert_utf16le_to_utf8(buf, len, dst)
}

/// Convert possibly broken UTF-16LE string into UTF-32 string.
///
/// During the conversion also validation of the input string is done.
/// This function is suitable to work with inputs from untrusted sources.
///
/// Returns the number of written code units; 0 if the input is not a valid UTF-16LE string
///
/// This function is not BOM-aware.
///
/// # Safety
/// + `dst` must be non-null and properly aligned.
/// + `dst` must be valid for writes of `count * size_of::<u32>()` bytes, where the `count` is the number of code units ([`u32`]) after successful conversion.
#[inline]
#[must_use]
pub unsafe fn convert_utf16le_to_utf32(src: &[u16], dst: *mut u32) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_convert_utf16le_to_utf32(buf, len, dst)
}

/// Convert possibly broken UTF-32 string into UTF-8 string.
///
/// During the conversion also validation of the input string is done.
/// This function is suitable to work with inputs from untrusted sources.
///
/// Returns the number of written code units; 0 if the input is not a valid UTF-32 string
///
/// This function uses native endianness.
///
/// # Safety
/// + `dst` must be non-null and properly aligned.
/// + `dst` must be valid for writes of `count * size_of::<u8>()` bytes, where the `count` is the number of code units ([`u8`]) after successful conversion.
#[inline]
#[must_use]
pub unsafe fn convert_utf32_to_utf8(src: &[u32], dst: *mut u8) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_convert_utf32_to_utf8(buf, len, dst)
}

/// Convert possibly broken UTF-32 string into UTF-16 string.
///
/// During the conversion also validation of the input string is done.
/// This function is suitable to work with inputs from untrusted sources.
///
/// Returns the number of written code units; 0 if the input is not a valid UTF-32 string
///
/// This function uses native endianness.
///
/// # Safety
/// + `dst` must be non-null and properly aligned.
/// + `dst` must be valid for writes of `count * size_of::<u16>()` bytes, where the `count` is the number of code units ([`u16`]) after successful conversion.
#[inline]
#[must_use]
pub unsafe fn convert_utf32_to_utf16(src: &[u32], dst: *mut u16) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_convert_utf32_to_utf16(buf, len, dst)
}

/// Convert possibly broken UTF-32 string into UTF-16BE string.
///
/// During the conversion also validation of the input string is done.
/// This function is suitable to work with inputs from untrusted sources.
///
/// Returns the number of written code units; 0 if the input is not a valid UTF-32 string
///
/// This function uses native endianness.
///
/// # Safety
/// + `dst` must be non-null and properly aligned.
/// + `dst` must be valid for writes of `count * size_of::<u16>()` bytes, where the `count` is the number of code units ([`u16`]) after successful conversion.
#[inline]
#[must_use]
pub unsafe fn convert_utf32_to_utf16be(src: &[u32], dst: *mut u16) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_convert_utf32_to_utf16be(buf, len, dst)
}

/// Convert possibly broken UTF-32 string into UTF-16LE string.
///
/// During the conversion also validation of the input string is done.
/// This function is suitable to work with inputs from untrusted sources.
///
/// Returns the number of written code units; 0 if the input is not a valid UTF-32 string
///
/// This function uses native endianness.
///
/// # Safety
/// + `dst` must be non-null and properly aligned.
/// + `dst` must be valid for writes of `count * size_of::<u16>()` bytes, where the `count` is the number of code units ([`u16`]) after successful conversion.
#[inline]
#[must_use]
pub unsafe fn convert_utf32_to_utf16le(src: &[u32], dst: *mut u16) -> usize {
    let len = src.len();
    let buf = src.as_ptr();
    crate::bindings::simdutf_convert_utf32_to_utf16le(buf, len, dst)
}
