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

/// The result type of validation and transcoding.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Result {
    /// The error code of validation/transcoding.
    pub error: ErrorCode,
    /// In case of success, indicates the number of code units validated/written.
    ///
    /// In case of error, indicates the position of the error in the input.
    pub count: usize,
}

/// The error code type of validation and transcoding.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    /// Success
    Success = 0,

    /// Any byte must have fewer than 5 header bits.
    HeaderBits = 1,

    /// The leading byte must be followed by N-1 continuation bytes, where N is the UTF-8 character length.
    /// This is also the error when the input is truncated.
    TooShort = 2,

    /// We either have too many consecutive continuation bytes or the string starts with a continuation byte.
    TooLong = 3,

    /// The decoded character must be above U+7F for two-byte characters, U+7FF for three-byte characters,
    /// and U+FFFF for four-byte characters.
    OverLong = 4,

    /// The decoded character must be less than or equal to U+10FFFF OR less than or equal than U+7F for ASCII.
    TooLarge = 5,

    /// The decoded character must be not be in U+D800...DFFF (UTF-8 or UTF-32)
    /// OR
    /// a high surrogate must be followed by a low surrogate
    /// and a low surrogate must be preceded by a high surrogate (UTF-16)
    /// OR
    /// there must be no surrogate at all and one is found (Latin1 functions)
    /// OR
    /// *specifically* for the function utf8_length_from_utf16_with_replacement,
    /// a surrogate (whether in error or not) has been found
    /// (I.e., whether we are in the Basic Multilingual Plane or not).
    Surrogate = 6,

    /// Found a character that cannot be part of a valid base64 string.
    /// This may include a misplaced padding character ('=').
    InvalidBase64Character = 7,

    /// The base64 input terminates with a single character, excluding padding (=).
    /// It is also used in strict mode when padding is not adequate.
    Base64InputRemainder = 8,

    /// The base64 input terminates with non-zero padding bits.
    Base64ExtraBits = 9,

    /// The output buffer is too small.
    OutputBufferTooSmall = 10,

    /// Not related to validation/transcoding.
    Other = 11,
}

/// The error code type of validation and transcoding.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Base64Options {
    /// Standard base64 format (with padding).
    Default = 0,
    /// Base64url format (no padding).
    Url = 1,
    /// Standard base64 format (no padding).
    DefaultNoPadding = 2,
    /// Base64url format (with padding).
    UrlWithPadding = 3,
    /// Standard base64 format accepting garbage characters
    DefaultAcceptGarbage = 4,
    /// Base64url format accepting garbage characters
    UrlAcceptGarbage = 5,
    /// Standard/base64url hybrid format (only meaningful for decoding!)
    DefaultOrUrl = 8,
    /// Standard/base64url hybrid format accepting garbage characters (only meaningful for decoding!)
    DefaultOrUrlAcceptGarbage = 12,
}

/// The last chunk handling options for base64 decoding.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LastChunkHandlingOptions {
    /// Standard base64 format, decode partial final chunk.
    Loose = 0,
    /// Error when the last chunk is partial, 2 or 3 chars, and unpadded, or non-zero bit padding.
    Strict = 1,
    /// If the last chunk is partial, ignore it (no error).
    StopBeforePartial = 2,
    /// Only decode full blocks (4 base64 characters, no padding).
    OnlyFullChunks = 3,
}
