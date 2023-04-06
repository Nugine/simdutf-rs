dev:
    cargo fmt
    cargo clippy --all

doc:
    cargo doc --open --no-deps

codegen:
    cargo run -p simdutf-codegen
    cargo fmt
    clang-format -i crates/simdutf/cpp/simdutfrs.cpp

test:
    cargo test -p simdutf
    cargo test -p simdutf --release
    RUSTFLAGS='-C target-cpu=native' CXXFLAGS='-march=native' cargo test -p simdutf --release
