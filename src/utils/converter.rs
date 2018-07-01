use super::array_copy;
use super::constants;

const HIGH_INTEGER_BITS: u32 = 0xFFFF_FFFF;
const HIGH_LONG_BITS: u64 = 0xFFFF_FFFF_FFFF_FFFF;
pub const RADIX: i8 = 3;
pub const MAX_TRIT_VALUE: i8 = (RADIX - 1) / 2;
pub const MIN_TRIT_VALUE: i8 = -MAX_TRIT_VALUE;
const NUMBER_OF_TRITS_IN_A_BYTE: usize = 5;
const NUMBER_OF_TRITS_IN_A_TRYTE: usize = 3;

lazy_static! {
    pub static ref BYTE_TO_TRITS_MAPPINGS: [[i8; NUMBER_OF_TRITS_IN_A_BYTE]; 243] = {
        let mut trits: [i8; NUMBER_OF_TRITS_IN_A_BYTE] = [0; NUMBER_OF_TRITS_IN_A_BYTE];
        let mut tmp = [[0; NUMBER_OF_TRITS_IN_A_BYTE]; 243];
        for tmp_entry in tmp.iter_mut().take(243) {
            tmp_entry.copy_from_slice(&trits[0..NUMBER_OF_TRITS_IN_A_BYTE]);
            increment(&mut trits, NUMBER_OF_TRITS_IN_A_BYTE);
        }
        tmp
    };
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

pub fn bytes_custom(trits: &[i8], offset: usize, size: usize) -> Vec<u8> {
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

pub fn bytes(trits: &[i8]) -> Vec<u8> {
    bytes_custom(trits, 0, trits.len())
}

pub fn get_trits(bytes: &[u8], trits: &mut [i8]) {
    let mut offset = 0;
    let mut i = 0;
    while i < bytes.len() && offset < trits.len() {
        let length = if trits.len() - offset < NUMBER_OF_TRITS_IN_A_BYTE {
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

pub fn trits_from_string(trytes: &str) -> Vec<i8> {
    trytes.chars().flat_map(char_to_trits).cloned().collect()
}

pub fn trits_from_string_with_length(trytes: &str, length: usize) -> Vec<i8> {
    let tmp: Vec<i8> = trytes.chars().flat_map(char_to_trits).cloned().collect();
    if tmp.len() < length {
        let mut result = vec![0; length];
        result[..tmp.len()].clone_from_slice(&tmp[..]);
        return result;
    }
    tmp
}

pub fn char_to_trits(tryte: char) -> &'static [i8; constants::TRITS_PER_TRYTE] {
    for (i, mapping) in TRYTE_TO_TRITS_MAPPINGS
        .iter()
        .enumerate()
        .take(constants::TRYTE_ALPHABET.len())
    {
        if constants::TRYTE_ALPHABET[i] == tryte {
            return &mapping;
        }
    }

    &TRYTE_TO_TRITS_MAPPINGS[0]
}

pub fn trits_to_char(trits: &[i8]) -> char {
    assert!(trits.len() <= constants::TRITS_PER_TRYTE);
    match TRYTE_TO_TRITS_MAPPINGS.iter().position(|&x| x == trits) {
        Some(p) => constants::TRYTE_ALPHABET[p],
        None => '-',
    }
}

pub fn trits_to_string(t: &[i8]) -> Option<String> {
    if t.len() % 3 != 0 {
        return None;
    }

    Some(
        t.chunks(constants::TRITS_PER_TRYTE)
            .map(trits_to_char)
            .collect(),
    )
}

pub fn trits(trytes: i64) -> Vec<i8> {
    let mut trits = Vec::new();
    let mut abs = trytes.abs();
    while abs > 0 {
        let mut remainder = (abs % i64::from(RADIX)) as i8;
        abs /= i64::from(RADIX);
        if remainder > MAX_TRIT_VALUE {
            remainder = MIN_TRIT_VALUE;
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

pub fn trits_with_length(trytes: i64, length: usize) -> Vec<i8> {
    let tmp: Vec<i8> = trits(trytes);
    if tmp.len() < length {
        let mut result = vec![0; length];
        result[..tmp.len()].clone_from_slice(&tmp[..]);
        return result;
    }
    tmp
}

pub fn copy_trits(value: i64, destination: &mut [i8], offset: usize, size: usize) {
    let mut abs = value.abs();
    for i in 0..size {
        let mut remainder = (abs % i64::from(RADIX)) as i8;
        abs /= i64::from(RADIX);
        if remainder > MAX_TRIT_VALUE {
            remainder = MIN_TRIT_VALUE;
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

pub fn value(trits: &[i8]) -> i8 {
    let mut value = 0;
    for trit in trits.iter().rev() {
        value = value * 3 + *trit;
    }
    value
}

pub fn long_value(trits: &[i8]) -> i64 {
    let mut v: i64 = 0;
    for trit in trits.iter().rev() {
        v = v * 3 + i64::from(*trit);
    }
    v
}

pub fn increment(trit_array: &mut [i8], size: usize) {
    for trit in trit_array.iter_mut().take(size) {
        *trit += 1;
        if *trit > MAX_TRIT_VALUE {
            *trit = MIN_TRIT_VALUE;
        } else {
            break;
        }
    }
}
