use ascii::{AsciiChar, AsciiStr};
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
        trytes.push(constants::TRYTE_ALPHABET[first]);
        trytes.push(constants::TRYTE_ALPHABET[second]);
    }
    Ok(trytes)
}

pub fn to_string(input: &str) -> Result<String> {
    if input.len() % 2 != 0 {
        return Ok(String::new());
    }
    let t = AsciiStr::from_ascii(input).chain_err(|| "unable to convert from UTF string to ascii")?;
    let mut tmp = String::new();
    let mut i = 0;
    while i < input.len() {
        let mut first: i32 = -1;
        for (x, item) in constants::TRYTE_ALPHABET.iter().enumerate() {
            if item == t
                .chars()
                .nth(i)
                .chain_err(|| format!("Couldn't retrieve [{}] value in {}", i, t))?
            {
                first = x as i32;
            }
        }
        let mut second: i32 = -1;
        for (x, item) in constants::TRYTE_ALPHABET.iter().enumerate() {
            if item == t
                .chars()
                .nth(i + 1)
                .chain_err(|| format!("Couldn't retrieve [{}] value in {}", i, t))?
            {
                second = x as i32;
            }
        }
        let decimal = (first + second * 27) as u8;
        tmp.push(
            AsciiChar::from(decimal)
                .chain_err(|| "Couldn't convert from u8 to ascii char")?
                .as_char(),
        );
        i += 2;
    }
    Ok(tmp)
}

#[cfg(test)]
mod tests {
    use rand::distributions::Alphanumeric;
    use rand::{self, Rng};
    use utils::trytes_converter;

    #[test]
    fn should_convert_string_to_trytes() {
        assert_eq!(trytes_converter::to_trytes("Z").unwrap(), "IC");
        assert_eq!(
            trytes_converter::to_trytes("JOTA JOTA").unwrap(),
            "TBYBCCKBEATBYBCCKB"
        );
    }

    #[test]
    fn should_convert_trytes_to_string() {
        assert_eq!(trytes_converter::to_string("IC").unwrap(), "Z");
        assert_eq!(
            trytes_converter::to_string("TBYBCCKBEATBYBCCKB").unwrap(),
            "JOTA JOTA"
        );
    }

    #[test]
    fn should_convert_back_and_forth() {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(1000)
            .collect();
        let back = trytes_converter::to_string(&trytes_converter::to_trytes(&s).unwrap()).unwrap();
        assert_eq!(s, back);
    }
}
