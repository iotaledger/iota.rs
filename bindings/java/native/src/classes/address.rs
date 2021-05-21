// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use iota_client::{
    api::GetAddressesBuilder as RustGetAddressesBuilderApi,
    node::GetAddressBuilder as RustGetAddressBuilderNode,
    bee_message::prelude::Address as RustAddress,
    bee_rest_api::types::{
        dtos::AddressDto as RustAddressDto, responses::BalanceAddressResponse as RustBalanceAddressResponse,
    },
    Seed as RustSeed,
};

use getset::{CopyGetters, Getters};
use std::{
    cell::RefCell,
    convert::From,
    fmt::{Display, Formatter},
    ops::Range,
    rc::Rc,
};

use crate::{
    full_node_api::Client, bee_types::{
        OutputsOptions, UtxoInput
    }, 
    Result
};

use anyhow::anyhow;

pub const ADDRESS_GAP_RANGE: usize = 20;

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

#[derive(Clone, Getters, CopyGetters, PartialEq)]
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

impl Display for BalanceAddressResponse {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "(address_type={}, address={}, balance={}, dust_allowed={})", 
        self.address_type, self.address, self.balance, self.dust_allowed)
    }
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

pub struct GetAddressBuilderNode<'a> {
    client: &'a Client,
}

impl<'a> GetAddressBuilderNode<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            client: client
        }
    }

    pub fn balance(&self, address: &str) -> Result<BalanceAddressResponse> {
        let res = crate::block_on(async {
            RustGetAddressBuilderNode::new(self.client.borrow()).balance(address).await
        });
        match res {
            Ok(r) => Ok(r.into()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
    
    pub fn outputs(&self, address: &str, options: OutputsOptions) -> Result<Vec<UtxoInput>> {
        let res = crate::block_on(async {
            RustGetAddressBuilderNode::new(self.client.borrow()).outputs(address, options.to_inner()).await
        });
        match res {
            Ok(r) => Ok(r.iter().map(|input| {
                input.clone().into()
            }).collect()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
}

struct GetAddressesBuilderApiInternal {
    seed: RustSeed,
    account_index: usize,
    range: Range<usize>,
    bech32_hrp: Option<String>,
    client: Option<Client>,
}

pub struct GetAddressesBuilderApi {
    fields: Rc<RefCell<Option<GetAddressesBuilderApiInternal>>>,
}

impl GetAddressesBuilderApi {
    pub fn new(seed: &str) -> Self {
        let internal = GetAddressesBuilderApiInternal {
            seed: RustSeed::from_bytes(seed.as_bytes()),
            account_index: 0,
            range: 0..ADDRESS_GAP_RANGE,
            bech32_hrp: None,
            client: None,
        };
        Self {
            fields: Rc::new(RefCell::new(Option::from(internal))),
        }
    }

    fn new_with_fields(fields: GetAddressesBuilderApiInternal) -> Self {
        Self {
            fields: Rc::new(RefCell::new(Option::from(fields))),
        }
    }

    /// Set the account index
    pub fn with_account_index(&self, account_index: usize) -> Self {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        fields.account_index = account_index;
        GetAddressesBuilderApi::new_with_fields(fields)
    }

    /// Set range to the builder
    pub fn with_range(&self, start: usize, end: usize) -> Self {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        fields.range = start..end;
        GetAddressesBuilderApi::new_with_fields(fields)
    }

    /// Set client to the builder
    pub fn with_client(&self, client: Client) -> Self {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        fields.client = Some(client);
        GetAddressesBuilderApi::new_with_fields(fields)
    }

    /// Set bech32 human readable part (hrp)
    pub fn with_bech32_hrp(&self, bech32_hrp: String) -> Self {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        fields.bech32_hrp = Some(bech32_hrp);
        GetAddressesBuilderApi::new_with_fields(fields)
    }

    pub fn finish(&self) -> Result<Vec<String>> {
        let fields = self.fields.borrow_mut().take().unwrap();
        let ret = crate::block_on(async {
            let mut builder = RustGetAddressesBuilderApi::new(&fields.seed)
                .with_account_index(fields.account_index)
                .with_range(fields.range);

            if let Some(b) = fields.bech32_hrp {
                builder = builder.with_bech32_hrp(b);
            }
            if let Some(c) = fields.client {
                builder.with_client(c.borrow()).finish().await
            } else {
                builder.finish().await
            }
        });

        match ret {
            Ok(e) => Ok(e),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
    
    /*
    // Consume the builder and get the vector of public and internal addresses bech32 encoded
    pub async fn get_all(self) -> Result<Vec<(String, bool)>> {
    
    }

    // Consume the builder and get the vector of public and internal addresses
    pub async fn get_all_raw(self) -> Result<Vec<(Address, bool)>> {
    
    }
    */
}
