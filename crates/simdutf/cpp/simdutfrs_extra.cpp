// Hand-written bindings

#include "simdutf.h"

extern "C" {

void simdutf_change_endianness_utf16(const char16_t *src, size_t len,
                                     char16_t *dst) {
    return simdutf::change_endianness_utf16(src, len, dst);
}

uint32_t simdutf_autodetect_encoding(const char *src, size_t len) {
    const simdutf::encoding_type encoding =
        simdutf::autodetect_encoding(src, len);
    return static_cast<uint32_t>(encoding);
}

uint32_t simdutf_detect_encodings(const char *src, size_t len) {
    const int encoding = simdutf::detect_encodings(src, len);
    return static_cast<uint32_t>(encoding);
}
}