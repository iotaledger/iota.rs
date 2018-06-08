use utils::input_validator;
use errors::*;
use pow::kerl::{self, Kerl};
use pow::traits::ICurl;
use utils::converter;
use utils::constants;

pub fn add_checksum(address: &str) -> String {
    assert!(input_validator::check_address(address));
    let mut address_with_checksum = address.to_string();
    address_with_checksum += &calculate_checksum(&address);
    address_with_checksum
}

pub fn remove_checksum(address: &str) -> String {
    if is_address_with_checksum(address) {
        return remove_checksum_from_address(address);
    } else if is_address_without_checksum(address) {
        return address.to_string();
    }
    panic!(constants::INVALID_ADDRESSES_INPUT_ERROR);
}

pub fn is_valid_checksum(address: &str) -> bool {
    let mut address_without_checksum = remove_checksum(address);
    let address_with_recalculated_checksum = address_without_checksum.clone() + &calculate_checksum(&address_without_checksum);
    address == address_with_recalculated_checksum
}

fn remove_checksum_from_address(address: &str) -> String {
    address[0..constants::ADDRESS_LENGTH_WITHOUT_CHECKSUM].to_string()
}

pub fn is_address_with_checksum(address: &str) -> bool {
    input_validator::check_address(address) && address.len() == constants::ADDRESS_LENGTH_WITH_CHECKSUM
}

pub fn is_address_without_checksum(address: &str) -> bool {
    input_validator::check_address(address) && address.len() == constants::ADDRESS_LENGTH_WITHOUT_CHECKSUM
}

fn calculate_checksum(address: &str) -> String {
    let mut curl = Kerl::default();
    curl.reset();
    curl.absorb(&converter::trits_from_string(address));
    let mut checksum_trits = [0; 243];
    curl.squeeze(&mut checksum_trits);
    let checksum = converter::trytes(&checksum_trits);
    checksum[72..81].to_string()
}