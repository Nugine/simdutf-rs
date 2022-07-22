//! Unicode validation and transcoding at billions of characters per second.
//!
//! This crate is the rust binding of [simdutf](https://github.com/simdutf/simdutf).

#![deny(
    missing_docs,
    missing_debug_implementations,
    clippy::all,
    clippy::cargo,
    clippy::missing_inline_in_public_items
)]
#![cfg_attr(no_std, not(test))]

mod bindings;

mod api;
pub use self::api::*;

#[cfg(test)]
mod tests;
