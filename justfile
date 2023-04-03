dev:
    cargo fmt
    cargo clippy --all
    cargo build -p simdutf --release
    cargo test -p simdutf

doc:
    cargo doc --open --no-deps

codegen:
    cargo run -p simdutf-codegen
    cargo fmt
    clang-format -i crates/simdutf/cpp/simdutfrs.cpp
