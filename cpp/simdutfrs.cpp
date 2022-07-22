#include "simdutf.cpp"

extern "C" {
bool simdutf_validate_utf8(const char *buf, size_t len) {
    return simdutf::validate_utf8(buf, len);
}
}
