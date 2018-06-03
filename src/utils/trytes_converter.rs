use ascii::{AsAsciiStrError, AsciiChar, AsciiStr};
use errors::*;
use utils::constants;

pub fn to_trytes(input: &str) -> Result<String> {
    let mut trytes = String::new();
    let tmp =
        AsciiStr::from_ascii(input).chain_err(|| "unable to convert from UTF string to ascii")?;
    for byte in tmp.chars() {
        let mut ascii = byte.as_byte() as usize;
        if ascii > 255 {
            ascii = 32;
        }
        let first = ascii % 27;
        let second = (ascii - first) / 27;
        trytes.push(
            constants::TRYTE_ALPHABET_ASCII
                .chars()
                .nth(first)
                .chain_err(|| "")?
                .as_char(),
        );
        trytes.push(
            constants::TRYTE_ALPHABET_ASCII
                .chars()
                .nth(second)
                .chain_err(|| "")?
                .as_char(),
        );
    }
    Ok(trytes)
}

pub fn to_string(input: &str) -> Result<String> {
    if input.len() % 2 != 0 {
        return Ok(String::new());
    }
    let t = AsciiStr::from_ascii(input).chain_err(|| "")?;
    let mut tmp = String::new();
    let mut i = 0;
    while i < input.len() {
        let mut first: i32 = -1;
        for (x, item) in constants::TRYTE_ALPHABET_ASCII.chars().enumerate() {
            if item.as_byte() == t.chars().nth(i).chain_err(|| "")?.as_byte() {
                first = x as i32;
            }
        }
        let mut second: i32 = -1;
        for (x, item) in constants::TRYTE_ALPHABET_ASCII.chars().enumerate() {
            if item.as_byte() == t.chars().nth(i + 1).chain_err(|| "")?.as_byte() {
                second = x as i32;
            }
        }
        let decimal = (first + second * 27) as u8;
        tmp.push(AsciiChar::from(decimal).chain_err(|| "")?.as_char());
        i = i + 2;
    }
    Ok(tmp)
}
