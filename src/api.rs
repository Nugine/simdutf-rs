use crate::bindings::*;

macro_rules! impl_validate {
    ($src_encoding:literal, $src_ty: ident, $f: ident, $name: ident) => {
        /// Validates the
        #[doc=$src_encoding]
        /// String.
        ///
        /// Returns true if and only if the data is valid
        #[doc=$src_encoding]
        /// .
        #[inline]
        #[must_use]
        pub fn $name(data: &[$src_ty]) -> bool {
            let len = data.len();
            let src = data.as_ptr();
            unsafe { $f(src, len) }
        }
    };
}

impl_validate!("ASCII", u8, simdutf_validate_ascii, validate_ascii);
impl_validate!("UTF8", u8, simdutf_validate_utf8, validate_utf8);
impl_validate!("UTF16-LE", u16, simdutf_validate_utf16le, validate_utf16le);
impl_validate!("UTF16-BE", u16, simdutf_validate_utf16be, validate_utf16be);
impl_validate!("UTF32-LE", u32, simdutf_validate_utf32, validate_utf32le);

macro_rules! impl_transcoding {
    (
        $src_encoding:literal, $src_ty: ident,  //
        $dst_encoding:literal, $dst_ty: ident,  //
        $ca_f:ident, $ca_name: ident,  //
        $cv_f:ident, $cv_name:ident, //
    ) => {
        /// Converts possibly broken
        #[doc=$src_encoding]
        ///  string into
        #[doc=$dst_encoding]
        ///  string.
        ///
        /// Returns the number of written
        #[doc=stringify!($dst_ty)]
        ///  words; 0 if the input was not valid
        #[doc=$src_encoding]
        ///  string.
        ///
        /// During the conversion also validation of the input string is done.
        /// This function is suitable to work with inputs from untrusted sources.
        ///
        /// # Safety
        /// + `src` and `dst` must be non-null and properly aligned.
        /// + `src` must be valid for reads of `len * size_of::<
        #[doc=stringify!($src_ty)]
        /// >()` bytes.
        /// + `dst` must be valid for writes of `count * size_of::<
        #[doc=stringify!($dst_ty)]
        /// >()` bytes,
        /// where the `count` is the number of
        #[doc=stringify!($dst_ty)]
        ///  words after successful conversion.
        #[inline]
        #[must_use]
        pub unsafe fn $ca_name(src: *const $src_ty, len: usize, dst: *mut $dst_ty) -> usize {
            $ca_f(src, len, dst)
        }

        /// Converts valid
        #[doc=$src_encoding]
        ///  string into
        #[doc=$dst_encoding]
        ///  string.
        ///
        /// Returns the number of written
        #[doc=stringify!($dst_ty)]
        ///  words.
        ///
        /// # Safety
        /// + `src` and `dst` must be non-null and properly aligned.
        /// + `src` must be valid for reads of `len * size_of::<
        #[doc=stringify!($src_ty)]
        /// >()` bytes.
        /// + `dst` must be valid for writes of `count * size_of::<
        #[doc=stringify!($dst_ty)]
        /// >()` bytes,
        /// where the `count` is the number of
        #[doc=stringify!($dst_ty)]
        ///  words after successful conversion.
        #[inline]
        #[must_use]
        pub unsafe fn $cv_name(src: *const $src_ty, len: usize, dst: *mut $dst_ty) -> usize {
            $cv_f(src, len, dst)
        }
    };
}

macro_rules! impl_count {
    (
        $src_encoding:literal, $src_ty: ident,  //
        $dst_encoding:literal, $dst_ty: ident,  //
        $cc_f:ident, $cc_name: ident,  //
    ) => {
        /// Computes the number of
        #[doc=stringify!($dst_ty)]
        /// code units
        /// after transcoding from
        #[doc=$src_encoding]
        /// to
        #[doc=$dst_encoding]
        /// .
        ///
        /// This function is not BOM-aware.
        ///
        /// # Safety
        /// This function assumes that the input string is valid
        #[doc=$src_encoding]
        /// .
        #[inline]
        #[must_use]
        pub unsafe fn $cc_name(src: &[$src_ty]) -> usize {
            let len = src.len();
            let src = src.as_ptr();
            $cc_f(src, len)
        }
    };
}

impl_transcoding!(
    "UTF8",
    u8,
    "UTF16-LE",
    u16,
    simdutf_convert_utf8_to_utf16le,
    convert_arbitrary_utf8_to_utf16le,
    simdutf_convert_valid_utf8_to_utf16le,
    convert_valid_utf8_to_utf16le,
);

impl_transcoding!(
    "UTF8",
    u8,
    "UTF16-BE",
    u16,
    simdutf_convert_utf8_to_utf16be,
    convert_arbitrary_utf8_to_utf16be,
    simdutf_convert_valid_utf8_to_utf16be,
    convert_valid_utf8_to_utf16be,
);

impl_transcoding!(
    "UTF8",
    u8,
    "UTF32-LE",
    u32,
    simdutf_convert_utf8_to_utf32,
    convert_arbitrary_utf8_to_utf32le,
    simdutf_convert_valid_utf8_to_utf32,
    convert_valid_utf8_to_utf32le,
);

impl_transcoding!(
    "UTF16-LE",
    u16,
    "UTF8",
    u8,
    simdutf_convert_utf16le_to_utf8,
    convert_arbitrary_utf16le_to_utf8,
    simdutf_convert_valid_utf16le_to_utf8,
    convert_valid_utf16le_to_utf8,
);

impl_transcoding!(
    "UTF16-LE",
    u16,
    "UTF32-LE",
    u32,
    simdutf_convert_utf16le_to_utf32,
    convert_arbitrary_utf16le_to_utf32le,
    simdutf_convert_valid_utf16le_to_utf32,
    convert_valid_utf16le_to_utf32le,
);

impl_transcoding!(
    "UTF16-BE",
    u16,
    "UTF8",
    u8,
    simdutf_convert_utf16be_to_utf8,
    convert_arbitrary_utf16be_to_utf8,
    simdutf_convert_valid_utf16be_to_utf8,
    convert_valid_utf16be_to_utf8,
);

impl_transcoding!(
    "UTF16-BE",
    u16,
    "UTF32-LE",
    u32,
    simdutf_convert_utf16be_to_utf32,
    convert_arbitrary_utf16be_to_utf32le,
    simdutf_convert_valid_utf16be_to_utf32,
    convert_valid_utf16be_to_utf32le,
);

impl_transcoding!(
    "UTF32-LE",
    u32,
    "UTF8",
    u8,
    simdutf_convert_utf32_to_utf8,
    convert_arbitrary_utf32le_to_utf8,
    simdutf_convert_valid_utf32_to_utf8,
    convert_valid_utf32le_to_utf8,
);

impl_transcoding!(
    "UTF32-LE",
    u32,
    "UTF16-LE",
    u16,
    simdutf_convert_utf32_to_utf16le,
    convert_arbitrary_utf32le_to_utf16le,
    simdutf_convert_valid_utf32_to_utf16le,
    convert_valid_utf32le_to_utf16le,
);

impl_transcoding!(
    "UTF32-LE",
    u32,
    "UTF16-BE",
    u16,
    simdutf_convert_utf32_to_utf16be,
    convert_arbitrary_utf32le_to_utf16be,
    simdutf_convert_valid_utf32_to_utf16be,
    convert_valid_utf32le_to_utf16be,
);

impl_count!(
    "UTF8",
    u8,
    "UTF16",
    u16,
    simdutf_utf16_length_from_utf8,
    count_utf16_from_utf8,
);

impl_count!(
    "UTF8",
    u8,
    "UTF32",
    u32,
    simdutf_utf32_length_from_utf8,
    count_utf32_from_utf8,
);

impl_count!(
    "UTF16-LE",
    u16,
    "UTF8",
    u8,
    simdutf_utf8_length_from_utf16le,
    count_utf8_from_utf16le,
);

impl_count!(
    "UTF16-LE",
    u16,
    "UTF32",
    u32,
    simdutf_utf32_length_from_utf16le,
    count_utf32_from_utf16le,
);

impl_count!(
    "UTF16-BE",
    u16,
    "UTF8",
    u8,
    simdutf_utf8_length_from_utf16be,
    count_utf8_from_utf16be,
);

impl_count!(
    "UTF16-BE",
    u16,
    "UTF32",
    u32,
    simdutf_utf32_length_from_utf16be,
    count_utf32_from_utf16be,
);

impl_count!(
    "UTF32-LE",
    u32,
    "UTF8",
    u8,
    simdutf_utf8_length_from_utf32,
    count_utf8_from_utf32le,
);

impl_count!(
    "UTF32-LE",
    u32,
    "UTF16",
    u16,
    simdutf_utf16_length_from_utf32,
    count_utf16_from_utf32le,
);

/// Changes the endianness of the utf16 string.
///
/// This function is not BOM-aware.
///
/// # Safety
/// + `src` and `dst` must be non-null and properly aligned.
/// + `src` and `dst` can alias.
/// + `src` must be valid for reads of `len * size_of::<u16>()` bytes.
/// + `dst` must be valid for writes of `len * size_of::<u16>()` bytes.
#[inline]
pub unsafe fn change_endianness_utf16(src: *const u16, len: usize, dst: *mut u16) {
    simdutf_change_endianness_utf16(src, len, dst)
}
