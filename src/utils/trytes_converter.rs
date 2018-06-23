use ascii::{AsciiChar, AsciiStr};
use failure::Error;
use super::constants;

#[derive(Debug, Fail)]
enum TryteConverterError {
    #[fail(display = "String [{}] is not valid ascii", string)]
    StringNotAscii { string: String },
}

pub fn to_trytes(input: &str) -> Result<String, Error> {
    let mut trytes = String::new();
    let tmp = AsciiStr::from_ascii(input)?;
    for byte in tmp.chars() {
        let mut ascii = byte.as_byte() as usize;
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
    if input_trytes.len() % 2 != 0 {
        return Ok(String::new());
    }
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
        tmp.push(AsciiChar::from(decimal as u8)?.as_char());
    }
    Ok(tmp)
}

#[cfg(test)]
mod tests {
    use rand::distributions::Alphanumeric;
    use rand::{self, Rng};
    use super::*;

    #[test]
    fn should_convert_string_to_trytes() {
        assert_eq!(to_trytes("Z").unwrap(), "IC");
        assert_eq!(
            to_trytes("JOTA JOTA").unwrap(),
            "TBYBCCKBEATBYBCCKB"
        );
    }

    #[test]
    fn should_convert_trytes_to_string() {
        assert_eq!(to_string("IC").unwrap(), "Z");
        assert_eq!(
            to_string("TBYBCCKBEATBYBCCKB").unwrap(),
            "JOTA JOTA"
        );
    }

    #[test]
    fn should_convert_back_and_forth() {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(1000)
            .collect();
        let back = to_string(&to_trytes(&s).unwrap()).unwrap();
        assert_eq!(s, back);
    }
}
