// Hand-written bindings

#include "simdutf.h"

extern "C" {

struct simdutfrs_result_t {
    uint32_t error;
    size_t count;
};

#if SIMDUTF_FEATURE_UTF16
void simdutfrs_change_endianness_utf16(const char16_t *src, size_t len,
                                       char16_t *dst) {
    return simdutf::change_endianness_utf16(src, len, dst);
}
#endif // SIMDUTF_FEATURE_UTF16

#if SIMDUTF_FEATURE_DETECT_ENCODING
uint32_t simdutfrs_autodetect_encoding(const char *src, size_t len) {
    const simdutf::encoding_type encoding =
        simdutf::autodetect_encoding(src, len);
    return static_cast<uint32_t>(encoding);
}

uint32_t simdutfrs_detect_encodings(const char *src, size_t len) {
    const int encoding = simdutf::detect_encodings(src, len);
    return static_cast<uint32_t>(encoding);
}
#endif // SIMDUTF_FEATURE_DETECT_ENCODING
}