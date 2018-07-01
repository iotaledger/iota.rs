#![allow(dead_code)]

pub mod api_utils;
pub mod checksum;
pub mod constants;
pub mod converter;
pub mod input_validator;
pub mod iota_units;
pub mod seed_random_generator;
pub mod signing;
pub mod stopwatch;
pub mod trytes_converter;
pub mod types;
pub mod unit_converter;

#[inline]
pub fn array_copy<T>(src: &[T], src_pos: usize, dest: &mut [T], dest_pos: usize, length: usize)
where
    T: Copy,
{
    dest[dest_pos..(length + dest_pos)].copy_from_slice(&src[src_pos..(length + src_pos)]);
}

pub fn right_pad(x: &mut String, len: usize, pad: char) {
    if x.len() < len {
        for _ in x.len()..len {
            x.push(pad);
        }
    }
}
