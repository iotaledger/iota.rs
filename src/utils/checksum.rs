use super::constants;
use super::converter;
use super::input_validator;
use crate::crypto::{Kerl, Sponge, HASH_LENGTH};

use crate::Result;

/// Adds a checksum to provided address
pub fn add_checksum(address: &str) -> Result<String> {
    assert!(input_validator::is_address(address));
    let mut address_with_checksum = address.to_string();
    address_with_checksum += &calculate_checksum(address)?;
    Ok(address_with_checksum)
}

/// Removes a checksum from the provided address
pub fn remove_checksum(address: &str) -> String {
    if is_address_with_checksum(address) {
        return remove_checksum_from_address(address);
    } else if is_address_without_checksum(address) {
        return address.to_string();
    }
    panic!(constants::INVALID_ADDRESSES_INPUT_ERROR);
}

/// If an address has a valid checksum
pub fn is_valid_checksum(address: &str) -> Result<bool> {
    let address_without_checksum = remove_checksum(address);
    let address_with_recalculated_checksum =
        address_without_checksum.clone() + &calculate_checksum(&address_without_checksum)?;
    Ok(address == address_with_recalculated_checksum)
}

fn remove_checksum_from_address(address: &str) -> String {
    address[0..constants::ADDRESS_LENGTH_WITHOUT_CHECKSUM].to_string()
}

/// Checks if an address has a checksum
pub fn is_address_with_checksum(address: &str) -> bool {
    input_validator::is_address(address) && address.len() == constants::ADDRESS_LENGTH_WITH_CHECKSUM
}

/// Checks if an address does not have a checksum
pub fn is_address_without_checksum(address: &str) -> bool {
    input_validator::is_address(address)
        && address.len() == constants::ADDRESS_LENGTH_WITHOUT_CHECKSUM
}

fn calculate_checksum(address: &str) -> Result<String> {
    let mut curl = Kerl::default();
    curl.absorb(&converter::trits_from_string(address))?;
    let mut checksum_trits = [0; HASH_LENGTH];
    curl.squeeze(&mut checksum_trits)?;
    let checksum = converter::trytes(&checksum_trits);
    Ok(checksum[72..81].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_ADDRESS_WITHOUT_CHECKSUM: &str =
        "LXQHWNY9CQOHPNMKFJFIJHGEPAENAOVFRDIBF99PPHDTWJDCGHLYETXT9NPUVSNKT9XDTDYNJKJCPQMZC";
    const TEST_ADDRESS_WITH_CHECKSUM: &str = "LXQHWNY9CQOHPNMKFJFIJHGEPAENAOVFRDIBF99PPHDTWJDCGHLYETXT9NPUVSNKT9XDTDYNJKJCPQMZCCOZVXMTXC";

    #[test]
    fn test_add_checksum() {
        assert_eq!(
            add_checksum(TEST_ADDRESS_WITHOUT_CHECKSUM).unwrap(),
            TEST_ADDRESS_WITH_CHECKSUM
        );
    }

    #[test]
    fn test_remove_checksum() {
        assert_eq!(
            remove_checksum(TEST_ADDRESS_WITH_CHECKSUM),
            TEST_ADDRESS_WITHOUT_CHECKSUM
        );
    }

    #[test]
    fn test_is_valid_checksum() {
        assert!(is_valid_checksum(TEST_ADDRESS_WITH_CHECKSUM).unwrap());
    }
}
