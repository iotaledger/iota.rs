pub mod api_utils;
pub mod checksum;
pub mod constants;
pub mod converter;
pub mod input_validator;
pub mod iota_units;
pub mod seed_random_generator;
pub mod signing;
pub mod stopwatch;
pub mod trit_adder;
pub mod trytes_converter;
pub mod types;
pub mod unit_converter;

pub fn right_pad(x: &mut String, len: usize, pad: char) {
    if x.len() < len {
        for _ in x.len()..len {
            x.push(pad);
        }
    }
}
