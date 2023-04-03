//! Hand-written bindings and wrappers.

extern "C" {
    fn simdutf_change_endianness_utf16(src: *const u16, len: usize, dst: *mut u16);
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
