mod checksum;
mod iota_units;
mod seed_random_generator;
mod stopwatch;
mod validators;

/// Provides constants for use throughout the library
pub mod constants;
/// Provides useful conversions between trinary types
pub mod converter;
/// Provides validation for types used throughout the library
pub mod input_validator;
/// Provides an adder that sums to slices of trits
pub mod trit_adder;
/// Provides a converter between ascii and tryte-encoded strings
pub mod trytes_converter;
/// Provides unit conversion for Iota
pub mod unit_converter;

pub use self::checksum::*;
pub use self::iota_units::IotaUnits;
pub use self::seed_random_generator::generate_new_seed;
pub use self::stopwatch::StopWatch;
pub use self::validators::*;

/// Right pads a string to a certain length in place
pub fn right_pad_string(x: &mut String, len: usize, pad: char) {
    while x.len() < len {
        x.push(pad);
    }
}

/// Right pads a vector to a certain length in place
pub fn right_pad_vec<T>(x: &mut Vec<T>, len: usize, pad: T)
where
    T: Copy,
{
    while x.len() < len {
        x.push(pad);
    }
}
