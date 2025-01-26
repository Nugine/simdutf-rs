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

assert_unchanged:
    #!/bin/bash -ex
    [[ -z "$(git status -s)" ]] # https://stackoverflow.com/a/9393642

upgrade:
    ./scripts/upgrade.py upgrade --force

codegen:
    cargo run -p simdutf-codegen
    cargo fmt
    clang-format -i cpp/simdutfrs.cpp
