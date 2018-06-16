use model::bundle::{self, Bundle};
use model::input::Input;
use utils::{checksum, converter, signing};

pub fn new_address(seed: &str, security: usize, index: usize, checksum: bool) -> String {
    let key = signing::key(&converter::trits_from_string(seed), index, security);
    let mut digests = signing::digests(&key);
    let address_trits = signing::address(&mut digests);

    let mut address = converter::trytes(&address_trits);

    if checksum {
        address = checksum::add_checksum(&address);
    }
    address
}

pub fn sign_inputs_and_return(
    seed: &str,
    inputs: &[Input],
    bundle: &mut Bundle,
    signature_fragments: &[String],
) -> Vec<String> {
    bundle.finalize();
    bundle.add_trytes(signature_fragments);

    for i in 0..bundle.transactions().len() {
        //TODO this is always false.. not sure what they're doing here
        if bundle.transactions()[i].value().unwrap() < 0 {
            let this_address = bundle.transactions()[i].address().clone().unwrap();
            let mut key_index = 0;
            let mut key_security = 0;
            for input in inputs {
                if input.address() == this_address {
                    key_index = input.key_index();
                    key_security = input.security();
                }
            }
            let bundle_hash = bundle.transactions()[i].bundle().clone().unwrap();

            let key = signing::key(
                &converter::trits_from_string(&seed),
                key_index,
                key_security,
            );
            let first_fragment = key[0..6561].to_vec();
            let normalized_bundle_hash = bundle::normalized_bundle(&bundle_hash);
            let first_bundle_fragment = normalized_bundle_hash[0..27].to_vec();
            let first_signed_fragment =
                signing::signature_fragment(&first_bundle_fragment, &first_fragment);

            *bundle.transactions_mut()[i].signature_fragments_mut() =
                Some(converter::trytes(&first_signed_fragment));

            for j in 1..key_security {
                if bundle.transactions()[i + j].address().clone().unwrap() == this_address
                    && bundle.transactions()[i + j].value().unwrap() == 0
                {
                    let second_fragment = key[6561 * j..6561 * (j + 1)].to_vec();
                    let second_bundle_fragment =
                        normalized_bundle_hash[27 * j..27 * (j + 1)].to_vec();
                    let second_signed_fragment =
                        signing::signature_fragment(&second_bundle_fragment, &second_fragment);
                    *bundle.transactions_mut()[i + j].signature_fragments_mut() =
                        Some(converter::trytes(&second_signed_fragment));
                }
            }
        }
    }
    let mut bundle_trytes: Vec<String> = Vec::new();
    for tx in bundle.transactions() {
        bundle_trytes.push(tx.to_trytes());
    }
    bundle_trytes.reverse();
    bundle_trytes
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::api_utils;
    const TEST_SEED: &str =
        "IHDEENZYITYVYSPKAURUZAQKGVJEREFDJMYTANNXXGPZ9GJWTEOJJ9IPMXOGZNQLSNMFDSQOTZAEETUEA";
    const ADDR_SEED: &str =
        "LIESNFZLPFNWAPWXBLKEABZEEWUDCXKTRKZIRTPCKLKWOMJSEREWKMMMODUOFWM9ELEVXADTSQWMSNFVD";

    #[test]
    fn test_address_generation() {
        assert_eq!(api_utils::new_address(TEST_SEED, 2, 0, true), "LXQHWNY9CQOHPNMKFJFIJHGEPAENAOVFRDIBF99PPHDTWJDCGHLYETXT9NPUVSNKT9XDTDYNJKJCPQMZCCOZVXMTXC");
        assert_eq!(api_utils::new_address(TEST_SEED, 2, 5, true), "HLHRSJNPUUGRYOVYPSTEQJKETXNXDIWQURLTYDBJADGIYZCFXZTTFSOCECPPPPY9BYWPODZOCWJKXEWXDPUYEOTFQA");

        assert_eq!(
            api_utils::new_address(ADDR_SEED, 1, 0, false),
            "HIPPOUPZFMHJUQBLBVWORCNJWAOSFLHDWF9IOFEYVHPTTAAF9NIBMRKBICAPHYCDKMEEOXOYHJBMONJ9D"
        );
        assert_eq!(
            api_utils::new_address(ADDR_SEED, 2, 0, false),
            "BPYZABTUMEIOARZTMCDNUDAPUOFCGKNGJWUGUXUKNNBVKQARCZIXFVBZAAMDAFRS9YOIXWOTEUNSXVOG9"
        );
        assert_eq!(
            api_utils::new_address(ADDR_SEED, 3, 0, false),
            "BYWHJJYSHSEGVZKKYTJTYILLEYBSIDLSPXDLDZSWQ9XTTRLOSCBCQ9TKXJYQAVASYCMUCWXZHJYRGDOBW"
        );

        let concat = ADDR_SEED.to_string() + ADDR_SEED;
        assert_eq!(
            api_utils::new_address(&concat, 1, 0, false),
            "VKPCVHWKSCYQNHULMPYDZTNKOQHZNPEGJVPEHPTDIUYUBFKFICDRLLSIULHCVHOHZRHJOHNASOFRWFWZC"
        );
        assert_eq!(
            api_utils::new_address(&concat, 2, 0, false),
            "PTHVACKMXOKIERJOFSRPBWCNKVEXQ9CWUTIJGEUORSKWEDDJCBFQCCBQZLTYXQCXEDWLTMRQM9OQPUGNC"
        );
        assert_eq!(
            api_utils::new_address(&concat, 3, 0, false),
            "AGSAAETPMSBCDOSNXFXIOBAE9MVEJCSWVP9PAULQ9VABOTWLDMXID9MXCCWQIWRTJBASWPIJDFUC9ISWD"
        );
    }
}
