// Hand-written bindings

#include "simdutf.h"

extern "C" {

void simdutf_change_endianness_utf16(const char16_t *src, size_t len,
                                     char16_t *dst) {
    return simdutf::change_endianness_utf16(src, len, dst);
}
}