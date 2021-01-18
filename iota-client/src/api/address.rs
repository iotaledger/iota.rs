// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{builder::Network, Client, Error, Result};

use bee_message::prelude::{Address, Bech32Address, Ed25519Address};
use bee_signing_ext::{
    binary::{BIP32Path, Ed25519PrivateKey, Ed25519Seed},
    Seed,
};
use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use core::convert::TryInto;
use std::ops::Range;

const HARDEND: u32 = 1 << 31;

/// Builder of find_addresses API
pub struct GetAddressesBuilder<'a> {
    _client: &'a Client,
    seed: &'a Seed,
    account_index: Option<usize>,
    range: Option<Range<usize>>,
}

impl<'a> GetAddressesBuilder<'a> {
    /// Create find_addresses builder
    pub fn new(_client: &'a Client, seed: &'a Seed) -> Self {
        Self {
            _client,
            seed,
            account_index: None,
            range: None,
        }
    }

    /// Sets the account index.
    pub fn with_account_index(mut self, account_index: usize) -> Self {
        self.account_index = Some(account_index);
        self
    }

    /// Set range to the builder
    pub fn with_range(mut self, range: Range<usize>) -> Self {
        self.range = Some(range);
        self
    }

    /// Consume the builder and get a vector of public Bech32Addresses
    pub fn finish(self) -> Result<Vec<Bech32Address>> {
        Ok(self
            .get_all()?
            .into_iter()
            .filter(|(_, internal)| !internal)
            .map(|(a, _)| a)
            .collect::<Vec<Bech32Address>>())
    }

    /// Consume the builder and get the vector of Bech32Address
    pub fn get_all(self) -> Result<Vec<(Bech32Address, bool)>> {
        let mut path = self
            .account_index
            .map(|i| BIP32Path::from_str(&crate::account_path!(i)).expect("invalid account index"))
            .ok_or_else(|| Error::MissingParameter(String::from("account index")))?;

        let range = match self.range {
            Some(r) => r,
            None => 0..20,
        };

        let seed = match self.seed {
            Seed::Ed25519(s) => s,
            _ => panic!("Other seed scheme isn't supported yet."),
        };

        let mut addresses = Vec::new();
        for i in range {
            let address = generate_address(&seed, &mut path, i, false);
            let internal_address = generate_address(&seed, &mut path, i, true);
            let bech32_hrp = self._client.get_network_info().bech32_hrp;
            addresses.push((Bech32Address(address.to_bech32(&bech32_hrp)), false));
            addresses.push((Bech32Address(internal_address.to_bech32(&bech32_hrp)), true));
        }

        Ok(addresses)
    }
}

fn generate_address(seed: &Ed25519Seed, path: &mut BIP32Path, index: usize, internal: bool) -> Address {
    path.push(internal as u32 + HARDEND);
    path.push(index as u32 + HARDEND);

    let public_key = Ed25519PrivateKey::generate_from_seed(seed, &path)
        .expect("Invalid Seed & BIP32Path. Probably because the index of path is not hardened.")
        .generate_public_key()
        .to_bytes();
    // Hash the public key to get the address
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(public_key);
    let mut result: [u8; 32] = [0; 32];
    hasher.finalize_variable(|res| {
        result = res.try_into().expect("Invalid Length of Public Key");
    });

    path.pop();
    path.pop();

    Address::Ed25519(Ed25519Address::new(result))
}

/// Function to find the index and public or internal type of an Bech32 encoded address
pub fn search_address(
    seed: &Seed,
    account_index: usize,
    range: Range<usize>,
    address: &Bech32Address,
) -> Result<(usize, bool)> {
    let iota = Client::build().with_node("http://0.0.0.0:14265")?.finish()?;
    let addresses = iota
        .find_addresses(&seed)
        .with_account_index(account_index)
        .with_range(range)
        .get_all()?;
    let mut index_counter = 0;
    for address_internal in addresses {
        if address_internal.0 == *address {
            return Ok((index_counter, address_internal.1));
        }
        if !address_internal.1 {
            index_counter += 1;
        }
    }
    Err(crate::error::Error::AddressNotFound)
}
