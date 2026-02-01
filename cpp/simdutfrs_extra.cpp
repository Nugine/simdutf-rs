// Hand-written bindings

#include "simdutf.h"

extern "C" {

struct simdutfrs_result_t {
    uint32_t error;
    size_t count;
};

void simdutf_change_endianness_utf16(const char16_t *src, size_t len,
                                     char16_t *dst) {
    return simdutf::change_endianness_utf16(src, len, dst);
}

// Note: upstream simdutf v8.0.0 provides simdutf_autodetect_encoding but with different return type
// We provide a wrapper with simdutfrs_ prefix that returns uint32_t for Rust compatibility
uint32_t simdutfrs_autodetect_encoding(const char *src, size_t len) {
    const simdutf::encoding_type encoding =
        simdutf::autodetect_encoding(src, len);
    return static_cast<uint32_t>(encoding);
}

// Note: upstream simdutf v8.0.0 provides simdutf_detect_encodings but with different return type
// We provide a wrapper with simdutfrs_ prefix that returns uint32_t for Rust compatibility
uint32_t simdutfrs_detect_encodings(const char *src, size_t len) {
    const int encoding = simdutf::detect_encodings(src, len);
    return static_cast<uint32_t>(encoding);
}
}