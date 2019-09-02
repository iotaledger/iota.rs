use crate::client::Result;
use iota_conversion::Trinary;

/// GetNewAddressOptions
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GetNewAddressOptions {
    /// Security factor 1-3 with 3 being most secure
    pub security: Option<usize>,
    /// How many iterations of generating to skip
    pub index: Option<usize>,
    /// Number of addresses to generate. If total isn't provided, we generate until we find an unused address
    pub total: Option<usize>,
}

/// Generate new address from given seed
///
/// * `seed` - Seed used to generate new address
/// * `security` - Security factor 1-3 with 3 being most secure
/// * `index` - How many iterations of generating to skip
/// * `checksum` - Whether or not to checksum address
pub fn new_address(seed: &str, security: usize, index: usize, checksum: bool) -> Result<String> {
    let key = iota_signing::key(&seed.trits(), index, security)?;
    let digests = iota_signing::digests(&key)?;
    let address_trits = iota_signing::address(&digests)?;
    let mut address = address_trits.trytes()?;
    if checksum {
        address = iota_signing::checksum::add_checksum(&address)?;
    }
    Ok(address)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SEED: &str =
        "IHDEENZYITYVYSPKAURUZAQKGVJEREFDJMYTANNXXGPZ9GJWTEOJJ9IPMXOGZNQLSNMFDSQOTZAEETUEA";
    const ADDR_SEED: &str =
        "LIESNFZLPFNWAPWXBLKEABZEEWUDCXKTRKZIRTPCKLKWOMJSEREWKMMMODUOFWM9ELEVXADTSQWMSNFVD";

    #[test]
    fn test_address_generation() {
        assert_eq!(new_address(&TEST_SEED, 2, 0, true).unwrap(), "LXQHWNY9CQOHPNMKFJFIJHGEPAENAOVFRDIBF99PPHDTWJDCGHLYETXT9NPUVSNKT9XDTDYNJKJCPQMZCCOZVXMTXC");
        assert_eq!(new_address(&TEST_SEED, 2, 5, true).unwrap(), "HLHRSJNPUUGRYOVYPSTEQJKETXNXDIWQURLTYDBJADGIYZCFXZTTFSOCECPPPPY9BYWPODZOCWJKXEWXDPUYEOTFQA");

        assert_eq!(
            new_address(&ADDR_SEED, 1, 0, false).unwrap(),
            "HIPPOUPZFMHJUQBLBVWORCNJWAOSFLHDWF9IOFEYVHPTTAAF9NIBMRKBICAPHYCDKMEEOXOYHJBMONJ9D"
        );
        assert_eq!(
            new_address(&ADDR_SEED, 2, 0, false).unwrap(),
            "BPYZABTUMEIOARZTMCDNUDAPUOFCGKNGJWUGUXUKNNBVKQARCZIXFVBZAAMDAFRS9YOIXWOTEUNSXVOG9"
        );
        assert_eq!(
            new_address(&ADDR_SEED, 3, 0, false).unwrap(),
            "BYWHJJYSHSEGVZKKYTJTYILLEYBSIDLSPXDLDZSWQ9XTTRLOSCBCQ9TKXJYQAVASYCMUCWXZHJYRGDOBW"
        );

        let concat = ADDR_SEED.to_string() + &ADDR_SEED;
        assert_eq!(
            new_address(&concat, 1, 0, false).unwrap(),
            "VKPCVHWKSCYQNHULMPYDZTNKOQHZNPEGJVPEHPTDIUYUBFKFICDRLLSIULHCVHOHZRHJOHNASOFRWFWZC"
        );
        assert_eq!(
            new_address(&concat, 2, 0, false).unwrap(),
            "PTHVACKMXOKIERJOFSRPBWCNKVEXQ9CWUTIJGEUORSKWEDDJCBFQCCBQZLTYXQCXEDWLTMRQM9OQPUGNC"
        );
        assert_eq!(
            new_address(&concat, 3, 0, false).unwrap(),
            "AGSAAETPMSBCDOSNXFXIOBAE9MVEJCSWVP9PAULQ9VABOTWLDMXID9MXCCWQIWRTJBASWPIJDFUC9ISWD"
        );
    }
}
