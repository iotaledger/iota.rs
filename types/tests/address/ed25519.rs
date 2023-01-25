// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use iota_types::block::{
    address::{dto::Ed25519AddressDto, Address, Ed25519Address},
    DtoError,
};
use packable::PackableExt;

const ED25519_ADDRESS: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649";
const ED25519_ADDRESS_INVALID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64";

#[test]
fn kind_const() {
    assert_eq!(Ed25519Address::KIND, 0);
}

#[test]
fn kind_method() {
    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS).unwrap());

    assert_eq!(address.kind(), Ed25519Address::KIND);
}

#[test]
fn length() {
    assert_eq!(Ed25519Address::LENGTH, 32);
}

#[test]
fn is_methods() {
    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS).unwrap());

    assert_eq!(address.is_ed25519(), true);
    assert_eq!(address.is_alias(), false);
    assert_eq!(address.is_nft(), false);
}

#[test]
fn as_methods() {
    let ed25519_address = Ed25519Address::from_str(ED25519_ADDRESS).unwrap();
    let address = Address::from(ed25519_address);

    assert_eq!(address.as_ed25519(), &ed25519_address);
    assert!(std::panic::catch_unwind(|| address.as_alias()).is_err());
    assert!(std::panic::catch_unwind(|| address.as_nft()).is_err());
}

#[test]
fn new_bytes() {
    let bytes = prefix_hex::decode::<[u8; Ed25519Address::LENGTH]>(ED25519_ADDRESS).unwrap();
    let ed25519_address = Ed25519Address::new(bytes);

    assert_eq!(ed25519_address.as_ref(), &bytes);
}

#[test]
fn from_str_to_str() {
    let ed25519_address = Ed25519Address::from_str(ED25519_ADDRESS).unwrap();

    assert_eq!(ed25519_address.to_string(), ED25519_ADDRESS);
}

#[test]
fn debug() {
    let ed25519_address = Ed25519Address::from_str(ED25519_ADDRESS).unwrap();

    assert_eq!(
        format!("{ed25519_address:?}"),
        "Ed25519Address(0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649)"
    );
}

#[test]
fn dto_fields() {
    let ed25519_address = Ed25519Address::from_str(ED25519_ADDRESS).unwrap();
    let dto = Ed25519AddressDto::from(&ed25519_address);

    assert_eq!(dto.kind, Ed25519Address::KIND);
    assert_eq!(dto.pub_key_hash, ED25519_ADDRESS.to_string());
}

#[test]
fn address_dto_roundtrip() {
    let ed25519_address = Ed25519Address::from_str(ED25519_ADDRESS).unwrap();
    let dto = Ed25519AddressDto::from(&ed25519_address);

    assert_eq!(Ed25519Address::try_from(&dto).unwrap(), ed25519_address);
}

#[test]
fn dto_invalid_pub_key_hash() {
    let dto = Ed25519AddressDto {
        kind: Ed25519Address::KIND,
        pub_key_hash: ED25519_ADDRESS_INVALID.to_string(),
    };

    assert!(matches!(
        Ed25519Address::try_from(&dto),
        Err(DtoError::InvalidField("pubKeyHash"))
    ));
}

#[test]
fn packed_len() {
    let address = Ed25519Address::from_str(ED25519_ADDRESS).unwrap();

    assert_eq!(address.packed_len(), Ed25519Address::LENGTH);
    assert_eq!(address.pack_to_vec().len(), Ed25519Address::LENGTH);
}

#[test]
fn pack_unpack() {
    let address = Ed25519Address::from_str(ED25519_ADDRESS).unwrap();
    let packed_address = address.pack_to_vec();

    assert_eq!(
        address,
        PackableExt::unpack_verified(packed_address.as_slice(), &()).unwrap()
    );
}
