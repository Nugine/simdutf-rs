mod common;
mod gen;

mod api;
mod bindings;

use self::gen::Codegen;

fn main() {
    {
        let path = "crates/simdutf/cpp/simdutfrs.cpp";
        let mut gen = Codegen::create_file(path).unwrap();
        bindings::codegen_cpp(&mut gen);
    }
    {
        let path = "crates/simdutf/src/bindings.rs";
        let mut gen = Codegen::create_file(path).unwrap();
        bindings::codegen_rust(&mut gen);
    }
    {
        let path = "crates/simdutf/src/generated.rs";
        let mut gen = Codegen::create_file(path).unwrap();
        api::codegen(&mut gen);
    }
}
