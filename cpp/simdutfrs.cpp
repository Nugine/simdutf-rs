#include "simdutf.cpp"

extern "C" {
bool simdutf_validate_utf8(const char *buf, size_t len) {
    return simdutf::validate_utf8(buf, len);
}

bool simdutf_validate_ascii(const char *buf, size_t len) noexcept {
    return simdutf::validate_ascii(buf, len);
}

bool simdutf_validate_utf16(const char16_t *buf, size_t len) noexcept {
    return simdutf::validate_utf16(buf, len);
}

bool simdutf_validate_utf32(const char32_t *buf, size_t len) noexcept {
    return simdutf::validate_utf32(buf, len);
}

size_t simdutf_convert_utf8_to_utf16(const char *input, size_t length,
                                     char16_t *utf16_output) noexcept {
    return simdutf::convert_utf8_to_utf16(input, length, utf16_output);
}

size_t simdutf_convert_utf8_to_utf32(const char *input, size_t length,
                                     char32_t *utf32_output) noexcept {
    return simdutf::convert_utf8_to_utf32(input, length, utf32_output);
}

size_t simdutf_convert_valid_utf8_to_utf16(const char *input, size_t length,
                                           char16_t *utf16_buffer) noexcept {
    return simdutf::convert_valid_utf8_to_utf16(input, length, utf16_buffer);
}

size_t simdutf_convert_valid_utf8_to_utf32(const char *input, size_t length,
                                           char32_t *utf32_buffer) noexcept {
    return simdutf::convert_valid_utf8_to_utf32(input, length, utf32_buffer);
}

size_t simdutf_utf16_length_from_utf8(const char *input,
                                      size_t length) noexcept {
    return simdutf::utf16_length_from_utf8(input, length);
}

size_t simdutf_utf32_length_from_utf8(const char *input,
                                      size_t length) noexcept {
    return simdutf::utf32_length_from_utf8(input, length);
}

size_t simdutf_convert_utf16_to_utf8(const char16_t *input, size_t length,
                                     char *utf8_buffer) noexcept {
    return simdutf::convert_utf16_to_utf8(input, length, utf8_buffer);
}

size_t simdutf_convert_valid_utf16_to_utf8(const char16_t *input, size_t length,
                                           char *utf8_buffer) noexcept {
    return simdutf::convert_valid_utf16_to_utf8(input, length, utf8_buffer);
}

size_t simdutf_convert_utf16_to_utf32(const char16_t *input, size_t length,
                                      char32_t *utf32_buffer) noexcept {
    return simdutf::convert_utf16_to_utf32(input, length, utf32_buffer);
}

size_t simdutf_convert_valid_utf16_to_utf32(const char16_t *input,
                                            size_t length,
                                            char32_t *utf32_buffer) noexcept {
    return simdutf::convert_valid_utf16_to_utf32(input, length, utf32_buffer);
}

size_t simdutf_utf8_length_from_utf16(const char16_t *input,
                                      size_t length) noexcept {
    return simdutf::utf8_length_from_utf16(input, length);
}

size_t simdutf_convert_utf32_to_utf8(const char32_t *input, size_t length,
                                     char *utf8_buffer) noexcept {
    return simdutf::convert_utf32_to_utf8(input, length, utf8_buffer);
}

size_t simdutf_convert_valid_utf32_to_utf8(const char32_t *input, size_t length,
                                           char *utf8_buffer) noexcept {
    return simdutf::convert_valid_utf32_to_utf8(input, length, utf8_buffer);
}

size_t simdutf_convert_utf32_to_utf16(const char32_t *input, size_t length,
                                      char16_t *utf16_buffer) noexcept {
    return simdutf::convert_utf32_to_utf16(input, length, utf16_buffer);
}

size_t simdutf_convert_valid_utf32_to_utf16(const char32_t *input,
                                            size_t length,
                                            char16_t *utf16_buffer) noexcept {
    return simdutf::convert_valid_utf32_to_utf16(input, length, utf16_buffer);
}

size_t simdutf_utf8_length_from_utf32(const char32_t *input,
                                      size_t length) noexcept {
    return simdutf::utf8_length_from_utf32(input, length);
}

size_t simdutf_utf16_length_from_utf32(const char32_t *input,
                                       size_t length) noexcept {
    return simdutf::utf16_length_from_utf32(input, length);
}

size_t simdutf_utf32_length_from_utf16(const char16_t *input,
                                       size_t length) noexcept {
    return simdutf::utf32_length_from_utf16(input, length);
}

size_t simdutf_count_utf16(const char16_t *input, size_t length) noexcept {
    return simdutf::count_utf16(input, length);
}

size_t simdutf_count_utf8(const char *input, size_t length) noexcept {
    return simdutf::count_utf8(input, length);
}
}
