#![deny(unused_extern_crates)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;

use iota_constants::{self, TRITS_PER_BYTE, TRITS_PER_TRYTE};

pub mod iota_units;
mod trinary;
pub mod unit_converter;

pub use trinary::*;

type Result<T> = ::std::result::Result<T, failure::Error>;

lazy_static! {
    /// Provides a byte to trits mapping
    pub static ref BYTE_TO_TRITS_MAPPINGS: [[i8; TRITS_PER_BYTE]; 243] = {
        let mut trits: [i8; TRITS_PER_BYTE] = [0; TRITS_PER_BYTE];
        let mut tmp = [[0; TRITS_PER_BYTE]; 243];
        for tmp_entry in tmp.iter_mut().take(243) {
            tmp_entry.copy_from_slice(&trits[0..TRITS_PER_BYTE]);
            increment(&mut trits, TRITS_PER_BYTE);
        }
        tmp
    };
    /// Provides a trytes to trits mapping
    pub static ref TRYTE_TO_TRITS_MAPPINGS: [[i8; TRITS_PER_TRYTE]; 27] = {
        let mut trits: [i8; TRITS_PER_BYTE] = [0; TRITS_PER_BYTE];
        let mut tmp = [[0; TRITS_PER_TRYTE]; 27];
        for tmp_entry in tmp.iter_mut().take(27) {
            tmp_entry.copy_from_slice(&trits[0..TRITS_PER_TRYTE]);
            increment(&mut trits, TRITS_PER_TRYTE);
        }
        tmp
    };
}

/// Converts a char into and array of trits
pub fn char_to_trits(tryte: char) -> &'static [i8] {
    for (i, mapping) in TRYTE_TO_TRITS_MAPPINGS
        .iter()
        .enumerate()
        .take(iota_constants::TRYTE_ALPHABET.len())
    {
        if iota_constants::TRYTE_ALPHABET[i] == tryte {
            return mapping;
        }
    }

    &TRYTE_TO_TRITS_MAPPINGS[0]
}

/// Converts a slice of trits into a char
pub fn trits_to_char(trits: &[i8]) -> char {
    assert!(trits.len() <= iota_constants::TRITS_PER_TRYTE);
    match TRYTE_TO_TRITS_MAPPINGS.iter().position(|&x| x == trits) {
        Some(p) => iota_constants::TRYTE_ALPHABET[p],
        None => '-',
    }
}

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

/// Increments a trit slice in place, only considering trits until index `size`
fn increment(trit_array: &mut [i8], size: usize) {
    for trit in trit_array.iter_mut().take(size) {
        *trit += 1;
        if *trit > iota_constants::MAX_TRIT_VALUE {
            *trit = iota_constants::MIN_TRIT_VALUE;
        } else {
            break;
        }
    }
}
