// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use iota_client::{
    api::{search_address as search_address_api, GetAddressesBuilder as RustGetAddressesBuilder},
    bee_message::prelude::Address as RustAddress,
    bee_rest_api::types::{
        dtos::AddressDto as RustAddressDto, responses::BalanceAddressResponse as RustBalanceAddressResponse,
    },
    node::GetAddressBuilder as RustGetAddressBuilder,
    Seed as RustSeed,
};

use getset::{CopyGetters, Getters};
use std::{
    borrow::Borrow,
    cell::RefCell,
    convert::From,
    fmt::{Display, Formatter},
    ops::Range,
    rc::Rc,
};

use crate::{
    bee_types::{OutputsOptions, SignatureUnlock, UtxoInput},
    full_node_api::Client,
    Result,
};

use anyhow::anyhow;

pub const ADDRESS_GAP_RANGE: usize = 20;

#[derive(Clone, Debug, Getters, CopyGetters, Eq, PartialEq)]
pub struct AddressDto {
    #[getset(get_copy = "pub")]
    pub kind: u8,
    #[getset(get = "pub")]
    pub address: String,
}

impl Display for AddressDto {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "(address={}, kind={})", self.address, self.kind)
    }
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

#[derive(Clone, Getters, CopyGetters, Eq, PartialEq, Debug)]
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
        write!(
            f,
            "(address_type={}, address={}, balance={}, dust_allowed={})",
            self.address_type, self.address, self.balance, self.dust_allowed
        )
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

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Address {
    address: RustAddress,
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.address)
    }
}

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
    pub fn to_inner_clone(&self) -> RustAddress {
        self.address
    }

    pub fn to_bech32(&self, hrp: &str) -> String {
        self.address.to_bech32(hrp)
    }

    pub fn verify(&self, msg: Vec<u8>, signature: SignatureUnlock) -> Result<()> {
        match self.address.verify(&msg, signature.to_inner_ref()) {
            Ok(()) => Ok(()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
}

#[derive(Clone, Debug, Getters, CopyGetters)]
pub struct IndexPublicDto {
    #[getset(get_copy = "pub")]
    index: usize,
    #[getset(get_copy = "pub")]
    is_public: bool,
}

/// Function to find the index and public or internal type of an Bech32 encoded address
pub fn search_address(
    seed: &str,
    bech32_hrp: &str,
    account_index: usize,
    range_low: usize,
    range_high: usize,
    address: Address,
) -> Result<IndexPublicDto> {
    let res = crate::block_on(async {
        search_address_api(
            &RustSeed::from_bytes(&hex::decode(seed)?),
            bech32_hrp,
            account_index,
            range_low..range_high,
            &address.address,
        )
        .await
    });
    match res {
        Ok((index, is_public)) => Ok(IndexPublicDto { index, is_public }),
        Err(e) => Err(anyhow!(e.to_string())),
    }
}

pub struct GetAddressBuilder<'a> {
    client: &'a Client,
}

impl<'a> GetAddressBuilder<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn balance(&self, address: &str) -> Result<BalanceAddressResponse> {
        let res = crate::block_on(async { RustGetAddressBuilder::new(self.client.borrow()).balance(address).await });
        match res {
            Ok(r) => Ok(r.into()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    pub fn outputs(&self, address: &str, options: OutputsOptions) -> Result<Vec<UtxoInput>> {
        let res = crate::block_on(async {
            RustGetAddressBuilder::new(self.client.borrow())
                .outputs(address, options.to_inner())
                .await
        });
        match res {
            Ok(r) => Ok(r.iter().map(|input| input.clone().into()).collect()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
}

struct GetAddressesBuilderInternal<'a> {
    seed: RustSeed,
    account_index: usize,
    range: Range<usize>,
    bech32_hrp: Option<String>,
    client: Option<&'a Client>,
}

pub struct GetAddressesBuilder<'a> {
    fields: Rc<RefCell<Option<GetAddressesBuilderInternal<'a>>>>,
}

impl<'a> GetAddressesBuilder<'a> {
    pub(crate) fn from_old(seed: &str) -> Self {
        let internal = GetAddressesBuilderInternal {
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

    pub fn from(seed: &str) -> Result<Self> {
        match hex::decode(seed) {
            Ok(s) => {
                let internal = GetAddressesBuilderInternal {
                    seed: RustSeed::from_bytes(&s),
                    account_index: 0,
                    range: 0..ADDRESS_GAP_RANGE,
                    bech32_hrp: None,
                    client: None,
                };
                Ok(Self {
                    fields: Rc::new(RefCell::new(Option::from(internal))),
                })
            }
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    fn new_with_fields(fields: GetAddressesBuilderInternal<'a>) -> Self {
        Self {
            fields: Rc::new(RefCell::new(Option::from(fields))),
        }
    }

    /// Set the account index
    pub fn with_account_index(&self, account_index: usize) -> Self {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        fields.account_index = account_index;
        GetAddressesBuilder::new_with_fields(fields)
    }

    /// Set range to the builder
    pub fn with_range(&self, start: usize, end: usize) -> Self {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        fields.range = start..end;
        GetAddressesBuilder::new_with_fields(fields)
    }

    /// Set client to the builder
    pub fn with_client(&self, client: &'a Client) -> Self {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        fields.client = Some(client);
        GetAddressesBuilder::new_with_fields(fields)
    }

    /// Set bech32 human readable part (hrp)
    pub fn with_bech32_hrp(&self, bech32_hrp: String) -> Self {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        fields.bech32_hrp = Some(bech32_hrp);
        GetAddressesBuilder::new_with_fields(fields)
    }

    pub fn finish(&self) -> Result<Vec<String>> {
        let fields = self.fields.borrow_mut().take().unwrap();
        let ret = crate::block_on(async {
            let mut builder = RustGetAddressesBuilder::new(&fields.seed)
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

    pub fn get_all(&self) -> Result<Vec<AddressStringPublicWrapper>> {
        let fields = self.fields.borrow_mut().take().unwrap();
        let ret = crate::block_on(async {
            let mut builder = RustGetAddressesBuilder::new(&fields.seed)
                .with_account_index(fields.account_index)
                .with_range(fields.range);

            if let Some(b) = fields.bech32_hrp {
                builder = builder.with_bech32_hrp(b);
            }
            if let Some(c) = fields.client {
                builder.with_client(c.borrow()).get_all().await
            } else {
                builder.get_all().await
            }
        });

        match ret {
            Ok(e) => Ok(e
                .iter()
                .map(|t| AddressStringPublicWrapper {
                    address: t.0.clone(),
                    public: t.1,
                })
                .collect()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    pub fn get_all_raw(&self) -> Result<Vec<AddressPublicWrapper>> {
        let fields = self.fields.borrow_mut().take().unwrap();
        let ret = crate::block_on(async {
            let mut builder = RustGetAddressesBuilder::new(&fields.seed)
                .with_account_index(fields.account_index)
                .with_range(fields.range);

            if let Some(b) = fields.bech32_hrp {
                builder = builder.with_bech32_hrp(b);
            }
            if let Some(c) = fields.client {
                builder.with_client(c.borrow()).get_all_raw().await
            } else {
                builder.get_all_raw().await
            }
        });

        match ret {
            Ok(e) => Ok(e
                .iter()
                .map(|t| AddressPublicWrapper {
                    address: t.0.into(),
                    public: t.1,
                })
                .collect()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
}

#[derive(Clone, Debug, Getters, CopyGetters, Eq, PartialEq)]
pub struct AddressStringPublicWrapper {
    #[getset(get = "pub")]
    address: String,
    #[getset(get_copy = "pub")]
    public: bool,
}
impl Display for AddressStringPublicWrapper {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "(address={}, public={})", self.address, self.public)
    }
}

#[derive(Clone, Debug, Getters, CopyGetters, Eq, PartialEq)]
pub struct AddressPublicWrapper {
    address: Address,
    #[getset(get_copy = "pub")]
    public: bool,
}

impl AddressPublicWrapper {
    pub fn address(&self) -> Address {
        self.address.clone()
    }
}
impl Display for AddressPublicWrapper {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "(address={}, public={})", self.address, self.public)
    }
}
