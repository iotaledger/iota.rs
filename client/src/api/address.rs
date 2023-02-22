// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Range;

use iota_types::block::address::Address;
use serde::Deserialize;

use crate::{
    api::types::{Bech32Addresses, RawAddresses},
    constants::{SHIMMER_COIN_TYPE, SHIMMER_TESTNET_BECH32_HRP},
    secret::{GenerateAddressOptions, SecretManage, SecretManager},
    Client, Result,
};

/// Builder of get_addresses API
#[must_use]
pub struct GetAddressesBuilder<'a> {
    client: Option<&'a Client>,
    secret_manager: &'a SecretManager,
    coin_type: u32,
    account_index: u32,
    range: Range<u32>,
    internal: bool,
    bech32_hrp: Option<String>,
    options: Option<GenerateAddressOptions>,
}

/// Get address builder from string
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAddressesBuilderOptions {
    /// Coin type
    pub coin_type: Option<u32>,
    /// Account index
    pub account_index: Option<u32>,
    /// Range
    pub range: Option<Range<u32>>,
    /// Internal addresses
    pub internal: Option<bool>,
    /// Bech32 human readable part
    pub bech32_hrp: Option<String>,
    /// Options
    pub options: Option<GenerateAddressOptions>,
}

impl<'a> GetAddressesBuilder<'a> {
    /// Create get_addresses builder
    pub fn new(manager: &'a SecretManager) -> Self {
        Self {
            client: None,
            secret_manager: manager,
            coin_type: SHIMMER_COIN_TYPE,
            account_index: 0,
            range: 0..super::ADDRESS_GAP_RANGE,
            internal: false,
            bech32_hrp: None,
            options: None,
        }
    }

    /// Provide a client to get the bech32_hrp from the node
    pub fn with_client(mut self, client: &'a Client) -> Self {
        self.client.replace(client);
        self
    }

    /// Set the coin type
    pub fn with_coin_type(mut self, coin_type: u32) -> Self {
        self.coin_type = coin_type;
        self
    }

    /// Set the account index
    pub fn with_account_index(mut self, account_index: u32) -> Self {
        self.account_index = account_index;
        self
    }

    /// Set range to the builder
    pub fn with_range(mut self, range: Range<u32>) -> Self {
        self.range = range;
        self
    }

    /// Set if internal or public addresses should be generated
    pub fn with_internal_addresses(mut self, internal: bool) -> Self {
        self.internal = internal;
        self
    }

    /// Set bech32 human readable part (hrp)
    pub fn with_bech32_hrp<T: Into<String>>(mut self, bech32_hrp: T) -> Self {
        self.bech32_hrp.replace(bech32_hrp.into());
        self
    }

    /// Set the metadata for the address generation (used for ledger to display addresses or not)
    pub fn with_options(mut self, options: GenerateAddressOptions) -> Self {
        self.options = Some(options);
        self
    }

    /// Set multiple options from address builder options type
    /// Useful for bindings
    pub fn set_options(mut self, options: GetAddressesBuilderOptions) -> Result<Self> {
        if let Some(coin_type) = options.coin_type {
            self = self.with_coin_type(coin_type);
        };

        if let Some(account_index) = options.account_index {
            self = self.with_account_index(account_index);
        }

        if let Some(range) = options.range {
            self = self.with_range(range);
        };

        if let Some(internal) = options.internal {
            self = self.with_internal_addresses(internal);
        };

        if let Some(bech32_hrp) = options.bech32_hrp {
            self = self.with_bech32_hrp(bech32_hrp);
        };

        if let Some(options) = options.options {
            self = self.with_options(options);
        };

        Ok(self)
    }

    /// Consume the builder and get a vector of public addresses bech32 encoded
    pub async fn finish(self) -> Result<Vec<String>> {
        let bech32_hrp = match self.bech32_hrp.clone() {
            Some(bech32_hrp) => bech32_hrp,
            None => match self.client {
                Some(client) => client.get_bech32_hrp().await?,
                None => SHIMMER_TESTNET_BECH32_HRP.to_string(),
            },
        };

        let addresses = self
            .secret_manager
            .generate_addresses(
                self.coin_type,
                self.account_index,
                self.range,
                self.internal,
                self.options.clone(),
            )
            .await?
            .into_iter()
            .map(|a| a.to_bech32(&bech32_hrp))
            .collect();

        Ok(addresses)
    }
    /// Consume the builder and get a vector of public addresses
    pub async fn get_raw(self) -> Result<Vec<Address>> {
        self.secret_manager
            .generate_addresses(
                self.coin_type,
                self.account_index,
                self.range,
                false,
                self.options.clone(),
            )
            .await
    }

    /// Consume the builder and get the vector of public and internal addresses bech32 encoded
    pub async fn get_all(self) -> Result<Bech32Addresses> {
        let bech32_hrp = match self.bech32_hrp.clone() {
            Some(bech32_hrp) => bech32_hrp,
            None => match self.client {
                Some(client) => client.get_bech32_hrp().await?,
                None => SHIMMER_TESTNET_BECH32_HRP.to_string(),
            },
        };
        let addresses = self.get_all_raw().await?;

        Ok(Bech32Addresses {
            public: addresses.public.into_iter().map(|a| a.to_bech32(&bech32_hrp)).collect(),
            internal: addresses
                .internal
                .into_iter()
                .map(|a| a.to_bech32(&bech32_hrp))
                .collect(),
        })
    }

    /// Consume the builder and get the vector of public and internal addresses
    pub async fn get_all_raw(self) -> Result<RawAddresses> {
        let public_addresses = self
            .secret_manager
            .generate_addresses(
                self.coin_type,
                self.account_index,
                self.range.clone(),
                false,
                self.options.clone(),
            )
            .await?;

        let internal_addresses = self
            .secret_manager
            .generate_addresses(
                self.coin_type,
                self.account_index,
                self.range,
                true,
                self.options.clone(),
            )
            .await?;

        Ok(RawAddresses {
            public: public_addresses,
            internal: internal_addresses,
        })
    }
}

/// Function to find the index and public (false) or internal (true) type of an Bech32 encoded address
pub async fn search_address(
    secret_manager: &SecretManager,
    bech32_hrp: &str,
    coin_type: u32,
    account_index: u32,
    range: Range<u32>,
    address: &Address,
) -> Result<(u32, bool)> {
    let addresses = GetAddressesBuilder::new(secret_manager)
        .with_coin_type(coin_type)
        .with_account_index(account_index)
        .with_range(range.clone())
        .get_all_raw()
        .await?;
    for index in 0..addresses.public.len() {
        if addresses.public[index] == *address {
            return Ok((range.start + index as u32, false));
        }
        if addresses.internal[index] == *address {
            return Ok((range.start + index as u32, true));
        }
    }
    Err(crate::error::Error::InputAddressNotFound {
        address: address.to_bech32(bech32_hrp),
        range: format!("{range:?}"),
    })
}
