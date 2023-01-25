// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_types::block::{
    address::{dto::NftAddressDto, Address, NftAddress},
    output::NftId,
    DtoError,
};
use packable::PackableExt;

const NFT_ID: &str = "0xb0c800965d7511f5fb4406274d4e607f87d5c5970bc05e896f841a700e86eafb";
const NFT_ID_INVALID: &str = "0xb0c800965d7511f5fb4406274d4e607f87d5c5970bc05e896f841a700e86e";

#[test]
fn kind() {
    assert_eq!(NftAddress::KIND, 16);

    let address = Address::from(NftAddress::from_str(NFT_ID).unwrap());

    assert_eq!(address.kind(), NftAddress::KIND);
}

#[test]
fn length() {
    assert_eq!(NftAddress::LENGTH, 32);
}

#[test]
fn is_methods() {
    let address = Address::from(NftAddress::from_str(NFT_ID).unwrap());

    assert_eq!(address.is_ed25519(), false);
    assert_eq!(address.is_alias(), false);
    assert_eq!(address.is_nft(), true);
}

#[test]
fn as_methods() {
    let nft_address = NftAddress::from_str(NFT_ID).unwrap();
    let address = Address::from(nft_address);

    assert!(std::panic::catch_unwind(|| address.as_ed25519()).is_err());
    assert!(std::panic::catch_unwind(|| address.as_alias()).is_err());
    assert_eq!(address.as_nft(), &nft_address);
}

#[test]
fn new_nft_id() {
    let nft_id = NftId::from_str(NFT_ID).unwrap();
    let nft_address = NftAddress::new(nft_id);

    assert_eq!(nft_address.nft_id(), &nft_id);
}

#[test]
fn new_into_nft_id() {
    let nft_id = NftId::from_str(NFT_ID).unwrap();
    let nft_address = NftAddress::new(nft_id);

    assert_eq!(nft_address.into_nft_id(), nft_id);
}

#[test]
fn from_str_to_str() {
    let nft_address = NftAddress::from_str(NFT_ID).unwrap();

    assert_eq!(nft_address.to_string(), NFT_ID);
}

#[test]
fn debug() {
    let nft_address = NftAddress::from_str(NFT_ID).unwrap();

    assert_eq!(
        format!("{nft_address:?}"),
        "NftAddress(0xb0c800965d7511f5fb4406274d4e607f87d5c5970bc05e896f841a700e86eafb)"
    );
}

#[test]
fn dto_fields() {
    let nft_address = NftAddress::from_str(NFT_ID).unwrap();
    let dto = NftAddressDto::from(&nft_address);

    assert_eq!(dto.kind, NftAddress::KIND);
    assert_eq!(dto.nft_id, NFT_ID.to_string());
}

#[test]
fn address_dto_roundtrip() {
    let nft_address = NftAddress::from_str(NFT_ID).unwrap();
    let dto = NftAddressDto::from(&nft_address);

    assert_eq!(NftAddress::try_from(&dto).unwrap(), nft_address);
}

#[test]
fn dto_invalid_nft_id() {
    let dto = NftAddressDto {
        kind: NftAddress::KIND,
        nft_id: NFT_ID_INVALID.to_string(),
    };

    assert!(matches!(
        NftAddress::try_from(&dto),
        Err(DtoError::InvalidField("nftId"))
    ));
}

#[test]
fn packed_len() {
    let address = NftAddress::from_str(NFT_ID).unwrap();

    assert_eq!(address.packed_len(), NftAddress::LENGTH);
    assert_eq!(address.pack_to_vec().len(), NftAddress::LENGTH);

    let address = Address::from(NftAddress::from_str(NFT_ID).unwrap());

    assert_eq!(address.packed_len(), 1 + NftAddress::LENGTH);
    assert_eq!(address.pack_to_vec().len(), 1 + NftAddress::LENGTH);
}

#[test]
fn pack_unpack() {
    let address = NftAddress::from_str(NFT_ID).unwrap();
    let packed_address = address.pack_to_vec();

    assert_eq!(
        address,
        PackableExt::unpack_verified(packed_address.as_slice(), &()).unwrap()
    );

    let address = Address::from(NftAddress::from_str(NFT_ID).unwrap());
    let packed_address = address.pack_to_vec();

    assert_eq!(
        address,
        PackableExt::unpack_verified(packed_address.as_slice(), &()).unwrap()
    );
}
