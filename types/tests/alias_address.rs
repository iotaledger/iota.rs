// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_types::block::{
    address::{dto::AliasAddressDto, AliasAddress},
    output::AliasId,
    DtoError,
};

const ALIAS_ID: &str = "0xb0c800965d7511f5fb4406274d4e607f87d5c5970bc05e896f841a700e86eafb";
const ALIAS_ID_INVALID: &str = "0xb0c800965d7511f5fb4406274d4e607f87d5c5970bc05e896f841a700e86e";

#[test]
fn kind() {
    assert_eq!(AliasAddress::KIND, 8);
}

#[test]
fn length() {
    assert_eq!(AliasAddress::LENGTH, 32);
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
        "AliasAddress(0xb0c800965d7511f5fb4406274d4e607f87d5c5970bc05e896f841a700e86eafb)"
    );
}

#[test]
fn dto_fields() {
    let alias_address = AliasAddress::from_str(ALIAS_ID).unwrap();
    let dto = AliasAddressDto::from(&alias_address);

    assert_eq!(dto.kind, AliasAddress::KIND);
    assert_eq!(dto.alias_id, ALIAS_ID.to_string());
}

#[test]
fn address_dto_roundtrip() {
    let alias_address = AliasAddress::from_str(ALIAS_ID).unwrap();
    let dto = AliasAddressDto::from(&alias_address);

    assert_eq!(AliasAddress::try_from(&dto).unwrap(), alias_address);
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
