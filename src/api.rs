use crate::bindings::*;

/// ASCII encoding
#[derive(Debug, Clone, Copy)]
pub struct ASCII;

/// UTF8 encoding
#[derive(Debug, Clone, Copy)]
pub struct UTF8;

/// Utf16LE encoding
#[derive(Debug, Clone, Copy)]
pub struct UTF16LE;

/// UTF32LE encoding
#[derive(Debug, Clone, Copy)]
pub struct UTF32LE;

/// Transcoding adapter
#[derive(Debug, Clone, Copy)]
pub struct Transcoder<S, D>(S, D);

macro_rules! impl_encoding {
    ($src_encoding:ident, $src_ty: ident, $f: ident) => {
        impl $src_encoding {
            /// Validate the
            #[doc=stringify!($src_encoding)]
            /// String.
            ///
            /// Returns true if and only if the `data` is valid
            #[doc=stringify!($src_encoding)]
            /// .
            #[inline]
            #[must_use]
            pub fn validate(data: &[$src_ty]) -> bool {
                let len = data.len();
                let src = data.as_ptr();
                unsafe { $f(src, len) }
            }
        }
    };
}

impl_encoding!(ASCII, u8, simdutf_validate_ascii);
impl_encoding!(UTF8, u8, simdutf_validate_utf8);
impl_encoding!(UTF16LE, u16, simdutf_validate_utf16);
impl_encoding!(UTF32LE, u32, simdutf_validate_utf32);

impl<S, D> Transcoder<S, D> {
    /// Returns a transcoder from `S` to `D`
    #[inline]
    pub const fn new(src: S, dst: D) -> Self {
        debug_assert!(core::mem::size_of::<S>() == 0);
        debug_assert!(core::mem::size_of::<D>() == 0);
        Self(src, dst)
    }
}

macro_rules! impl_transcoding {
    ($src_encoding:ident, $src_ty: ident, $dst_encoding:ident, $dst_ty: ident, $cc_f:ident, $ca_f:ident, $cv_f:ident) => {
        impl Transcoder<$src_encoding, $dst_encoding> {
            /// Computes the number of
            #[doc=stringify!($src_ty)]
            ///  words that this
            #[doc=stringify!($src_encoding)]
            ///  string would require in
            #[doc=stringify!($dst_encoding)]
            ///  format.
            ///
            /// Returns the number of
            #[doc=stringify!($dst_ty)]
            ///  words required to encode the
            #[doc=stringify!($src_encoding)]
            ///  string as
            #[doc=stringify!($dst_encoding)]
            ///
            /// This function is not BOM-aware.
            ///
            /// # Safety
            /// This function assumes that the input string is valid
            #[doc=stringify!($src_encoding)]
            /// .
            #[inline]
            #[must_use]
            pub unsafe fn converted_count(&self, src: &[$src_ty]) -> usize {
                let len = src.len();
                let src = src.as_ptr();
                $cc_f(src, len)
            }

            /// Converts possibly broken
            #[doc=stringify!($src_encoding)]
            ///  string into
            #[doc=stringify!($dst_encoding)]
            ///  string.
            ///
            /// Returns the number of written
            #[doc=stringify!($dst_ty)]
            ///  words; 0 if the input was not valid
            #[doc=stringify!($src_encoding)]
            ///  string.
            ///
            /// During the conversion also validation of the input string is done.
            /// This function is suitable to work with inputs from untrusted sources.
            ///
            /// # Safety
            /// + `dst` must be valid for writes of `count * size_of::<
            #[doc=stringify!($dst_ty)]
            /// >()` bytes,
            /// where the `count` is the number of
            #[doc=stringify!($dst_ty)]
            ///  words after successful conversion.
            /// + `dst` must be non-null and properly aligned.
            #[inline]
            #[must_use]
            pub unsafe fn convert_arbitrary(&self, src: &[$src_ty], dst: *mut $dst_ty) -> usize {
                let len = src.len();
                let src = src.as_ptr();
                $ca_f(src, len, dst)
            }

            /// Converts valid
            #[doc=stringify!($src_encoding)]
            ///  string into
            #[doc=stringify!($dst_encoding)]
            ///  string.
            ///
            /// Returns the number of written
            #[doc=stringify!($dst_ty)]
            ///  words.
            ///
            /// # Safety
            /// + `dst` must be valid for writes of `count * size_of::<
            #[doc=stringify!($dst_ty)]
            /// >()` bytes,
            /// where the `count` is the number of
            #[doc=stringify!($dst_ty)]
            ///  words after successful conversion.
            /// + `dst` must be non-null and properly aligned.
            #[inline]
            #[must_use]
            pub unsafe fn convert_valid(&self, src: &[$src_ty], dst: *mut $dst_ty) -> usize {
                let len = src.len();
                let src = src.as_ptr();
                $cv_f(src, len, dst)
            }
        }
    };
}

impl_transcoding!(
    UTF8,
    u8,
    UTF16LE,
    u16,
    simdutf_utf16_length_from_utf8,
    simdutf_convert_utf8_to_utf16,
    simdutf_convert_valid_utf8_to_utf16
);

impl_transcoding!(
    UTF8,
    u8,
    UTF32LE,
    u32,
    simdutf_utf32_length_from_utf8,
    simdutf_convert_utf8_to_utf32,
    simdutf_convert_valid_utf8_to_utf32
);

impl_transcoding!(
    UTF16LE,
    u16,
    UTF8,
    u8,
    simdutf_utf8_length_from_utf16,
    simdutf_convert_utf16_to_utf8,
    simdutf_convert_valid_utf16_to_utf8
);

impl_transcoding!(
    UTF16LE,
    u16,
    UTF32LE,
    u32,
    simdutf_utf32_length_from_utf16,
    simdutf_convert_utf16_to_utf32,
    simdutf_convert_valid_utf16_to_utf32
);

impl_transcoding!(
    UTF32LE,
    u32,
    UTF8,
    u8,
    simdutf_utf8_length_from_utf32,
    simdutf_convert_utf32_to_utf8,
    simdutf_convert_valid_utf32_to_utf8
);

impl_transcoding!(
    UTF32LE,
    u32,
    UTF16LE,
    u16,
    simdutf_utf16_length_from_utf32,
    simdutf_convert_utf32_to_utf16,
    simdutf_convert_valid_utf32_to_utf16
);
