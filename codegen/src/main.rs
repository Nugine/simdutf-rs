mod common;

mod api;
mod bindings;

use codegen_writer::Codegen;

fn main() {
    {
        let path = "cpp/simdutfrs.cpp";
        let gen = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(gen, bindings::codegen_cpp);
    }
    {
        let path = "src/bindings.rs";
        let gen = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(gen, bindings::codegen_rust);
    }
    {
        let path = "src/generated.rs";
        let gen = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(gen, api::codegen);
    }
}
