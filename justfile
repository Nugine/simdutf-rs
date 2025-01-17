dev:
    just fmt
    just lint
    just test

fmt *ARGS:
    cargo fmt --all {{ARGS}}

lint *ARGS:
    cargo clippy --all-features --tests --benches {{ARGS}}

test *ARGS:
    cargo test -p simdutf {{ARGS}}
    cargo test -p simdutf --release {{ARGS}}
    RUSTFLAGS='-C target-cpu=native' CXXFLAGS='-march=native' cargo test -p simdutf --release {{ARGS}}

doc *ARGS:
    RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --open --no-deps --all-features {{ARGS}}

ci:
    just fmt --check
    just lint -- -D warnings
    just test

codegen:
    cargo run -p simdutf-codegen
    cargo fmt
    clang-format -i crates/simdutf/cpp/simdutfrs.cpp

sync-version:
    ./scripts/check-release.py
    cargo set-version -p simdutf    0.5.72

upgrade:
    ./scripts/download.sh
    ./scripts/vendor.sh
    ./scripts/check-release.py
