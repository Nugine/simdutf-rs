mod common;
mod gen;

mod bindings;
mod cpp;
mod validate;

use self::gen::Codegen;

fn main() {
    {
        let path = "crates/simdutf/cpp/simdutfrs.cpp";
        let mut gen = Codegen::create_file(path).unwrap();
        cpp::codegen(&mut gen);
    }
    {
        let path = "crates/simdutf/src/bindings.rs";
        let mut gen = Codegen::create_file(path).unwrap();
        bindings::codegen(&mut gen);
    }
    {
        let path = "crates/simdutf/src/validate.rs";
        let mut gen = Codegen::create_file(path).unwrap();
        validate::codegen(&mut gen);
    }
}
