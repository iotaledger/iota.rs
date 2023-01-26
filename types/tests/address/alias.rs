// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_types::block::{
    address::{
        dto::{AddressDto, AliasAddressDto},
        Address, AliasAddress,
    },
    output::AliasId,
    DtoError,
};
use packable::PackableExt;

const ALIAS_ID: &str = "0xe9ba80ad1561e437b663a1f1efbfabd544b0d7da7bb33e0a62e99b20ee450bee";
const ALIAS_BECH32: &str = "rms1pr5m4q9dz4s7gdakvwslrmal4025fvxhmfamx0s2vt5ekg8wg597um6lcnn";
const ALIAS_ID_INVALID: &str = "0xb0c800965d7511f5fb4406274d4e607f87d5c5970bc05e896f841a700e86e";

#[test]
fn kind() {
    assert_eq!(AliasAddress::KIND, 8);

    let address = Address::from(AliasAddress::from_str(ALIAS_ID).unwrap());

    assert_eq!(address.kind(), AliasAddress::KIND);
}

#[test]
fn length() {
    assert_eq!(AliasAddress::LENGTH, 32);
}

#[test]
fn is_methods() {
    let address = Address::from(AliasAddress::from_str(ALIAS_ID).unwrap());

    assert!(!address.is_ed25519());
    assert!(address.is_alias());
    assert!(!address.is_nft());
}

#[test]
fn as_methods() {
    let alias_address = AliasAddress::from_str(ALIAS_ID).unwrap();
    let address = Address::from(alias_address);

    assert!(std::panic::catch_unwind(|| address.as_ed25519()).is_err());
    assert_eq!(address.as_alias(), &alias_address);
    assert!(std::panic::catch_unwind(|| address.as_nft()).is_err());
}

#[test]
fn new_alias_id() {
    let alias_id = AliasId::from_str(ALIAS_ID).unwrap();
    let alias_address = AliasAddress::new(alias_id);

    assert_eq!(alias_address.alias_id(), &alias_id);
}

#[test]
fn new_into_alias_id() {
    let alias_id = AliasId::from_str(ALIAS_ID).unwrap();
    let alias_address = AliasAddress::new(alias_id);

    assert_eq!(alias_address.into_alias_id(), alias_id);
}

#[test]
fn from_str_to_str() {
    let alias_address = AliasAddress::from_str(ALIAS_ID).unwrap();

    assert_eq!(alias_address.to_string(), ALIAS_ID);
}

#[test]
fn debug() {
    let alias_address = AliasAddress::from_str(ALIAS_ID).unwrap();

    assert_eq!(
        format!("{alias_address:?}"),
        "AliasAddress(0xe9ba80ad1561e437b663a1f1efbfabd544b0d7da7bb33e0a62e99b20ee450bee)"
    );
}

#[test]
fn bech32() {
    let address = Address::from(AliasAddress::from_str(ALIAS_ID).unwrap());

    assert_eq!(address.to_bech32("rms"), ALIAS_BECH32);
}

#[test]
fn bech32_roundtrip() {
    let address = Address::from(AliasAddress::from_str(ALIAS_ID).unwrap());
    let bech32 = address.to_bech32("rms");

    assert_eq!(
        Address::try_from_bech32(bech32).unwrap(),
        (String::from("rms"), address)
    );
}

#[test]
fn dto_fields() {
    let alias_address = AliasAddress::from_str(ALIAS_ID).unwrap();
    let alias_dto = AliasAddressDto::from(&alias_address);

    assert_eq!(alias_dto.kind, AliasAddress::KIND);
    assert_eq!(alias_dto.alias_id, ALIAS_ID.to_string());

    let address = Address::from(alias_address);
    let dto = AddressDto::from(&address);

    assert_eq!(dto, AddressDto::Alias(alias_dto));
}

#[test]
fn dto_roundtrip() {
    let alias_address = AliasAddress::from_str(ALIAS_ID).unwrap();
    let alias_dto = AliasAddressDto::from(&alias_address);

    assert_eq!(AliasAddress::try_from(&alias_dto).unwrap(), alias_address);

    let address = Address::from(alias_address);
    let dto = AddressDto::from(&address);

    assert_eq!(Address::try_from(&dto).unwrap(), address);
}

#[test]
fn dto_invalid_alias_id() {
    let dto = AliasAddressDto {
        kind: AliasAddress::KIND,
        alias_id: ALIAS_ID_INVALID.to_string(),
    };

    assert!(matches!(
        AliasAddress::try_from(&dto),
        Err(DtoError::InvalidField("aliasId"))
    ));
}

#[test]
fn packed_len() {
    let address = AliasAddress::from_str(ALIAS_ID).unwrap();

    assert_eq!(address.packed_len(), AliasAddress::LENGTH);
    assert_eq!(address.pack_to_vec().len(), AliasAddress::LENGTH);

    let address = Address::from(AliasAddress::from_str(ALIAS_ID).unwrap());

    assert_eq!(address.packed_len(), 1 + AliasAddress::LENGTH);
    assert_eq!(address.pack_to_vec().len(), 1 + AliasAddress::LENGTH);
}

#[test]
fn pack_unpack() {
    let address = AliasAddress::from_str(ALIAS_ID).unwrap();
    let packed_address = address.pack_to_vec();

    assert_eq!(
        address,
        PackableExt::unpack_verified(packed_address.as_slice(), &()).unwrap()
    );

    let address = Address::from(AliasAddress::from_str(ALIAS_ID).unwrap());
    let packed_address = address.pack_to_vec();

    assert_eq!(
        address,
        PackableExt::unpack_verified(packed_address.as_slice(), &()).unwrap()
    );
}
