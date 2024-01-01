dev:
    cargo fmt
    cargo clippy --all
    just test

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

sync-version:
    ./scripts/check-release.py
    cargo set-version -p simdutf    0.4.16

publish:
    cargo publish -p simdutf

upgrade:
    ./scripts/download.sh
    ./scripts/vendor.sh
    ./scripts/check-release.py
