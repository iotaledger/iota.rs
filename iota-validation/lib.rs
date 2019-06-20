#![deny(unused_extern_crates)]

#[macro_use]
extern crate lazy_static;

pub use input_validator::*;
pub use validators::*;

pub mod input_validator;
mod validators;

type Result<T> = ::std::result::Result<T, failure::Error>;
