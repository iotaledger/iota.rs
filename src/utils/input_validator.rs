use model::transfer::Transfer;
use regex::Regex;
use utils::constants;

pub fn is_address(address: &str) -> bool {
    address.len() == constants::ADDRESS_LENGTH_WITHOUT_CHECKSUM
        || address.len() == constants::ADDRESS_LENGTH_WITH_CHECKSUM
            && is_trytes(&address, address.len())
}

pub fn check_address(address: &str) -> bool {
    if !is_address(address) {
        return false;
    }
    true
}

pub fn is_addresses_collection_valid(addresses: Vec<String>) -> bool {
    for address in addresses {
        if !check_address(&address) {
            return false;
        }
    }
    true
}

pub fn is_trytes(trytes: &str, length: usize) -> bool {
    let tmp = if length == 0 {
        "0,".to_string()
    } else {
        length.to_string()
    };
    let re = Regex::new(&format!("{}{}{}", "^[A-Z9]{", tmp, "}$")).expect("Failed to parse regex");
    re.is_match(trytes)
}

pub fn is_nine_trytes(trytes: &str, length: usize) -> bool {
    let tmp = if length == 0 {
        "0,".to_string()
    } else {
        length.to_string()
    };
    let re = Regex::new(&format!("{}{}{}", "^[9]{", tmp, "}$")).expect("Failed to parse regex");
    re.is_match(trytes)
}

pub fn is_value(value: &str) -> bool {
    match value.parse::<i64>() {
        Ok(val) => true,
        Err(e) => match value.parse::<u64>() {
            Ok(val) => true,
            Err(e) => false,
        },
    }
}

pub fn is_array_of_trytes(trytes: Vec<String>) -> bool {
    for tryte in trytes {
        if !is_trytes(&tryte, 2673) {
            return false;
        }
    }
    true
}

pub fn is_array_of_hashes(hashes: Vec<String>) -> bool {
    for hash in hashes {
        if hash.len() == 90 {
            if !is_trytes(&hash, 90) {
                return false;
            }
        } else {
            if !is_trytes(&hash, 81) {
                return false;
            }
        }
    }
    true
}

pub fn is_valid_transfer(transfer: &Transfer) -> bool {
    if !is_address(transfer.address()) {
        return false;
    }
    if *transfer.message() == None
        || !is_trytes(
            &transfer.message().clone().unwrap(),
            transfer.message().clone().unwrap().len(),
        ) {
        return false;
    }
    if *transfer.tag() == None
        || !is_trytes(
            &transfer.tag().clone().unwrap(),
            transfer.tag().clone().unwrap().len(),
        ) {
        return false;
    }
    true
}

pub fn is_transfers_collection_valid(transfers: Vec<Transfer>) -> bool {
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
    is_trytes(seed, seed.len())
}

pub fn is_hashes(hashes: Vec<String>) -> bool {
    for hash in hashes {
        if !is_trytes(&hash, 81) {
            return false;
        }
    }
    true
}

pub fn is_hash(hash: &str) -> bool {
    is_trytes(hash, 81)
}

pub fn is_array_of_attached_trytes(trytes: Vec<String>) -> bool {
    for tryte_value in trytes {
        if !is_trytes(&tryte_value, 2673) {
            return false;
        }
        let last_trytes: String = tryte_value[2673 - (3 * 81)..].to_string();
        if is_nine_trytes(&last_trytes, last_trytes.len()) {
            return false;
        }
    }
    true
}
