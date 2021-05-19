// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use iota_client::{
    Seed,
    api::{
        GetAddressesBuilder as RustGetAddressesBuilderApi
    },
    bee_message::prelude::{Address as RustAddress},
    bee_rest_api::types::{
        dtos::AddressDto as RustAddressDto, responses::BalanceAddressResponse as RustBalanceAddressResponse,
    }
};

use std::{cell::RefCell,rc::Rc};
use getset::{CopyGetters, Getters};
use std::{
    convert::From,
    fmt::{Display, Formatter},
};

use crate::{
    Result,
    full_node_api::Client,
};
use anyhow::anyhow;

#[derive(Clone, Debug, Getters, CopyGetters, PartialEq)]
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

#[derive(Clone, Debug, Getters, CopyGetters, PartialEq)]
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

pub struct GetAddressesBuilderApi<'a> {
    builder: Rc<RefCell<Option<RustGetAddressesBuilderApi<'a>>>>,
}

impl<'a> Default for GetAddressesBuilderApi<'a> {
    fn default() -> Self {
        Self {
            builder: Rc::new(RefCell::new(Option::from(RustGetAddressesBuilderApi::default()))),
        }
    }
}

impl<'a> GetAddressesBuilderApi<'a> {
    fn new_with_builder(builder: RustGetAddressesBuilderApi<'a>) -> Self {
        Self {
            builder: Rc::new(RefCell::new(Option::from(builder))),
        }
    }
    
    pub fn new(seed: String) -> Self {
        let rust_seed = Seed::from_bytes(&hex::decode(seed).unwrap());

        Self {
            builder: Rc::new(RefCell::new(Option::from(RustGetAddressesBuilderApi::new(&rust_seed)))),
        }
    }

    /// Provide a client to get the bech32_hrp from the node
    pub fn with_client(self, client: Client) -> Self {
        let new_builder = self.builder.borrow_mut().take().unwrap().with_client(client.borrow());
        GetAddressesBuilderApi::new_with_builder(new_builder)
    }

    /// Set the account index
    pub fn with_account_index(&self, account_index: usize) -> Self {
        let new_builder = self.builder.borrow_mut().take().unwrap().with_account_index(account_index);
        GetAddressesBuilderApi::new_with_builder(new_builder)
    }

    /// Set range to the builder
    pub fn with_range(&self, start: usize, end: usize) -> Self {
        let range = start..end;
        let new_builder = self.builder.borrow_mut().take().unwrap().with_range(range);
        GetAddressesBuilderApi::new_with_builder(new_builder)
    }

    /// Set bech32 human readable part (hrp)
    pub fn with_bech32_hrp(&self, bech32_hrp: String) -> Self {
        let new_builder = self.builder.borrow_mut().take().unwrap().with_bech32_hrp(bech32_hrp);
        GetAddressesBuilderApi::new_with_builder(new_builder)
    }

    /*
    /// Consume the builder and get a vector of public addresses bech32 encoded
    pub async fn finish(self) -> Result<Vec<String>> {
        let client = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move { self.builder.borrow_mut().take().unwrap().finish().await.unwrap() });

        Ok(Client::try_from(client).unwrap())
    }

    /// Consume the builder and get the vector of public and internal addresses bech32 encoded
    pub async fn get_all(self) -> Result<Vec<(String, bool)>> {
        
    }
    /// Consume the builder and get the vector of public and internal addresses
    pub async fn get_all_raw(self) -> Result<Vec<(Address, bool)>> {

    }*/
}