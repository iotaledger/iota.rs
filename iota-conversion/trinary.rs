use crate::Result;
use iota_constants::{TRINARY_RADIX, TRITS_PER_BYTE, TRITS_PER_TRYTE};

lazy_static! {
    /// Provides a byte to trits mapping
    pub static ref BYTE_TO_TRITS_MAPPINGS: [[i8; TRITS_PER_BYTE]; 243] = {
        let mut trits: [i8; TRITS_PER_BYTE] = [0; TRITS_PER_BYTE];
        let mut tmp = [[0; TRITS_PER_BYTE]; 243];
        tmp.iter_mut().for_each(|tmp_entry| {
            tmp_entry.copy_from_slice(&trits[0..TRITS_PER_BYTE]);
            increment(&mut trits, TRITS_PER_BYTE);
        });
        tmp
    };
    /// Provides a trytes to trits mapping
    pub static ref TRYTE_TO_TRITS_MAPPINGS: [[i8; TRITS_PER_TRYTE]; 27] = {
        let mut trits: [i8; TRITS_PER_BYTE] = [0; TRITS_PER_BYTE];
        let mut tmp = [[0; TRITS_PER_TRYTE]; 27];
        tmp.iter_mut().for_each(|tmp_entry| {
            tmp_entry.copy_from_slice(&trits[0..TRITS_PER_TRYTE]);
            increment(&mut trits, TRITS_PER_TRYTE);
        });
        tmp
    };
}

/// Trait used to enable conversion to trinary types
pub trait Trinary {
    /// Provides the trit vector representation of the value
    fn trits(&self) -> Vec<Trit>;
    /// Provides the trit vector representation of the value with given length, padding with `0` if required
    fn trits_with_length(&self, length: usize) -> Vec<Trit>;
    /// Provides the tryte string representation of the value
    fn trytes(&self) -> Result<Trytes>;
}

/// Type alias for `i8`
pub type Trit = i8;
/// Type alias for `String`
pub type Trytes = String;

impl Trinary for i64 {
    fn trits(&self) -> Vec<Trit> {
        let mut trits = Vec::new();
        let mut abs = self.abs();
        while abs > 0 {
            let mut remainder = (abs % i64::from(TRINARY_RADIX as i8)) as i8;
            abs /= i64::from(TRINARY_RADIX as i8);
            if remainder > iota_constants::MAX_TRIT_VALUE {
                remainder = iota_constants::MIN_TRIT_VALUE;
                abs += 1;
            }
            trits.push(remainder);
        }
        if *self < 0 {
            trits.iter_mut().for_each(|trit| *trit = -*trit);
        }
        trits
    }

    fn trits_with_length(&self, length: usize) -> Vec<Trit> {
        trits_with_length(&self.trits(), length)
    }

    fn trytes(&self) -> Result<Trytes> {
        self.trits().trytes()
    }
}

impl Trinary for Vec<Trit> {
    fn trits(&self) -> Vec<Trit> {
        self.to_vec()
    }
    fn trits_with_length(&self, length: usize) -> Vec<Trit> {
        trits_with_length(&self.trits(), length)
    }
    fn trytes(&self) -> Result<Trytes> {
        trytes(self)
    }
}

impl Trinary for &[Trit] {
    fn trits(&self) -> Vec<Trit> {
        self.to_vec()
    }
    fn trits_with_length(&self, length: usize) -> Vec<Trit> {
        trits_with_length(&self.trits(), length)
    }
    fn trytes(&self) -> Result<Trytes> {
        trytes(self)
    }
}

impl Trinary for [Trit; 243] {
    fn trits(&self) -> Vec<Trit> {
        self.to_vec()
    }
    fn trits_with_length(&self, length: usize) -> Vec<Trit> {
        trits_with_length(&self.trits(), length)
    }
    fn trytes(&self) -> Result<Trytes> {
        ensure!(self.len() % 3 == 0, "Invalid trit length.");

        self.chunks(iota_constants::TRITS_PER_TRYTE)
            .map(trits_to_char)
            .collect()
    }
}

impl Trinary for Trytes {
    fn trits(&self) -> Vec<Trit> {
        self.chars().flat_map(char_to_trits).cloned().collect()
    }
    fn trits_with_length(&self, length: usize) -> Vec<Trit> {
        trits_with_length(&self.trits(), length)
    }
    fn trytes(&self) -> Result<Trytes> {
        Ok(self.clone())
    }
}

impl Trinary for &str {
    fn trits(&self) -> Vec<Trit> {
        self.chars().flat_map(char_to_trits).cloned().collect()
    }
    fn trits_with_length(&self, length: usize) -> Vec<Trit> {
        trits_with_length(&self.trits(), length)
    }
    fn trytes(&self) -> Result<Trytes> {
        Ok(self.to_string())
    }
}

/// Increments a trit slice in place, only considering trits until index `size`
fn increment(trit_array: &mut [Trit], size: usize) {
    for trit in trit_array.iter_mut().take(size) {
        *trit += 1;
        if *trit > iota_constants::MAX_TRIT_VALUE {
            *trit = iota_constants::MIN_TRIT_VALUE;
        } else {
            break;
        }
    }
}

fn char_to_trits(tryte: char) -> &'static [Trit] {
    match iota_constants::TRYTE_ALPHABET
        .iter()
        .position(|&x| x == tryte)
    {
        Some(p) => &TRYTE_TO_TRITS_MAPPINGS[p],
        None => &TRYTE_TO_TRITS_MAPPINGS[0],
    }
}

fn trits_to_char(trits: &[Trit]) -> Result<char> {
    ensure!(
        trits.len() <= iota_constants::TRITS_PER_TRYTE,
        "Provided trit slice is too long: {:?}",
        trits
    );
    Ok(
        match TRYTE_TO_TRITS_MAPPINGS.iter().position(|&x| x == trits) {
            Some(p) => iota_constants::TRYTE_ALPHABET[p],
            None => '-',
        },
    )
}

fn trytes(trits: &[Trit]) -> Result<Trytes> {
    ensure!(trits.len() % 3 == 0, "Invalid trit length.");

    trits
        .chunks(iota_constants::TRITS_PER_TRYTE)
        .map(trits_to_char)
        .collect()
}

fn trits_with_length(trits: &[Trit], length: usize) -> Vec<Trit> {
    if trits.len() < length {
        let mut result = vec![0; length];
        result[..trits.len()].copy_from_slice(&trits);
        result
    } else {
        trits[..length].to_vec()
    }
}
