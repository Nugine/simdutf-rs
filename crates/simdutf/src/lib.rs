//! Unicode validation and transcoding at billions of characters per second.
//!
//! This crate is the Rust binding of [simdutf](https://github.com/simdutf/simdutf).

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
