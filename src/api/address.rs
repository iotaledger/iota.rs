// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    signing::{mnemonic::IOTA_COIN_TYPE, GenerateAddressMetadata, Network, SignerHandle},
    Client, Error, Result,
};

use bee_message::address::Address;

use std::ops::Range;

/// Builder of get_addresses API
pub struct GetAddressesBuilder<'a> {
    client: Option<&'a Client>,
    signer: Option<&'a SignerHandle>,
    account_index: u32,
    range: Range<u32>,
    bech32_hrp: Option<String>,
    metadata: GenerateAddressMetadata,
}

impl<'a> Default for GetAddressesBuilder<'a> {
    fn default() -> Self {
        Self {
            client: None,
            signer: None,
            account_index: 0,
            range: 0..super::ADDRESS_GAP_RANGE,
            bech32_hrp: None,
            metadata: GenerateAddressMetadata {
                syncing: true,
                network: Network::Testnet,
            },
        }
    }
}

impl<'a> GetAddressesBuilder<'a> {
    /// Create get_addresses builder
    pub fn new(signer: &'a SignerHandle) -> Self {
        Self {
            signer: Some(signer),
            ..Default::default()
        }
    }

    /// Provide a client to get the bech32_hrp from the node
    pub fn with_client(mut self, client: &'a Client) -> Self {
        self.client.replace(client);
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

    /// Set bech32 human readable part (hrp)
    pub fn with_bech32_hrp(mut self, bech32_hrp: String) -> Self {
        self.bech32_hrp.replace(bech32_hrp);
        self
    }

    /// Set the metadata for the address generation (used for ledger to display addresses or not)
    pub fn with_generate_metadata(mut self, metadata: GenerateAddressMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Consume the builder and get a vector of public addresses bech32 encoded
    pub async fn finish(self) -> Result<Vec<String>> {
        let bech32_hrp = match self.bech32_hrp.clone() {
            Some(bech32_hrp) => bech32_hrp,
            None => match self.client {
                Some(client) => client.get_bech32_hrp().await?,
                None => "iota".to_string(),
            },
        };
        let mut addresses = Vec::new();
        let signer = self.signer.ok_or(Error::MissingParameter("signer"))?;
        let mut signer = signer.lock().await;
        for address_index in self.range {
            let address = signer
                .generate_address(4219, self.account_index, address_index, false, self.metadata.clone())
                .await?;
            addresses.push(address.to_bech32(&bech32_hrp));
        }

        Ok(addresses)
    }

    /// Consume the builder and get the vector of public and internal addresses bech32 encoded
    pub async fn get_all(self) -> Result<Vec<(String, bool)>> {
        let bech32_hrp = match self.bech32_hrp.clone() {
            Some(bech32_hrp) => bech32_hrp,
            None => match self.client {
                Some(client) => client.get_bech32_hrp().await?,
                None => "iota".to_string(),
            },
        };
        let addresses = self
            .get_all_raw()
            .await?
            .into_iter()
            .map(|(a, b)| (a.to_bech32(&bech32_hrp), b))
            .collect();

        Ok(addresses)
    }

    /// Consume the builder and get the vector of public and internal addresses
    pub async fn get_all_raw(self) -> Result<Vec<(Address, bool)>> {
        let mut addresses = Vec::new();
        let signer = self.signer.ok_or(Error::MissingParameter("signer"))?;
        let mut signer = signer.lock().await;
        for address_index in self.range {
            let address = signer
                .generate_address(
                    IOTA_COIN_TYPE,
                    self.account_index,
                    address_index,
                    false,
                    self.metadata.clone(),
                )
                .await?;

            let internal_address = signer
                .generate_address(
                    IOTA_COIN_TYPE,
                    self.account_index,
                    address_index,
                    true,
                    self.metadata.clone(),
                )
                .await?;
            addresses.push((address, false));
            addresses.push((internal_address, true));
        }

        Ok(addresses)
    }
}

/// Function to find the index and public or internal type of an Bech32 encoded address
pub async fn search_address(
    signer: &SignerHandle,
    bech32_hrp: &str,
    account_index: u32,
    range: Range<u32>,
    address: &Address,
) -> Result<(u32, bool)> {
    let addresses = GetAddressesBuilder::new(signer)
        .with_bech32_hrp(bech32_hrp.to_owned())
        .with_account_index(account_index)
        .with_range(range.clone())
        .get_all()
        .await?;
    let mut index_counter = range.start;
    for address_internal in addresses {
        if address_internal.0 == *address.to_bech32(bech32_hrp) {
            return Ok((index_counter, address_internal.1));
        }
        if !address_internal.1 {
            index_counter += 1;
        }
    }
    Err(crate::error::Error::InputAddressNotFound(
        address.to_bech32(bech32_hrp),
        format!("{:?}", range),
    ))
}
