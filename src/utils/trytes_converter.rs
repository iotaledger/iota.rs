use super::constants;
use failure::Error;
use std::collections::HashMap;

lazy_static! {
    static ref CHAR_TO_ASCII_MAP: HashMap<char, usize> = {
        let mut res: HashMap<char, usize> = HashMap::new();
        let mut ascii = 65;
        for c in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            res.insert(c, ascii);
            ascii += 1;
        }
        ascii = 97;
        for c in "abcdefghijklmnopqrstuvwxyz".chars() {
            res.insert(c, ascii);
            ascii += 1;
        }
        ascii = 48;
         for c in "0123456789".chars() {
            res.insert(c, ascii);
            ascii += 1;
        }
        res.insert(' ', 32);
        res
    };

    static ref ASCII_TO_CHAR_MAP: HashMap<usize, char> = {
        let mut res: HashMap<usize, char> = HashMap::new();
        for (key, val) in CHAR_TO_ASCII_MAP.iter() {
            res.insert(*val, *key);
        }
        res
    };
}

#[derive(Debug, Fail)]
enum TryteConverterError {
    #[fail(display = "String [{}] is not valid ascii", string)]
    StringNotAscii { string: String },
}

pub fn to_trytes(input: &str) -> Result<String, Error> {
    let mut trytes = String::new();
    let mut tmp_ascii = Vec::new();
    for c in input.chars() {
        if let Some(ascii) = CHAR_TO_ASCII_MAP.get(&c) {
            tmp_ascii.push(ascii);
        }
    }
    for byte in tmp_ascii {
        let mut ascii = *byte;
        if ascii > 255 {
            ascii = 32;
        }
        let first = ascii % 27;
        let second = (ascii - first) / 27;
        trytes.push(constants::TRYTE_ALPHABET[first]);
        trytes.push(constants::TRYTE_ALPHABET[second]);
    }
    Ok(trytes)
}

pub fn to_string(input_trytes: &str) -> Result<String, Error> {
    ensure!(input_trytes.len() % 2 == 0, "Invalid input tryte length.");
    let mut tmp = String::new();
    let chars: Vec<char> = input_trytes.chars().collect();
    for letters in chars.chunks(2) {
        let first = match constants::TRYTE_ALPHABET
            .iter()
            .position(|&x| x == letters[0])
        {
            Some(x) => x,
            None => {
                return Err(Error::from(TryteConverterError::StringNotAscii {
                    string: input_trytes.to_string(),
                }))
            }
        };
        let second = match constants::TRYTE_ALPHABET
            .iter()
            .position(|&x| x == letters[1])
        {
            Some(x) => x,
            None => {
                return Err(Error::from(TryteConverterError::StringNotAscii {
                    string: input_trytes.to_string(),
                }))
            }
        };
        let decimal = first + second * 27;
        if let Some(t) = ASCII_TO_CHAR_MAP.get(&decimal) {
            tmp.push(*t);
        }
    }
    Ok(tmp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Alphanumeric;
    use rand::{self, Rng};

    #[test]
    fn should_convert_string_to_trytes() {
        assert_eq!(to_trytes("Z").unwrap(), "IC");
        assert_eq!(to_trytes("JOTA JOTA").unwrap(), "TBYBCCKBEATBYBCCKB");
    }

    #[test]
    fn should_convert_trytes_to_string() {
        assert_eq!(to_string("IC").unwrap(), "Z");
        assert_eq!(to_string("TBYBCCKBEATBYBCCKB").unwrap(), "JOTA JOTA");
    }

    #[test]
    fn should_convert_back_and_forth() {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(1000)
            .collect();
        let trytes = to_trytes(&s).unwrap();
        let back = to_string(&trytes).unwrap();
        assert_eq!(s, back);
    }
}
