//! Unicode validation and transcoding at billions of characters per second.
//!
//! This crate is the rust binding of [simdutf](https://github.com/simdutf/simdutf).

#![deny(missing_docs, clippy::all, clippy::cargo)]
#![cfg_attr(no_std, not(test))]

mod bindings;

mod api;
pub use self::api::*;

#[cfg(test)]
mod tests;
