mod checksum;
mod iota_units;
mod seed_random_generator;
mod stopwatch;
mod validators;

pub mod api_utils;
pub mod constants;
pub mod converter;
pub mod input_validator;
pub mod trit_adder;
pub mod trytes_converter;
pub mod unit_converter;

pub use self::checksum::*;
pub use self::iota_units::IotaUnits;
pub use self::seed_random_generator::generate_new_seed;
pub use self::stopwatch::StopWatch;
pub use self::validators::*;

pub fn right_pad_string(x: &mut String, len: usize, pad: char) {
    while x.len() < len {
        x.push(pad);
    }
}

pub fn right_pad_vec<T>(x: &mut Vec<T>, len: usize, pad: T)
where
    T: Copy,
{
    while x.len() < len {
        x.push(pad);
    }
}
