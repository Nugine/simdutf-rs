use crate::common::*;
use crate::gen::Codegen;

use std::format as f;

pub fn codegen(g: &mut Codegen) {
    g.lines([
        "// Auto generated by codegen/src/generated.rs", //
        "",
    ]);

    codegen_validate(g);
    codegen_count(g);
    codegen_transcoding_length(g);
    codegen_transcoding_convert(g);
}

fn decl_ne_and_bom(g: &mut Codegen, encoding: &str) {
    if matches!(encoding, "utf16" | "utf32") {
        g.ln("/// This function uses native endianness.");
        g.ln("///");
    }

    if encoding.starts_with("utf16") {
        g.ln("/// This function is not BOM-aware.");
        g.ln("///");
    }
}

fn decl_assume(g: &mut Codegen, encoding: &str) {
    let doc_name = map_doc_name(encoding);
    g.ln(f!("/// + The input string must be valid {doc_name}."));
}

fn decl_dst(g: &mut Codegen, encoding: &str) {
    let to_ch = map_rs_char_type(encoding);
    g.ln("/// + `dst` must be non-null and properly aligned.");
    g.ln(f!("/// + `dst` must be valid for writes of `count * size_of::<{to_ch}>()` bytes, \
        where the `count` is the number of code units ([`{to_ch}`]) after successful conversion."));
}

fn codegen_validate(g: &mut Codegen) {
    for_each_validate(|encoding| {
        let ch = map_rs_char_type(encoding);
        let doc_name = map_doc_name(encoding);

        g.ln(f!("/// Validate the {doc_name} string."));
        g.ln("///");

        decl_ne_and_bom(g, encoding);

        g.ln(f!("/// Returns [`true`] if and only if the string is valid {doc_name}."));

        g.ln("#[inline]");
        g.ln("#[must_use]");
        g.ln(f!("pub fn validate_{encoding}(src: &[{ch}]) -> bool {{"));
        g.ln("let len = src.len();");
        g.ln("let buf = src.as_ptr();");
        g.ln(f!("unsafe {{ crate::bindings::simdutf_validate_{encoding}(buf, len) }}"));
        g.ln("}");
        g.lf();
    });
}

fn codegen_count(g: &mut Codegen) {
    for_each_count(|encoding| {
        let ch = map_rs_char_type(encoding);
        let doc_name = map_doc_name(encoding);

        g.ln(f!("/// Count the number of code points in the {doc_name} string."));
        g.ln("///");

        decl_ne_and_bom(g, encoding);

        g.ln("/// # Safety");
        decl_assume(g, encoding);

        g.ln("#[inline]");
        g.ln("#[must_use]");
        g.ln(f!("pub unsafe fn count_{encoding}(src: &[{ch}]) -> usize {{"));
        g.ln("let len = src.len();");
        g.ln("let buf = src.as_ptr();");
        g.ln(f!("crate::bindings::simdutf_count_{encoding}(buf, len)"));
        g.ln("}");
        g.lf();
    });
}

fn codegen_transcoding_length(g: &mut Codegen) {
    for_each_transcoding_length(|from, to| {
        let from_ch = map_rs_char_type(from);
        let from_doc_name = map_doc_name(from);
        let to_doc_name = map_doc_name(to);

        g.ln(f!("/// Count the number of code units that \
             the {from_doc_name} string would require in {to_doc_name} format."));
        g.ln("///");

        if to == "utf32" {
            g.ln(f!("/// This function is equivalent to [`count_{from}`]."));
            g.ln("///");
        }

        decl_ne_and_bom(g, from);

        g.ln("/// # Safety");
        decl_assume(g, from);

        g.ln("#[inline]");
        g.ln("#[must_use]");
        g.ln(f!("pub unsafe fn {to}_length_from_{from}(src: &[{from_ch}]) -> usize {{"));
        g.ln("let len = src.len();");
        g.ln("let buf = src.as_ptr();");
        g.ln(f!("crate::bindings::simdutf_{to}_length_from_{from}(buf, len)"));
        g.ln("}");
        g.lf();
    })
}

fn codegen_transcoding_convert(g: &mut Codegen) {
    for_each_transcoding_convert(|from, to| {
        let from_ch = map_rs_char_type(from);
        let to_ch = map_rs_char_type(to);
        let from_doc_name = map_doc_name(from);
        let to_doc_name = map_doc_name(to);

        g.ln(f!("/// Convert possibly broken {from_doc_name} string into {to_doc_name} string."));
        g.ln("///");
        g.ln("/// During the conversion also validation of the input string is done.");
        g.ln("/// This function is suitable to work with inputs from untrusted sources.");
        g.ln("///");
        g.ln(f!("/// Returns the number of written code units; \
            0 if the input is not a valid {from_doc_name} string"));
        g.ln("///");

        decl_ne_and_bom(g, from);

        g.ln("/// # Safety");
        decl_dst(g, to);

        g.ln("#[inline]");
        g.ln("#[must_use]");
        g.ln(f!("pub unsafe fn \
                convert_{from}_to_{to}(src: &[{from_ch}], dst: *mut {to_ch}) -> usize {{"));
        g.ln("let len = src.len();");
        g.ln("let buf = src.as_ptr();");
        g.ln(f!("crate::bindings::simdutf_convert_{from}_to_{to}(buf, len, dst)"));
        g.ln("}");
        g.lf();
    });

    for_each_transcoding_convert(|from, to| {
        let from_ch = map_rs_char_type(from);
        let to_ch = map_rs_char_type(to);
        let from_doc_name = map_doc_name(from);
        let to_doc_name = map_doc_name(to);

        g.ln(f!("/// Convert valid {from_doc_name} string into {to_doc_name} string."));
        g.ln("///");
        g.ln("/// Returns the number of written code units.");
        g.ln("///");

        decl_ne_and_bom(g, from);

        g.ln("/// # Safety");
        decl_assume(g, from);
        decl_dst(g, to);

        g.ln("#[inline]");
        g.ln("#[must_use]");
        g.ln(f!("pub unsafe fn \
                convert_valid_{from}_to_{to}(src: &[{from_ch}], dst: *mut {to_ch}) -> usize {{"));
        g.ln("let len = src.len();");
        g.ln("let buf = src.as_ptr();");
        g.ln(f!("crate::bindings::simdutf_convert_valid_{from}_to_{to}(buf, len, dst)"));
        g.ln("}");
        g.lf();
    })
}
