use super::constants;
use crate::Result;

const HIGH_INTEGER_BITS: u32 = 0xFFFF_FFFF;
const HIGH_LONG_BITS: u64 = 0xFFFF_FFFF_FFFF_FFFF;
const RADIX: i8 = 3;
const NUMBER_OF_TRITS_IN_A_BYTE: usize = 5;
const NUMBER_OF_TRITS_IN_A_TRYTE: usize = 3;

lazy_static! {
    /// Provides a byte to trits mapping
    pub static ref BYTE_TO_TRITS_MAPPINGS: [[i8; NUMBER_OF_TRITS_IN_A_BYTE]; 243] = {
        let mut trits: [i8; NUMBER_OF_TRITS_IN_A_BYTE] = [0; NUMBER_OF_TRITS_IN_A_BYTE];
        let mut tmp = [[0; NUMBER_OF_TRITS_IN_A_BYTE]; 243];
        for tmp_entry in tmp.iter_mut().take(243) {
            tmp_entry.copy_from_slice(&trits[0..NUMBER_OF_TRITS_IN_A_BYTE]);
            increment(&mut trits, NUMBER_OF_TRITS_IN_A_BYTE);
        }
        tmp
    };
    /// Provides a trytes to trits mapping
    pub static ref TRYTE_TO_TRITS_MAPPINGS: [[i8; NUMBER_OF_TRITS_IN_A_TRYTE]; 27] = {
        let mut trits: [i8; NUMBER_OF_TRITS_IN_A_BYTE] = [0; NUMBER_OF_TRITS_IN_A_BYTE];
        let mut tmp = [[0; NUMBER_OF_TRITS_IN_A_TRYTE]; 27];
        for tmp_entry in tmp.iter_mut().take(27) {
            tmp_entry.copy_from_slice(&trits[0..NUMBER_OF_TRITS_IN_A_TRYTE]);
            increment(&mut trits, NUMBER_OF_TRITS_IN_A_TRYTE);
        }
        tmp
    };
}

/// Converts a provided slice of bytes into trits and stores them
/// in place into `trits`
pub fn get_trits(bytes: &[u8], trits: &mut [i8]) {
    let mut offset = 0;
    let mut i = 0;
    while i < bytes.len() && offset < trits.len() {
        let length = if trits.len() - offset < NUMBER_OF_TRITS_IN_A_BYTE {
            trits.len() - offset
        } else {
            NUMBER_OF_TRITS_IN_A_BYTE
        };
        trits[offset..offset + length]
            .copy_from_slice(&BYTE_TO_TRITS_MAPPINGS[bytes[i] as usize][0..length]);
        offset += NUMBER_OF_TRITS_IN_A_BYTE;
        i += 1;
    }
    while offset < trits.len() {
        trits[offset] = 0;
        offset += 1;
    }
}

/// Converts a string into trits
pub fn trits_from_string(trytes: &str) -> Vec<i8> {
    trytes.chars().flat_map(char_to_trits).cloned().collect()
}

/// Converts a string into trits and ensures the output length
pub fn trits_from_string_with_length(trytes: &str, length: usize) -> Vec<i8> {
    let tmp: Vec<i8> = trits_from_string(trytes);
    if tmp.len() < length {
        let mut result = vec![0; length];
        result[..tmp.len()].copy_from_slice(&tmp[..]);
        result
    } else {
        tmp[0..length].to_vec()
    }
}

/// Converts a char into and array of trits
pub fn char_to_trits(tryte: char) -> &'static [i8] {
    for (i, mapping) in TRYTE_TO_TRITS_MAPPINGS
        .iter()
        .enumerate()
        .take(constants::TRYTE_ALPHABET.len())
    {
        if constants::TRYTE_ALPHABET[i] == tryte {
            return mapping;
        }
    }

    &TRYTE_TO_TRITS_MAPPINGS[0]
}

/// Converts a slice of trits into a char
pub fn trits_to_char(trits: &[i8]) -> char {
    assert!(trits.len() <= constants::TRITS_PER_TRYTE);
    match TRYTE_TO_TRITS_MAPPINGS.iter().position(|&x| x == trits) {
        Some(p) => constants::TRYTE_ALPHABET[p],
        None => '-',
    }
}

/// Converts a slice of trits into a string
pub fn trits_to_string(t: &[i8]) -> Result<String> {
    ensure!(t.len() % 3 == 0, "Invalid trit length.");

    Ok(t.chunks(constants::TRITS_PER_TRYTE)
        .map(trits_to_char)
        .collect())
}

/// Converts a numeric representation of trytes into a vec of trits
pub fn trits(trytes: i64) -> Vec<i8> {
    let mut trits = Vec::new();
    let mut abs = trytes.abs();
    while abs > 0 {
        let mut remainder = (abs % i64::from(RADIX)) as i8;
        abs /= i64::from(RADIX);
        if remainder > constants::MAX_TRIT_VALUE {
            remainder = constants::MIN_TRIT_VALUE;
            abs += 1;
        }
        trits.push(remainder);
    }
    if trytes < 0 {
        for trit in &mut trits {
            *trit = -*trit;
        }
    }
    trits
}

/// Converts a numeric representation of trytes into a vec of trits with a guaranteed length
pub fn trits_with_length(trytes: i64, length: usize) -> Vec<i8> {
    let tmp: Vec<i8> = trits(trytes);
    if tmp.len() < length {
        let mut result = vec![0; length];
        result[..tmp.len()].copy_from_slice(&tmp[..]);
        result
    } else {
        tmp[0..length].to_vec()
    }
}

/// Copy
fn copy_trits(value: i64, destination: &mut [i8], offset: usize, size: usize) {
    let mut abs = value.abs();
    for i in 0..size {
        let mut remainder = (abs % i64::from(RADIX)) as i8;
        abs /= i64::from(RADIX);
        if remainder > constants::MAX_TRIT_VALUE {
            remainder = constants::MIN_TRIT_VALUE;
            abs += 1;
        }
        destination[offset + i] = remainder;
    }

    if value < 0 {
        for i in 0..size {
            destination[offset + i] = -destination[offset + i];
        }
    }
}

/// Converts a slice of trits into a tryte string
pub fn trytes(trits: &[i8]) -> String {
    let mut trytes = String::new();
    for i in 0..(trits.len() + NUMBER_OF_TRITS_IN_A_TRYTE - 1) / NUMBER_OF_TRITS_IN_A_TRYTE {
        let mut j = trits[i * 3] + trits[i * 3 + 1] * 3 + trits[i * 3 + 2] * 9;
        if j < 0 {
            j += constants::TRYTE_ALPHABET.len() as i8;
        }
        trytes.push(constants::TRYTE_ALPHABET[j as usize]);
    }
    trytes
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
pub fn increment(trit_array: &mut [i8], size: usize) {
    for trit in trit_array.iter_mut().take(size) {
        *trit += 1;
        if *trit > constants::MAX_TRIT_VALUE {
            *trit = constants::MIN_TRIT_VALUE;
        } else {
            break;
        }
    }
}
