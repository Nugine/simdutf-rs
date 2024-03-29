use crate::common::*;

use codegen_writer::g;
use codegen_writer::glines;

pub fn codegen() {
    glines![
        "// Auto generated by codegen/src/api.rs"
        ""
        "use crate::extra::Result;"
        ""
    ];

    codegen_validate();
    codegen_count();
    codegen_transcoding_length();
    codegen_transcoding_convert();
}

fn decl_ne_and_bom(encoding: &str) {
    if matches!(encoding, "utf16" | "utf32") {
        g!("/// This function uses native endianness.");
        g!("///");
    }

    if encoding.starts_with("utf16") {
        g!("/// This function is not BOM-aware.");
        g!("///");
    }
}

fn decl_assume(encoding: &str) {
    let doc_name = map_doc_name(encoding);
    g!("/// + The input string must be valid {doc_name}.");
}

fn decl_src_dst(from: &str, to: &str) {
    let from_ch = map_rs_char_type(from);
    let to_ch = map_rs_char_type(to);
    g!("/// + `src` and `dst` must be non-null and properly aligned.");
    g!("/// + `src` must be valid for reads of `len * size_of::<{from_ch}>()` bytes");
    g!("/// + `dst` must be valid for writes of `count * size_of::<{to_ch}>()` bytes, \
        where the `count` is the number of code units ([`{to_ch}`]) after successful conversion.");
    g!("/// + The memory regions of `src` and `dst` must not overlap."); // TODO: inplace mode?
}

fn codegen_validate() {
    for_each_validate(|encoding| {
        let ch = map_rs_char_type(encoding);
        let doc_name = map_doc_name(encoding);

        g!("/// Validate the {doc_name} string.");
        g!("///");

        decl_ne_and_bom(encoding);

        g!("/// Returns [`true`] if and only if the string is valid {doc_name}.");

        g!("#[inline]");
        g!("#[must_use]");
        g!("pub fn validate_{encoding}(src: &[{ch}]) -> bool {{");
        g!("let len = src.len();");
        g!("let buf = src.as_ptr();");
        g!("unsafe {{ crate::bindings::simdutf_validate_{encoding}(buf, len) }}");
        g!("}}");
        g!();
    });

    for_each_validate(|encoding| {
        let ch = map_rs_char_type(encoding);
        let doc_name = map_doc_name(encoding);

        g!("/// Validate the {doc_name} string.");
        g!("///");

        decl_ne_and_bom(encoding);

        g!("#[inline]");
        g!("#[must_use]");
        g!("pub fn validate_{encoding}_with_errors(src: &[{ch}]) -> Result {{");
        g!("let len = src.len();");
        g!("let buf = src.as_ptr();");
        g!("unsafe {{ crate::bindings::simdutf_validate_{encoding}_with_errors(buf, len) }}");
        g!("}}");
        g!();
    });
}

fn codegen_count() {
    for_each_count(|encoding| {
        let ch = map_rs_char_type(encoding);
        let doc_name = map_doc_name(encoding);

        g!("/// Count the number of code points in the {doc_name} string.");
        g!("///");

        decl_ne_and_bom(encoding);

        g!("/// # Safety");
        decl_assume(encoding);

        g!("#[inline]");
        g!("#[must_use]");
        g!("pub unsafe fn count_{encoding}(src: &[{ch}]) -> usize {{");
        g!("let len = src.len();");
        g!("let buf = src.as_ptr();");
        g!("crate::bindings::simdutf_count_{encoding}(buf, len)");
        g!("}}");
        g!();
    });
}

fn codegen_transcoding_length() {
    for_each_transcoding_length(|from, to| {
        let from_ch = map_rs_char_type(from);
        let from_doc_name = map_doc_name(from);
        let to_doc_name = map_doc_name(to);

        g!("/// Count the number of code units that \
             the {from_doc_name} string would require in {to_doc_name} format.");
        g!("///");

        if to == "utf32" {
            g!("/// This function is equivalent to [`count_{from}`].");
            g!("///");
        }

        decl_ne_and_bom(from);

        g!("/// # Safety");
        decl_assume(from);

        g!("#[inline]");
        g!("#[must_use]");
        g!("pub unsafe fn {to}_length_from_{from}(src: &[{from_ch}]) -> usize {{");
        g!("let len = src.len();");
        g!("let buf = src.as_ptr();");
        g!("crate::bindings::simdutf_{to}_length_from_{from}(buf, len)");
        g!("}}");
        g!();
    })
}

fn codegen_transcoding_convert() {
    for_each_transcoding_convert(|from, to| {
        let from_ch = map_rs_char_type(from);
        let to_ch = map_rs_char_type(to);
        let from_doc_name = map_doc_name(from);
        let to_doc_name = map_doc_name(to);

        g!("/// Convert possibly broken {from_doc_name} string into {to_doc_name} string.");
        g!("///");
        g!("/// During the conversion also validation of the input string is done.");
        g!("/// This function is suitable to work with inputs from untrusted sources.");
        g!("///");
        g!("/// Returns the number of written code units; \
            0 if the input is not a valid {from_doc_name} string");
        g!("///");

        decl_ne_and_bom(from);

        g!("/// # Safety");
        decl_src_dst(from, to);

        g!("#[inline]");
        g!("#[must_use]");
        g!("pub unsafe fn convert_{from}_to_{to}\
            (src: *const {from_ch}, len: usize, dst: *mut {to_ch}) -> usize {{");
        g!("crate::bindings::simdutf_convert_{from}_to_{to}(src, len, dst)");
        g!("}}");
        g!();
    });

    for_each_transcoding_convert(|from, to| {
        let from_ch = map_rs_char_type(from);
        let to_ch = map_rs_char_type(to);
        let from_doc_name = map_doc_name(from);
        let to_doc_name = map_doc_name(to);

        g!("/// Convert possibly broken {from_doc_name} string into {to_doc_name} string.");
        g!("///");
        g!("/// During the conversion also validation of the input string is done.");
        g!("/// This function is suitable to work with inputs from untrusted sources.");
        g!("///");

        decl_ne_and_bom(from);

        g!("/// # Safety");
        decl_src_dst(from, to);

        g!("#[inline]");
        g!("#[must_use]");
        g!("pub unsafe fn convert_{from}_to_{to}_with_errors\
            (src: *const {from_ch}, len: usize, dst: *mut {to_ch}) -> Result {{");
        g!("crate::bindings::simdutf_convert_{from}_to_{to}_with_errors(src, len, dst)");
        g!("}}");
        g!();
    });

    for_each_transcoding_convert(|from, to| {
        let from_ch = map_rs_char_type(from);
        let to_ch = map_rs_char_type(to);
        let from_doc_name = map_doc_name(from);
        let to_doc_name = map_doc_name(to);

        g!("/// Convert valid {from_doc_name} string into {to_doc_name} string.");
        g!("///");
        g!("/// Returns the number of written code units.");
        g!("///");

        decl_ne_and_bom(from);

        g!("/// # Safety");
        decl_assume(from);
        decl_src_dst(from, to);

        g!("#[inline]");
        g!("#[must_use]");
        g!("pub unsafe fn convert_valid_{from}_to_{to}\
            (src: *const {from_ch}, len: usize, dst: *mut {to_ch}) -> usize {{");
        g!("crate::bindings::simdutf_convert_valid_{from}_to_{to}(src, len, dst)");
        g!("}}");
        g!();
    })
}
