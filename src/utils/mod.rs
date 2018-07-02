mod iota_units;
mod seed_random_generator;
mod stopwatch;
mod checksum;

pub mod api_utils;
pub mod constants;
pub mod converter;
pub mod input_validator;
pub mod trit_adder;
pub mod trytes_converter;
pub mod unit_converter;

pub use self::checksum::*;
pub use self::stopwatch::StopWatch;
pub use self::seed_random_generator::generate_new_seed;
pub use self::iota_units::IotaUnits;

pub fn right_pad(x: &mut String, len: usize, pad: char) {
    if x.len() < len {
        for _ in x.len()..len {
            x.push(pad);
        }
    }
}
