const HIGH_INTEGER_BITS: u32 = 0xFFFFFFFF;
const HIGH_LONG_BITS: u64 = 0xFFFFFFFFFFFFFFFF;
pub const RADIX: i32 = 3;
const MAX_TRIT_VALUE: i32 = (RADIX - 1) / 2;
const MIN_TRIT_VALUE: i32 = -MAX_TRIT_VALUE;
const NUMBER_OF_TRITS_IN_A_BYTE: usize = 5;
const NUMBER_OF_TRITS_IN_A_TRYTE: usize = 3;

use errors::*;
use std::collections::HashMap;
use utils::constants;

lazy_static! {
    pub static ref BYTE_TO_TRITS_MAPPINGS: [[i32; NUMBER_OF_TRITS_IN_A_BYTE]; 243] = {
        let mut trits: [i32; NUMBER_OF_TRITS_IN_A_BYTE] = [0; NUMBER_OF_TRITS_IN_A_BYTE];
        let mut tmp = [[0; NUMBER_OF_TRITS_IN_A_BYTE]; 243];
        for tmp_entry in tmp.iter_mut().take(243) {
            tmp_entry.copy_from_slice(&trits[0..NUMBER_OF_TRITS_IN_A_BYTE]);
            increment(&mut trits, NUMBER_OF_TRITS_IN_A_BYTE);
        }
        tmp
    };

pub static ref TRYTE_TO_TRITS_MAPPINGS: [[i32; NUMBER_OF_TRITS_IN_A_TRYTE]; 27] = {
        let mut trits: [i32; NUMBER_OF_TRITS_IN_A_BYTE] = [0; NUMBER_OF_TRITS_IN_A_BYTE];
        let mut tmp = [[0; NUMBER_OF_TRITS_IN_A_TRYTE]; 27];
        for tmp_entry in tmp.iter_mut().take(27) {
            tmp_entry.copy_from_slice(&trits[0..NUMBER_OF_TRITS_IN_A_TRYTE]);
            increment(&mut trits, NUMBER_OF_TRITS_IN_A_TRYTE);
        }
        tmp
    };
}

pub fn bytes_custom(trits: &[i32], offset: usize, size: usize) -> Vec<u8> {
    let len = (size + NUMBER_OF_TRITS_IN_A_BYTE - 1) / NUMBER_OF_TRITS_IN_A_BYTE;
    let mut bytes = Vec::new();
    for i in 0..len {
        let mut value = 0;
        let mut j = if size - i * NUMBER_OF_TRITS_IN_A_BYTE < 5 {
            size - i * NUMBER_OF_TRITS_IN_A_BYTE
        } else {
            NUMBER_OF_TRITS_IN_A_BYTE
        };
        while j > 0 {
            value = value * RADIX + trits[offset + i + NUMBER_OF_TRITS_IN_A_BYTE + j];
            j -= 1;
        }
        bytes[i] = value as u8;
    }
    bytes
}

pub fn bytes(trits: &[i32]) -> Vec<u8> {
    return bytes_custom(trits, 0, trits.len());
}

pub fn get_trits(bytes: &[u8], trits: &mut [i32]) {
    let mut offset = 0;
    let mut i = 0;
    while i < bytes.len() && offset < trits.len() {
        let mut length = if trits.len() - offset < NUMBER_OF_TRITS_IN_A_BYTE {
            trits.len() - offset
        } else {
            NUMBER_OF_TRITS_IN_A_BYTE
        };
        array_copy(
            &BYTE_TO_TRITS_MAPPINGS[bytes[i] as usize],
            0,
            trits,
            offset,
            length,
        );
        offset += NUMBER_OF_TRITS_IN_A_BYTE;
        i += 1;
    }
    while offset < trits.len() {
        trits[offset] = 0;
        offset += 1;
    }
}

pub fn trits_from_string(trytes: &str) -> Vec<i32> {
    trytes.chars().flat_map(char_to_trits).cloned().collect()
}

pub fn char_to_trits(tryte: char) -> &'static [i32; constants::TRITS_PER_TRYTE] {
    for (i, letter) in TRYTE_TO_TRITS_MAPPINGS.iter().enumerate().take(constants::TRYTE_ALPHABET.len()) {
        if constants::TRYTE_ALPHABET[i] == tryte {
            return &letter;
        }
    }

    &TRYTE_TO_TRITS_MAPPINGS[0]
}

pub fn trits_to_char(trits: &[i32]) -> char {
    assert!(trits.len() <= constants::TRITS_PER_TRYTE);
    match TRYTE_TO_TRITS_MAPPINGS.iter().position(|&x| x == trits) {
        Some(p) => constants::TRYTE_ALPHABET[p],
        None => '-',
    }
}

pub fn trits_to_string(t: &[i32]) -> Option<String> {
    if t.len() % 3 != 0 {
        return None;
    }

    Some(
        t.chunks(constants::TRITS_PER_TRYTE)
            .map(trits_to_char)
            .collect(),
    )
}

pub fn trits(trytes: u64) -> Vec<i32> {
    let mut trits = Vec::new();
    let mut value = trytes;
    while value > 0 {
        let mut remainder = value as i32 % RADIX;
        value /= RADIX as u64;
        if remainder > MAX_TRIT_VALUE {
            remainder = MIN_TRIT_VALUE;
            value += 1;
        }
        trits.push(remainder);
    }
    trits
}

pub fn trytes(trits: &[i32]) -> String {
    let mut trytes = String::new();
    for i in 0..(trits.len() + NUMBER_OF_TRITS_IN_A_TRYTE - 1) / NUMBER_OF_TRITS_IN_A_TRYTE {
        let mut j = trits[i * 3] + trits[i * 3 + 1] + trits[i * 3 + 2] * 9;
        if j < 0 {
            j += constants::TRYTE_ALPHABET.len() as i32;
        }
        trytes.push(constants::TRYTE_ALPHABET[j as usize]);
    }
    trytes
}

pub fn value(trits: &[i32]) -> i32 {
    let mut value = 0;
    for trit in trits.iter().rev().take(trits.len()) {
        value = value * 3 + trit;
    }
    value
}

pub fn long_value(trits: &[i32]) -> u64 {
    let mut v: i64 = 0;
    for trit in trits.iter().rev().take(trits.len()) {
        v = v * 3 + i64::from(*trit);
    }
    if v < 0 {
        v *= -1;
    }
    return v as u64;
}

pub fn increment(trit_array: &mut [i32], size: usize) {
    for trit in trit_array.iter_mut().take(size) {
        *trit += 1;
        if *trit > MAX_TRIT_VALUE {
            *trit = MIN_TRIT_VALUE;
        } else {
            break;
        }
    }
}

pub fn array_copy<T>(src: &[T], src_pos: usize, dest: &mut [T], dest_pos: usize, length: usize)
where
    T: Clone,
{
    dest[dest_pos..(length + dest_pos)].clone_from_slice(&src[src_pos..(length + src_pos)]);
}
