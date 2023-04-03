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

    g.ln("/// # Safety");
    g.ln(f!("/// This function assumes that the input string is valid {doc_name}."));
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
        g.ln(f!("pub fn validate_{encoding}(data: &[{ch}]) -> bool {{"));
        g.ln("let len = data.len();");
        g.ln("let buf = data.as_ptr();");
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
        decl_assume(g, encoding);

        g.ln("#[inline]");
        g.ln("#[must_use]");
        g.ln(f!("pub unsafe fn count_{encoding}(data: &[{ch}]) -> usize {{"));
        g.ln("let len = data.len();");
        g.ln("let buf = data.as_ptr();");
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
        decl_assume(g, from);

        g.ln("#[inline]");
        g.ln("#[must_use]");
        g.ln(f!("pub unsafe fn {to}_length_from_{from}(data: &[{from_ch}]) -> usize {{"));
        g.ln("let len = data.len();");
        g.ln("let buf = data.as_ptr();");
        g.ln(f!("crate::bindings::simdutf_{to}_length_from_{from}(buf, len)"));
        g.ln("}");
        g.lf();
    })
}
