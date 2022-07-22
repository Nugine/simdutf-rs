dev:
    cargo fmt
    cargo clippy --all
    cargo build --release
    cargo test

doc:
    cargo doc --open --no-deps
