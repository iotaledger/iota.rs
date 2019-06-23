#![deny(unused_extern_crates)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;

pub mod iota_units;
mod trinary;
pub mod unit_converter;
pub use trinary::*;

type Result<T> = ::std::result::Result<T, failure::Error>;

/// Converts a slice of trits into a numeric value
pub fn value(trits: &[i8]) -> i8 {
    trits.iter().rev().fold(0, |acc, trit| acc * 3 + *trit)
}

/// Converts a slice of trits into a numeric value
pub fn long_value(trits: &[i8]) -> i64 {
    trits
        .iter()
        .rev()
        .fold(0, |acc, trit| acc * 3 + i64::from(*trit))
}
