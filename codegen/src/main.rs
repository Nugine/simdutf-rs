mod common;

mod api;
mod bindings;

use codegen_writer::Codegen;

fn main() {
    {
        let path = "crates/simdutf/cpp/simdutfrs.cpp";
        let gen = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(gen, bindings::codegen_cpp);
    }
    {
        let path = "crates/simdutf/src/bindings.rs";
        let gen = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(gen, bindings::codegen_rust);
    }
    {
        let path = "crates/simdutf/src/generated.rs";
        let gen = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(gen, api::codegen);
    }
}
