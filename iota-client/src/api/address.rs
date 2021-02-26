// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{Client, Error, Result};

use bee_message::prelude::{Address, Bech32Address, Ed25519Address};
use core::convert::TryInto;
use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    slip10::{Chain, Curve, Seed},
};
use std::ops::Range;

/// Builder of get_addresses API
pub struct GetAddressesBuilder<'a> {
    client: Option<&'a Client>,
    seed: Option<&'a Seed>,
    account_index: usize,
    range: Range<usize>,
    bech32_hrp: Option<String>,
}

impl<'a> Default for GetAddressesBuilder<'a> {
    fn default() -> Self {
        Self {
            client: None,
            seed: None,
            account_index: 0,
            range: 0..20,
            bech32_hrp: None,
        }
    }
}

impl<'a> GetAddressesBuilder<'a> {
    /// Create get_addresses builder
    pub fn new(seed: &'a Seed) -> Self {
        Self {
            seed: Some(seed),
            ..Default::default()
        }
    }

    /// Provide a client to get the bech32_hrp from the node
    pub fn with_client(mut self, client: &'a Client) -> Self {
        self.client = Some(client);
        self
    }

    /// Set the account index
    pub fn with_account_index(mut self, account_index: usize) -> Self {
        self.account_index = account_index;
        self
    }

    /// Set range to the builder
    pub fn with_range(mut self, range: Range<usize>) -> Self {
        self.range = range;
        self
    }

    /// Set bech32 human readable part (hrp)
    pub fn with_bech32_hrp(mut self, bech32_hrp: String) -> Self {
        self.bech32_hrp = Some(bech32_hrp);
        self
    }

    /// Consume the builder and get a vector of public Bech32Addresses
    pub async fn finish(self) -> Result<Vec<Bech32Address>> {
        Ok(self
            .get_all()
            .await?
            .into_iter()
            .filter(|(_, internal)| !internal)
            .map(|(a, _)| a)
            .collect::<Vec<Bech32Address>>())
    }

    /// Consume the builder and get the vector of Bech32Addresses
    pub async fn get_all(self) -> Result<Vec<(Bech32Address, bool)>> {
        let mut addresses = Vec::new();
        let bech32_hrp = match self.bech32_hrp {
            Some(bech32_hrp) => bech32_hrp,
            None => {
                self.client
                    .ok_or_else(|| Error::MissingParameter(String::from("Client or bech32_hrp")))?
                    .get_bech32_hrp()
                    .await?
            }
        };
        for address_index in self.range {
            let address = generate_address(
                &self.seed.unwrap(),
                self.account_index as u32,
                address_index as u32,
                false,
            )?;
            let internal_address = generate_address(
                &self.seed.unwrap(),
                self.account_index as u32,
                address_index as u32,
                true,
            )?;
            addresses.push((Bech32Address(address.to_bech32(&bech32_hrp)), false));
            addresses.push((Bech32Address(internal_address.to_bech32(&bech32_hrp)), true));
        }

        Ok(addresses)
    }
}

fn generate_address(seed: &Seed, account_index: u32, address_index: u32, internal: bool) -> Result<Address> {
    let chain = Chain::from_u32_hardened(vec![44, 4218, account_index, internal as u32, address_index]);
    let public_key = seed
        .derive(Curve::Ed25519, &chain)?
        .secret_key()?
        .public_key()
        .to_compressed_bytes();
    // Hash the public key to get the address
    let result = Blake2b256::digest(&public_key);

    Ok(Address::Ed25519(Ed25519Address::new(result.try_into().unwrap())))
}

/// Function to find the index and public or internal type of an Bech32 encoded address
pub async fn search_address(
    seed: &Seed,
    bech32_hrp: String,
    account_index: usize,
    range: Range<usize>,
    address: &Bech32Address,
) -> Result<(usize, bool)> {
    let addresses = GetAddressesBuilder::new(&seed)
        .with_bech32_hrp(bech32_hrp)
        .with_account_index(account_index)
        .with_range(range.clone())
        .get_all()
        .await?;
    let mut index_counter = 0;
    for address_internal in addresses {
        if address_internal.0 == *address {
            return Ok((index_counter, address_internal.1));
        }
        if !address_internal.1 {
            index_counter += 1;
        }
    }
    Err(crate::error::Error::InputAddressNotFound(format!("{:?}", range)))
}
