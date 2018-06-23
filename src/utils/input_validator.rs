use crate::model::transfer::Transfer;
use regex::Regex;
use super::constants;

pub fn is_address(address: &str) -> bool {
    address.len() == constants::ADDRESS_LENGTH_WITHOUT_CHECKSUM
        || address.len() == constants::ADDRESS_LENGTH_WITH_CHECKSUM && is_trytes(address)
}

pub fn check_address(address: &str) -> bool {
    if !is_address(address) {
        return false;
    }
    true
}

pub fn is_addresses_collection_valid(addresses: &[String]) -> bool {
    for address in addresses {
        if !check_address(&address) {
            return false;
        }
    }
    true
}

pub fn is_trytes(trytes: &str) -> bool {
    let tmp = if trytes.is_empty() {
        "0,".to_string()
    } else {
        trytes.len().to_string()
    };
    let re = Regex::new(&format!("{}{}{}", "^[A-Z9]{", tmp, "}$")).expect("Failed to parse regex");
    re.is_match(trytes)
}

pub fn is_nine_trytes(trytes: &str) -> bool {
    let tmp = if trytes.is_empty() {
        "0,".to_string()
    } else {
        trytes.len().to_string()
    };
    let re = Regex::new(&format!("{}{}{}", "^[9]{", tmp, "}$")).expect("Failed to parse regex");
    re.is_match(trytes)
}

pub fn is_value(value: &str) -> bool {
    match value.parse::<i64>() {
        Ok(_val) => true,
        Err(_e) => match value.parse::<u64>() {
            Ok(_val) => true,
            Err(_e) => false,
        },
    }
}

pub fn is_array_of_trytes(trytes: &[String]) -> bool {
    for tryte in trytes {
        if !is_trytes(&tryte[0..2673]) {
            return false;
        }
    }
    true
}

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

pub fn is_valid_transfer(transfer: &Transfer) -> bool {
    if !is_address(transfer.address()) {
        return false;
    }
    if !is_trytes(transfer.message()) {
        return false;
    }
    if !is_trytes(transfer.tag()) {
        return false;
    }
    true
}

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

pub fn is_valid_seed(seed: &str) -> bool {
    is_trytes(seed)
}

pub fn is_hashes(hashes: &[String]) -> bool {
    for hash in hashes {
        if !is_trytes(&hash[0..81]) {
            return false;
        }
    }
    true
}

pub fn is_hash(hash: &str) -> bool {
    is_trytes(&hash[0..81])
}

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
    fn test_check_address() {
        assert!(check_address(TEST_ADDRESS_WITHOUT_CHECKSUM))
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
        *t.address_mut() = TEST_ADDRESS_WITH_CHECKSUM.to_string();
        *t.value_mut() = 0;
        *t.message_mut() = TEST_MESSAGE.to_string();
        *t.tag_mut() = TEST_TAG.to_string();
        assert!(is_valid_transfer(&t));
    }

    #[test]
    fn test_is_transfers_collection_valid() {
        let mut t = Transfer::default();
        *t.address_mut() = TEST_ADDRESS_WITH_CHECKSUM.to_string();
        *t.value_mut() = 0;
        *t.message_mut() = TEST_MESSAGE.to_string();
        *t.tag_mut() = TEST_TAG.to_string();

        let mut t2 = Transfer::default();
        *t2.address_mut() = TEST_ADDRESS_WITH_CHECKSUM.to_string();
        *t2.value_mut() = 0;
        *t2.message_mut() = "".to_string();
        *t2.tag_mut() = "".to_string();

        let mut t3 = Transfer::default();
        *t3.address_mut() = TEST_ADDRESS_WITH_CHECKSUM.to_string();
        *t3.value_mut() = 0;
        let transfers = vec![t, t2, t3];
        assert!(is_transfers_collection_valid(&transfers));
    }
}
