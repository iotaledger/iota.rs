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
    let mut value = 0;
    for trit in trits.iter().rev() {
        value = value * 3 + *trit;
    }
    value
}

/// Converts a slice of trits into a numeric value
pub fn long_value(trits: &[i8]) -> i64 {
    let mut v: i64 = 0;
    for trit in trits.iter().rev() {
        v = v * 3 + i64::from(*trit);
    }
    v
}
