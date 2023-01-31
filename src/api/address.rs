// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{Client, Error, Result};

use bee_message::prelude::{Address, Ed25519Address};
use core::convert::TryInto;
use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    keys::slip10::{Chain, Curve, Seed},
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
            range: 0..super::ADDRESS_GAP_RANGE,
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
        self.client.replace(client);
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
        self.bech32_hrp.replace(bech32_hrp);
        self
    }

    /// Consume the builder and get a vector of public addresses bech32 encoded
    pub async fn finish(self) -> Result<Vec<String>> {
        Ok(self
            .get_all()
            .await?
            .into_iter()
            .filter(|(_, internal)| !internal)
            .map(|(a, _)| a)
            .collect())
    }

    /// Consume the builder and get the vector of public and internal addresses bech32 encoded
    pub async fn get_all(self) -> Result<Vec<(String, bool)>> {
        let bech32_hrp = match self.bech32_hrp.clone() {
            Some(bech32_hrp) => bech32_hrp,
            None => {
                self.client
                    .ok_or(Error::MissingParameter("Client or bech32_hrp"))?
                    .get_bech32_hrp()
                    .await?
            }
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
        for address_index in self.range {
            let address = generate_address(
                self.seed.ok_or(Error::MissingParameter("Seed"))?,
                self.account_index as u32,
                address_index as u32,
                false,
            )?;
            let internal_address = generate_address(
                self.seed.ok_or(Error::MissingParameter("Seed"))?,
                self.account_index as u32,
                address_index as u32,
                true,
            )?;
            addresses.push((address, false));
            addresses.push((internal_address, true));
        }

        Ok(addresses)
    }
}

fn generate_address(seed: &Seed, account_index: u32, address_index: u32, internal: bool) -> Result<Address> {
    // 44 is for BIP 44 (HD wallets) and 4218 is the registered index for IOTA https://github.com/satoshilabs/slips/blob/master/slip-0044.md
    let chain = Chain::from_u32_hardened(vec![44, 4218, account_index, internal as u32, address_index]);
    let public_key = seed
        .derive(Curve::Ed25519, &chain)?
        .secret_key()
        .public_key()
        .to_bytes();
    // Hash the public key to get the address
    let result = Blake2b256::digest(&public_key)
        .try_into()
        .map_err(|_e| Error::Blake2b256Error("Hashing the public key while generating the address failed."));

    Ok(Address::Ed25519(Ed25519Address::new(result?)))
}

/// Function to find the index and public or internal type of an Bech32 encoded address
pub async fn search_address(
    seed: &Seed,
    bech32_hrp: &str,
    account_index: usize,
    range: Range<usize>,
    address: &Address,
) -> Result<(usize, bool)> {
    let addresses = GetAddressesBuilder::new(seed)
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
        format!("{range:?}"),
    ))
}
