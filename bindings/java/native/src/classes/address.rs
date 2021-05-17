// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use iota_client::bee_message::prelude::{Address as RustAddress};
use bee_rest_api::types::{
    dtos::AddressDto as RustAddressDto, responses::BalanceAddressResponse as RustBalanceAddressResponse,
};
use getset::{CopyGetters, Getters};
use std::{
    convert::From,
    fmt::{Display, Formatter},
};

use crate::Result;
use anyhow::anyhow;

#[derive(Clone, Debug, Getters, CopyGetters)]
pub struct AddressDto {
    #[getset(get_copy = "pub")]
    pub kind: u8,
    #[getset(get = "pub")]
    pub address: String,
}

impl From<RustAddressDto> for AddressDto {
    fn from(address: RustAddressDto) -> Self {
        match address {
            RustAddressDto::Ed25519(ed) => Self {
                kind: ed.kind,
                address: ed.address,
            },
        }
    }
}

#[derive(Clone, Debug, Getters, CopyGetters)]
pub struct BalanceAddressResponse {
    #[getset(get_copy = "pub")]
    pub address_type: u8,
    #[getset(get = "pub")]
    pub address: String,
    #[getset(get_copy = "pub")]
    pub balance: u64,
    #[getset(get_copy = "pub")]
    pub dust_allowed: bool,
}

impl From<RustBalanceAddressResponse> for BalanceAddressResponse {
    fn from(response: RustBalanceAddressResponse) -> Self {
        Self {
            address_type: response.address_type,
            address: response.address,
            balance: response.balance,
            dust_allowed: response.dust_allowed,
        }
    }
}

#[derive(Clone, Debug, Getters, CopyGetters)]
pub struct AddressOutputsOptions {}

//
#[derive(Clone, PartialEq)]
pub struct Address {
    address: RustAddress,
}
//
impl Display for Address {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.address)
    }
}
//
impl From<RustAddress> for Address {
    fn from(address: RustAddress) -> Self {
        Self { address }
    }
}

impl Address {
    pub fn try_from_bech32(addr: &str) -> Result<Self> {
        match RustAddress::try_from_bech32(addr) {
            Ok(addr) => Ok(addr.into()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
    // pub fn kind(&self) -> u8 {
    // match self {
    // Self::Ed25519(_) => Ed25519Address::KIND,
    // }
    // }
    //
    // Tries to create an `Address` from a Bech32 encoded string.
    //
    //
    // Encodes this address to a Bech32 string with the hrp (human readable part) argument as prefix.
    // pub fn to_bech32(&self, hrp: &str) -> String
    //
    // Verifies a [`SignatureUnlock`] for a message against the [`Address`].
    // pub fn verify(&self, msg: &[u8], signature: &SignatureUnlock) -> Result<(), Error> {
}
