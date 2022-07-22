//! TODO

#![deny(missing_docs, clippy::all)]
#![cfg_attr(no_std, not(test))]

mod bindings;

mod api;
pub use self::api::*;

#[cfg(test)]
mod tests;
