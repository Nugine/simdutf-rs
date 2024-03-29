use crate::common::*;

use codegen_writer::g;
use codegen_writer::glines;

pub fn codegen_cpp() {
    glines![
        "// Auto generated by codegen/src/bindings.rs"
        "#include \"simdutf.cpp\""
        "#include \"simdutfrs_extra.cpp\""
        ""
        "extern \"C\" {"
        ""
    ];

    for_each_validate(|encoding| {
        let ch = map_cpp_char_type(encoding);
        g!("bool simdutf_validate_{encoding}(const {ch}* buf, size_t len) {{");
        g!("    return simdutf::validate_{encoding}(buf, len);");
        g!("}}");
        g!();
    });

    for_each_validate(|encoding| {
        let ch = map_cpp_char_type(encoding);
        g!("simdutfrs_result_t simdutf_validate_{encoding}_with_errors(const {ch}* buf, size_t len) {{");
        g!("    const simdutf::result ans = simdutf::validate_{encoding}_with_errors(buf, len);");
        g!("    return {{ static_cast<uint32_t>(ans.error), ans.count }};");
        g!("}}");
        g!();
    });

    for_each_count(|encoding| {
        let ch = map_cpp_char_type(encoding);
        g!("size_t simdutf_count_{encoding}(const {ch}* buf, size_t len) {{");
        g!("    return simdutf::count_{encoding}(buf, len);");
        g!("}}");
        g!();
    });

    for_each_transcoding_length(|from, to| {
        let from_ch = map_cpp_char_type(from);
        g!("size_t simdutf_{to}_length_from_{from}(const {from_ch}* buf, size_t len) {{");
        g!("    return simdutf::{to}_length_from_{from}(buf, len);");
        g!("}}");
        g!();
    });

    for_each_transcoding_convert(|from, to| {
        let from_ch = map_cpp_char_type(from);
        let to_ch = map_cpp_char_type(to);
        g!("size_t simdutf_convert_{from}_to_{to}(const {from_ch}* src, size_t len, {to_ch}* dst) {{");
        g!("    return simdutf::convert_{from}_to_{to}(src, len, dst);");
        g!("}}");
        g!();
    });

    for_each_transcoding_convert(|from, to| {
        let from_ch = map_cpp_char_type(from);
        let to_ch = map_cpp_char_type(to);
        g!("simdutfrs_result_t \
            simdutf_convert_{from}_to_{to}_with_errors(const {from_ch}* src, size_t len, {to_ch}* dst) {{");
        g!("const simdutf::result ans = \
            simdutf::convert_{from}_to_{to}_with_errors(src, len, dst);");
        g!("    return {{ static_cast<uint32_t>(ans.error), ans.count }};");
        g!("}}");
        g!();
    });

    for_each_transcoding_convert(|from, to| {
        let from_ch = map_cpp_char_type(from);
        let to_ch = map_cpp_char_type(to);
        g!("size_t simdutf_convert_valid_{from}_to_{to}(const {from_ch}* src, size_t len, {to_ch}* dst) {{");
        g!("    return simdutf::convert_valid_{from}_to_{to}(src, len, dst);");
        g!("}}");
        g!();
    });

    g!("}}");
}

pub fn codegen_rust() {
    glines![
        "// Auto generated by codegen/src/bindings.rs"
        ""
        "use crate::extra::Result;"
        ""
        "extern \"C\" {"
        ""
    ];

    for_each_validate(|encoding| {
        let ch = map_rs_char_type(encoding);
        g!("pub fn simdutf_validate_{encoding}(buf: *const {ch}, len: usize) -> bool;");
    });
    g!();

    for_each_validate(|encoding| {
        let ch = map_rs_char_type(encoding);
        g!("pub fn simdutf_validate_{encoding}_with_errors\
            (buf: *const {ch}, len: usize) -> Result;");
    });
    g!();

    for_each_count(|encoding| {
        let ch = map_rs_char_type(encoding);
        g!("pub fn simdutf_count_{encoding}(buf: *const {ch}, len: usize) -> usize;");
    });
    g!();

    for_each_transcoding_length(|from, to| {
        let from_ch = map_rs_char_type(from);
        g!("pub fn simdutf_{to}_length_from_{from}(buf: *const {from_ch}, len: usize) -> usize;");
    });
    g!();

    for_each_transcoding_convert(|from, to| {
        let from_ch = map_rs_char_type(from);
        let to_ch = map_rs_char_type(to);
        g!("pub fn simdutf_convert_{from}_to_{to}\
            (src: *const {from_ch}, len: usize, dst: *mut {to_ch}) -> usize;");
    });
    g!();

    for_each_transcoding_convert(|from, to| {
        let from_ch = map_rs_char_type(from);
        let to_ch = map_rs_char_type(to);
        g!("pub fn simdutf_convert_{from}_to_{to}_with_errors\
            (src: *const {from_ch}, len: usize, dst: *mut {to_ch}) -> Result;");
    });
    g!();

    for_each_transcoding_convert(|from, to| {
        let from_ch = map_rs_char_type(from);
        let to_ch = map_rs_char_type(to);
        g!("pub fn simdutf_convert_valid_{from}_to_{to}\
            (src: *const {from_ch}, len: usize, dst: *mut {to_ch}) -> usize;");
    });
    g!();

    g!("}}");
}
