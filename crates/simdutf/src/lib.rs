//! Unicode validation and transcoding at billions of characters per second.
//!
//! This crate is the Rust binding of [simdutf].
//!
//! ## Compilation
//!
//! This crate works out of the box as long as
//! you have a C++11-compatible toolchain installed correctly.
//!
//! [simdutf] links C++ standard library, which adds a dynamic linking dependency.
//!
//! For more details, see [simdutf] documentation and [cc] documentation.
//!
//! Here is an example for local benchmark:
//!
//! ```bash
//! export RUSTFLAGS='-C target-cpu=native'
//! export CXXFLAGS='-march=native'
//! cargo build --release
//! ```
//!
//! [simdutf]: https://github.com/simdutf/simdutf
//! [cc]: https://github.com/rust-lang/cc-rs
//!

#![deny(
    missing_docs,
    missing_debug_implementations,
    clippy::all,
    clippy::cargo,
    clippy::missing_inline_in_public_items,
    clippy::must_use_candidate
)]
#![cfg_attr(not(test), no_std)]

mod bindings;

mod generated;
pub use self::generated::*;

mod extra;
pub use self::extra::*;
