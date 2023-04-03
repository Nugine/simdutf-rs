//! Hand-written bindings and wrappers.

extern "C" {
    fn simdutf_change_endianness_utf16(src: *const u16, len: usize, dst: *mut u16);
    fn simdutf_autodetect_encoding(src: *const u8, len: usize) -> u32;
    fn simdutf_detect_encodings(src: *const u8, len: usize) -> u32;
}

/// Change the endianness of UTF-16 string.
///
/// This function does not validate the input.
///
/// This function is not BOM-aware.
///
/// # Safety
/// + `src` and `dst` must be non-null and properly aligned.
/// + `src` must be valid for reads of `len * size_of::<u16>()` bytes.
/// + `dst` must be valid for writes of `len * size_of::<u16>()` bytes.
/// + `src` and `dst` can alias.
#[inline]
pub unsafe fn change_endianness_utf16(src: *const u16, len: usize, dst: *mut u16) {
    simdutf_change_endianness_utf16(src, len, dst);
}

bitflags::bitflags! {
    /// The encoding of a string, defined as a bitflags type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Encoding: u32 {
        /// Unspecified encoding.
        const UNSPECIFIED = 0;

        /// UTF-8 encoding.
        const UTF8 = 1;

        /// UTF-16LE encoding.
        const UTF16_LE = 2;

        /// UTF-16BE encoding.
        const UTF16_BE = 4;

        /// UTF-32LE encoding.
        const UTF32_LE = 8;

        /// UTF-32BE encoding.
        const UTF32_BE = 16;
    }
}

/// Autodetect the encoding of the input.
///
/// This function returns a single encoding.
///
#[inline]
#[must_use]
pub fn autodetect_single_encoding(src: &[u8]) -> Encoding {
    unsafe {
        let ans = simdutf_autodetect_encoding(src.as_ptr(), src.len());
        Encoding::from_bits_retain(ans)
    }
}

/// Autodetect the possible encodings of the input in one pass.
///
/// This function returns a bitset of possible encodings.
#[inline]
#[must_use]
pub fn autodetect_encodings(src: &[u8]) -> Encoding {
    unsafe {
        let ans = simdutf_detect_encodings(src.as_ptr(), src.len());
        Encoding::from_bits_retain(ans)
    }
}
