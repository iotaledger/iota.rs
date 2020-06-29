use std::convert::TryInto;
use std::error;
use std::fmt;
use std::str;

use iota_constants;

/// Converts a sequence of bytes to trytes
pub fn bytes_to_trytes(input: &[u8]) -> String {
    let mut trytes = String::with_capacity(input.len() * 2);
    for byte in input {
        let first = byte % 27;
        let second = byte / 27;

        trytes.push(iota_constants::TRYTE_ALPHABET[first as usize]);
        trytes.push(iota_constants::TRYTE_ALPHABET[second as usize]);
    }
    trytes
}

#[derive(Debug)]
/// Dummy error for the string to trytes conversion
pub struct ToTrytesError {}
impl error::Error for ToTrytesError {}
impl fmt::Display for ToTrytesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Trytes could not be converted to string")
    }
}

/// Converts a UTF-8 string containing ascii into a tryte-encoded string
pub fn to_trytes(input: &str) -> Result<String, ToTrytesError> {
    Ok(bytes_to_trytes(input.as_bytes()))
}

/// Errors that can be caused when converting trytes to bytes
#[derive(Debug)]
pub enum TrytesToBytesError {
    /// Since two trytes can encode up to 27*27 values, not everey pair of trytes is valid for conversion
    InvalidTrytePair,
    /// A character was not a valid tryte
    StringNotTrytes(char),
}
impl std::error::Error for TrytesToBytesError {}
impl std::fmt::Display for TrytesToBytesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TrytesToBytesError::*;
        match self {
            InvalidTrytePair => write!(f, "Got an invalid tryte pair with value > 255"),
            StringNotTrytes(c) => write!(
                f,
                "The input string did not consist of trytes (Invalid char: {})",
                c
            ),
        }
    }
}

/// Convert a single character tryte to its numerical equivalent
fn tryte_to_val(tryte: char) -> std::result::Result<usize, TrytesToBytesError> {
    iota_constants::TRYTE_ALPHABET
        .iter()
        .position(|&x| x == tryte)
        .map(|x| x as usize)
        .ok_or_else(|| TrytesToBytesError::StringNotTrytes(tryte))
}

/// Converts a sequence of trytes into bytes
pub fn trytes_to_bytes(input: &str, buf: &mut [u8]) -> std::result::Result<(), TrytesToBytesError> {
    // We have to add one here because the input may contain an uneven amount of trytes
    assert!(
        buf.len() * 2 + 1 >= input.len(),
        "Buffer to short (buffer length: {}, number of trytes: {})",
        buf.len(),
        input.len()
    );
    for (idx, pair) in input
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .filter(|pair| pair.len() == 2)
        .enumerate()
    {
        let first = tryte_to_val(pair[0])?;
        let second = tryte_to_val(pair[1])?;

        let decimal = first + second * 27;
        buf[idx] = decimal
            .try_into()
            .map_err(|_| TrytesToBytesError::InvalidTrytePair)?;
    }
    Ok(())
}

#[derive(Debug)]
/// Indicates an error during conversion from trytes to utf-8 string
pub enum TrytesToStringError {
    /// The tryte string had an invalid format
    TryteFormatError(TrytesToBytesError),
    /// The encoded bytes are not valid utf-8
    Utf8Error(str::Utf8Error),
}
impl error::Error for TrytesToStringError {}
impl fmt::Display for TrytesToStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TrytesToStringError::*;
        match self {
            TryteFormatError(e) => {
                write!(f, "Trytes could not be converted to bytes. Cause:\n{}", e)
            }
            Utf8Error(e) => write!(f, "Bytes contained invalid UTF-8 data:\n{}", e),
        }
    }
}
impl From<str::Utf8Error> for TrytesToStringError {
    fn from(err: str::Utf8Error) -> Self {
        TrytesToStringError::Utf8Error(err)
    }
}
impl From<TrytesToBytesError> for TrytesToStringError {
    fn from(err: TrytesToBytesError) -> Self {
        TrytesToStringError::TryteFormatError(err)
    }
}

/// Converts a tryte-encoded string into a UTF-8 string
pub fn to_string(input_trytes: &str) -> Result<String, TrytesToStringError> {
    let mut buf = vec![0 as u8; input_trytes.len() / 2];
    trytes_to_bytes(input_trytes, buf.as_mut_slice())?;
    Ok(str::from_utf8(buf.as_slice())?.to_string())
}

#[cfg(test)]
mod tests {
    use rand::distributions::Alphanumeric;
    use rand::{self, Rng};

    use super::*;

    #[test]
    fn should_convert_string_to_trytes() {
        assert_eq!(to_trytes("Z").unwrap(), "IC");
        assert_eq!(to_trytes("\n").unwrap(), "J9");
        assert_eq!(to_trytes("JOTA JOTA").unwrap(), "TBYBCCKBEATBYBCCKB");
        assert_eq!(to_trytes(" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~").unwrap(), "EAFAGAHAIAJAKALAMANAOAPAQARASATAUAVAWAXAYAZA9BABBBCBDBEBFBGBHBIBJBKBLBMBNBOBPBQBRBSBTBUBVBWBXBYBZB9CACBCCCDCECFCGCHCICJCKCLCMCNCOCPCQCRCSCTCUCVCWCXCYCZC9DADBDCDDDEDFDGDHDIDJDKDLDMDNDODPDQDRD");
    }

    #[test]
    fn should_convert_trytes_to_string() {
        assert_eq!(to_string("IC").unwrap(), "Z");
        assert_eq!(to_string("J9").unwrap(), "\n");
        assert_eq!(to_string("TBYBCCKBEATBYBCCKB").unwrap(), "JOTA JOTA");
        assert_eq!(to_string("TBYBCCKBEATBYBCCKB9").unwrap(), "JOTA JOTA");
        assert_eq!(to_string("EAFAGAHAIAJAKALAMANAOAPAQARASATAUAVAWAXAYAZA9BABBBCBDBEBFBGBHBIBJBKBLBMBNBOBPBQBRBSBTBUBVBWBXBYBZB9CACBCCCDCECFCGCHCICJCKCLCMCNCOCPCQCRCSCTCUCVCWCXCYCZC9DADBDCDDDEDFDGDHDIDJDKDLDMDNDODPDQDRD").unwrap(), " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~");
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

    #[test]
    fn should_convert_back_and_forth_utf8() {
        let s = "(┛ಠ_ಠ)┛彡┻━┻";
        let trytes = to_trytes(s).unwrap();
        let back = to_string(&trytes).unwrap();
        assert_eq!(s, back);
    }
}
