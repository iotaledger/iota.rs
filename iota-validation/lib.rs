#![deny(unused_extern_crates)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

//! Validation used throughout iota related crates

#[macro_use]
extern crate lazy_static;

pub use input_validator::*;
pub use validators::*;

/// Module to validate input's type
pub mod input_validator;
mod validators;

type Result<T> = ::std::result::Result<T, failure::Error>;
