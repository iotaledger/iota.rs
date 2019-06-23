use regex::Regex;

use iota_constants;
use iota_model::*;

lazy_static! {
    static ref TRYTE_REGEX: Regex = Regex::new("^[A-Z9]*$").expect("Failed to parse regex");
    static ref NINE_TRYTE_REGEX: Regex = Regex::new("^[9]*$").expect("Failed to parse regex");
}

/// Validates that the provided string is an address
pub fn is_address(address: &str) -> bool {
    address.len() == iota_constants::ADDRESS_LENGTH_WITHOUT_CHECKSUM
        || address.len() == iota_constants::ADDRESS_LENGTH_WITH_CHECKSUM && is_trytes(address)
}

/// Validates that a slice of strings are all addresses
pub fn is_addresses_collection_valid(addresses: &[String]) -> bool {
    for address in addresses {
        if !is_address(&address) {
            return false;
        }
    }
    true
}

/// Validates that a string contains only tryte characters
pub fn is_trytes(trytes: &str) -> bool {
    TRYTE_REGEX.is_match(trytes)
}

/// Validates that a string contains only the number 9
pub fn is_nine_trytes(trytes: &str) -> bool {
    NINE_TRYTE_REGEX.is_match(trytes)
}

/// Validates that a string contains only tryte characters
pub fn is_trytes_with_length(trytes: &str, len: usize) -> bool {
    trytes.len() == len && TRYTE_REGEX.is_match(trytes)
}

/// Validates that a string is an integer
pub fn is_value(value: &str) -> bool {
    match value.parse::<i64>() {
        Ok(_val) => true,
        Err(_e) => match value.parse::<u64>() {
            Ok(_val) => true,
            Err(_e) => false,
        },
    }
}

/// Validates that a slice of strings are all valid trytes
pub fn is_array_of_trytes(trytes: &[String]) -> bool {
    for tryte in trytes {
        if !is_trytes(&tryte) {
            return false;
        }
    }
    true
}

/// Validates that a slice of strings are all valid hashes
pub fn is_array_of_hashes(hashes: &[String]) -> bool {
    for hash in hashes {
        if hash.len() == 90 {
            if !is_trytes(&hash[0..90]) {
                return false;
            }
        } else if !is_trytes(&hash[0..81]) {
            return false;
        }
    }
    true
}

/// Validates a transfer
pub fn is_valid_transfer(transfer: &Transfer) -> bool {
    is_address(&transfer.address) && is_trytes(&transfer.message) && is_trytes(&transfer.tag)
}

/// Validates a slice of transfers
pub fn is_transfers_collection_valid(transfers: &[Transfer]) -> bool {
    if transfers.is_empty() {
        return false;
    }
    for transfer in transfers {
        if !is_valid_transfer(&transfer) {
            return false;
        }
    }
    true
}

/// Validates a slice of transactions
pub fn is_slice_of_transactions(bundle: &[Transaction]) -> bool {
    if bundle.is_empty() {
        return false;
    }

    let mut valid = true;
    for tx in bundle {
        if tx.hash == "" {
            return false;
        }
        valid &= is_hash(&tx.hash);
        if tx.signature_fragments == "" {
            return false;
        }
        valid &= is_trytes(&tx.signature_fragments);
        if tx.address == "" {
            return false;
        }
        valid &= is_hash(&tx.address);
        if tx.tag == "" {
            return false;
        }
        valid &= is_trytes(&tx.tag);
        if tx.obsolete_tag == "" {
            return false;
        }
        valid &= is_trytes(&tx.obsolete_tag);
        if tx.bundle == "" {
            return false;
        }
        valid &= is_hash(&tx.bundle);
        if tx.trunk_transaction == "" {
            return false;
        }
        valid &= is_hash(&tx.trunk_transaction);
        if tx.branch_transaction == "" {
            return false;
        }
        valid &= is_hash(&tx.branch_transaction);
        if tx.nonce == "" {
            return false;
        }
        valid &= is_trytes(&tx.nonce);
        if !valid {
            return false;
        }
    }
    valid
}

/// Validates that a string is a seed
pub fn is_valid_seed(seed: &str) -> bool {
    is_trytes(seed)
}

/// Validates that a string is a hash
pub fn is_hash(hash: &str) -> bool {
    is_trytes(&hash[0..81])
}

/// Validates that a slice of strings are all hash
pub fn is_hashes(hashes: &[String]) -> bool {
    for hash in hashes {
        if !is_trytes(&hash[0..81]) {
            return false;
        }
    }
    true
}

/// Validates that a slice of strings contains only attached trytes
pub fn is_array_of_attached_trytes(trytes: &[String]) -> bool {
    for tryte_value in trytes {
        if !is_trytes(&tryte_value[0..2673]) {
            return false;
        }
        if is_nine_trytes(&tryte_value[2673 - (3 * 81)..]) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ADDRESS_WITHOUT_CHECKSUM: &str = "PNGMCSNRCTRHCHPXYTPKEJYPCOWKOMRXZFHH9N9VDIKMNVAZCMIYRHVJIAZARZTUETJVFDMBEBIQE9QTHBFWDAOEFA";
    const TEST_ADDRESS_WITH_CHECKSUM: &str = "PNGMCSNRCTRHCHPXYTPKEJYPCOWKOMRXZFHH9N9VDIKMNVAZCMIYRHVJIAZARZTUETJVFDMBEBIQE9QTHBFWDAOEFA";
    const TEST_TRYTES: &str = "BYSWEAUTWXHXZ9YBZISEK9LUHWGMHXCGEVNZHRLUWQFCUSDXZHOFHWHL9MQPVJXXZLIXPXPXF9KYEREFSKCPKYIIKPZVLHUTDFQKKVVBBN9ATTLPCNPJDWDEVIYYLGPZGCWXOBDXMLJC9VO9QXTTBLAXTTBFUAROYEGQIVB9MJWJKXJMCUPTWAUGFZBTZCSJVRBGMYXTVBDDS9MYUJCPZ9YDWWQNIPUAIJXXSNLKUBSCOIJPCLEFPOXFJREXQCUVUMKSDOVQGGHRNILCO9GNCLWFM9APMNMWYASHXQAYBEXF9QRIHIBHYEJOYHRQJAOKAQ9AJJFQ9WEIWIJOTZATIBOXQLBMIJU9PCGBLVDDVFP9CFFSXTDUXMEGOOFXWRTLFGV9XXMYWEMGQEEEDBTIJ9OJOXFAPFQXCDAXOUDMLVYRMRLUDBETOLRJQAEDDLNVIRQJUBZBO9CCFDHIX9MSQCWYAXJVWHCUPTRSXJDESISQPRKZAFKFRULCGVRSBLVFOPEYLEE99JD9SEBALQINPDAZHFAB9RNBH9AZWIJOTLBZVIEJIAYGMC9AZGNFWGRSWAXTYSXVROVNKCOQQIWGPNQZKHUNODGYADPYLZZZUQRTJRTODOUKAOITNOMWNGHJBBA99QUMBHRENGBHTH9KHUAOXBVIVDVYYZMSEYSJWIOGGXZVRGN999EEGQMCOYVJQRIRROMPCQBLDYIGQO9AMORPYFSSUGACOJXGAQSPDY9YWRRPESNXXBDQ9OZOXVIOMLGTSWAMKMTDRSPGJKGBXQIVNRJRFRYEZ9VJDLHIKPSKMYC9YEGHFDS9SGVDHRIXBEMLFIINOHVPXIFAZCJKBHVMQZEVWCOSNWQRDYWVAIBLSCBGESJUIBWZECPUCAYAWMTQKRMCHONIPKJYYTEGZCJYCT9ABRWTJLRQXKMWY9GWZMHYZNWPXULNZAPVQLPMYQZCYNEPOCGOHBJUZLZDPIXVHLDMQYJUUBEDXXPXFLNRGIPWBRNQQZJSGSJTTYHIGGFAWJVXWL9THTPWOOHTNQWCNYOYZXALHAZXVMIZE9WMQUDCHDJMIBWKTYH9AC9AFOT9DPCADCV9ZWUTE9QNOMSZPTZDJLJZCJGHXUNBJFUBJWQUEZDMHXGBPTNSPZBR9TGSKVOHMOQSWPGFLSWNESFKSAZY9HHERAXALZCABFYPOVLAHMIHVDBGKUMDXC9WHHTIRYHZVWNXSVQUWCR9M9RAGMFEZZKZ9XEOQGOSLFQCHHOKLDSA9QCMDGCGMRYJZLBVIFOLBIJPROKMHOYTBTJIWUZWJMCTKCJKKTR9LCVYPVJI9AHGI9JOWMIWZAGMLDFJA9WU9QAMEFGABIBEZNNAL9OXSBFLOEHKDGHWFQSHMPLYFCNXAAZYJLMQDEYRGL9QKCEUEJ9LLVUOINVSZZQHCIKPAGMT9CAYIIMTTBCPKWTYHOJIIY9GYNPAJNUJ9BKYYXSV9JSPEXYMCFAIKTGNRSQGUNIYZCRT9FOWENSZQPD9ALUPYYAVICHVYELYFPUYDTWUSWNIYFXPX9MICCCOOZIWRNJIDALWGWRATGLJXNAYTNIZWQ9YTVDBOFZRKO9CFWRPAQQRXTPACOWCPRLYRYSJARRKSQPR9TCFXDVIXLP9XVL99ERRDSOHBFJDJQQGGGCZNDQ9NYCTQJWVZIAELCRBJJFDMCNZU9FIZRPGNURTXOCDSQGXTQHKHUECGWFUUYS9J9NYQ9U9P9UUP9YMZHWWWCIASCFLCMSKTELZWUGCDE9YOKVOVKTAYPHDF9ZCCQAYPJIJNGSHUIHHCOSSOOBUDOKE9CJZGYSSGNCQJVBEFTZFJ9SQUHOASKRRGBSHWKBCBWBTJHOGQ9WOMQFHWJVEG9NYX9KWBTCAIXNXHEBDIOFO9ALYMFGRICLCKKLG9FOBOX9PDWNQRGHBKHGKKRLWTBEQMCWQRLHAVYYZDIIPKVQTHYTWQMTOACXZOQCDTJTBAAUWXSGJF9PNQIJ9AJRUMUVCPWYVYVARKR9RKGOUHHNKNVGGPDDLGKPQNOYHNKAVVKCXWXOQPZNSLATUJT9AUWRMPPSWHSTTYDFAQDXOCYTZHOYYGAIM9CELMZ9AZPWB9MJXGHOKDNNSZVUDAGXTJJSSZCPZVPZBYNNTUQABSXQWZCHDQSLGK9UOHCFKBIBNETK999999999999999999999999999999999999999999999999999999999999999999999999999999999NOXDXXKUDWLOFJLIPQIBRBMGDYCPGDNLQOLQS99EQYKBIU9VHCJVIPFUYCQDNY9APGEVYLCENJIOBLWNB999999999XKBRHUD99C99999999NKZKEKWLDKMJCI9N9XQOLWEPAYWSH9999999999999999999999999KDDTGZLIPBNZKMLTOLOXQVNGLASESDQVPTXALEKRMIOHQLUHD9ELQDBQETS9QFGTYOYWLNTSKKMVJAUXSIROUICDOXKSYZTDPEDKOQENTJOWJONDEWROCEJIEWFWLUAACVSJFTMCHHXJBJRKAAPUDXXVXFWP9X9999IROUICDOXKSYZTDPEDKOQENTJOWJONDEWROCEJIEWFWLUAACVSJFTMCHHXJBJRKAAPUDXXVXFWP9X9999";
    const TEST_HASH: &str =
        "OAATQS9VQLSXCLDJVJJVYUGONXAXOFMJOZNSYWRZSWECMXAQQURHQBJNLD9IOFEPGZEPEMPXCIVRX9999";
    const TEST_MESSAGE: &str = "JOTA";
    const TEST_TAG: &str = "JOTASPAM9999999999999999999";

    #[test]
    fn test_is_address() {
        assert!(is_address(TEST_ADDRESS_WITHOUT_CHECKSUM))
    }

    #[test]
    fn test_is_trytes() {
        assert!(is_trytes(TEST_TRYTES))
    }

    #[test]
    fn test_is_value() {
        assert!(is_value("1234"));
    }

    #[test]
    fn test_is_array_of_hashes() {
        assert!(is_array_of_hashes(&vec![
            TEST_HASH.to_string(),
            TEST_HASH.to_string(),
        ]));
    }

    #[test]
    fn test_is_array_of_trytes() {
        assert!(is_array_of_trytes(&vec![
            TEST_TRYTES.to_string(),
            TEST_TRYTES.to_string(),
        ]));
    }

    #[test]
    fn test_is_nine_trytes() {
        assert!(is_nine_trytes("999999999"));
    }

    #[test]
    fn test_is_valid_transfer() {
        let mut t = Transfer::default();
        t.address = TEST_ADDRESS_WITH_CHECKSUM.to_string();
        t.value = 0;
        t.message = TEST_MESSAGE.to_string();
        t.tag = TEST_TAG.to_string();
        assert!(is_valid_transfer(&t));
    }

    #[test]
    fn test_is_transfers_collection_valid() {
        let mut t = Transfer::default();
        t.address = TEST_ADDRESS_WITH_CHECKSUM.to_string();
        t.value = 0;
        t.message = TEST_MESSAGE.to_string();
        t.tag = TEST_TAG.to_string();

        let mut t2 = Transfer::default();
        t2.address = TEST_ADDRESS_WITH_CHECKSUM.to_string();
        t2.value = 0;
        t2.message = "".to_string();

        let transfers = vec![t, t2];
        assert!(is_transfers_collection_valid(&transfers));
    }
}
