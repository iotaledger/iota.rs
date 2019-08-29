#![deny(unused_extern_crates)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

//! Trinary and unit conversion traits and methods

#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;

/// Provides useful unit definitions for Iota
pub mod iota_units;
mod trinary;
/// Converts between strings and tryte-encoded strings
pub mod trytes_converter;
/// Provides converters between various unit representations of Iota
pub mod unit_converter;

pub use trinary::*;

type Result<T> = ::std::result::Result<T, failure::Error>;

/// Converts a slice of trits into a numeric value
pub fn value(trits: &[i8]) -> i8 {
    trits.iter().rev().fold(0, |acc, trit| acc * 3 + *trit)
}

/// Converts a slice of trits into a numeric value in i64
pub fn long_value(trits: &[i8]) -> i64 {
    trits
        .iter()
        .rev()
        .fold(0, |acc, trit| acc * 3 + i64::from(*trit))
}
