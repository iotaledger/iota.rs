use crate::client::Client;
use crate::options::FindTransactionsOptions;
use crate::Result;
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

impl<'a> Client<'a> {
    /// Generates a new address
    ///
    /// * `seed` - Seed used to generate new address
    /// * `checksum` - Whether or not to checksum address
    /// * `return_all` - Whether to return all generated addresses, or just the last one
    /// * `options` - See `GetNewAddressOptions`
    pub async fn get_new_address(
        &mut self,
        seed: &str,
        checksum: bool,
        return_all: bool,
        options: GetNewAddressOptions,
    ) -> Result<Vec<String>> {
        let mut index = options.index.unwrap_or_default();
        let security = options.security.unwrap_or(2);
        ensure!(iota_validation::is_trytes(&seed), "Invalid seed.");
        ensure!(security > 0 && security < 4, "Invalid security.");

        let mut all_addresses: Vec<String> = Vec::new();

        match options.total {
            Some(total) => {
                ensure!(total > 0, "Invalid total.");
                for i in index..total {
                    let address = new_address(&seed, security, i, checksum)?;
                    all_addresses.push(address);
                }
                Ok(all_addresses)
            }
            None => loop {
                let new_address = new_address(&seed, security, index, checksum)?;
                if return_all {
                    all_addresses.push(new_address.clone());
                }
                index += 1;
                let new_address_vec = vec![new_address];
                let were_addr_spent = self.were_addresses_spent_from(&new_address_vec).await?;
                if !were_addr_spent.state(0) {
                    let resp = self
                        .find_transactions(FindTransactionsOptions {
                            addresses: new_address_vec.clone(),
                            ..FindTransactionsOptions::default()
                        })
                        .await?;
                    if resp.take_hashes().unwrap_or_default().is_empty() {
                        if return_all {
                            return Ok(all_addresses);
                        } else {
                            return Ok(new_address_vec);
                        }
                    }
                }
            },
        }
    }
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
