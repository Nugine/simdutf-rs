fn main() {
    cc::Build::new()
        .file("cpp/simdutfrs.cpp")
        .compile("simdutfrs");

    // FIXME: https://github.com/simdutf/simdutf/issues/158
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
