// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// gets the BIP32 account path from a given account_index/address_internal/address_index
#[macro_export]
macro_rules! account_path {
    ($account_index:expr) => {
        format!("m/44'/4218'/{}'", $account_index)
    };
}

use crate::{chrysalis2::Seed, Error, Result};
// use bee_crypto::ternary::sponge::{Kerl, Sponge};
// use bee_ternary::{b1t6, T1B1Buf, T3B1Buf, Trits, TryteBuf};
// use bee_transaction::bundled::{Address as TryteAddress, BundledTransactionField};
use bee_message::prelude::{Address, Ed25519Address};

use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use core::convert::TryInto;
use slip10::BIP32Path;
use std::ops::Range;
use std::str::FromStr;

const HARDEND: u32 = 1 << 31;

/// Builder of find_addresses API
pub struct GetAddressesBuilder<'a> {
    seed: &'a Seed,
    account_index: Option<usize>,
    range: Option<Range<usize>>,
}

impl<'a> GetAddressesBuilder<'a> {
    /// Create find_addresses builder
    pub fn new(seed: &'a Seed) -> Self {
        Self {
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

    /// Consume the builder and get a vector of address
    pub fn finish(self) -> Result<Vec<Ed25519Address>> {
        Ok(self
            .get_all()?
            .into_iter()
            .filter(|(_, internal)| !internal)
            .map(|(a, _)| a)
            .collect::<Vec<Ed25519Address>>())
    }

    /// Consume the builder and get the vector of address
    pub fn get_all(self) -> Result<Vec<(Ed25519Address, bool)>> {
        let mut path = self
            .account_index
            .map(|i| BIP32Path::from_str(&crate::account_path!(i)).expect("invalid account index"))
            .ok_or_else(|| Error::ChrysalisAddressError(String::from("missing account index")))?;

        let range = match self.range {
            Some(r) => r,
            None => 0..20,
        };

        let mut addresses = Vec::new();
        for i in range {
            let address = generate_address(&self.seed, &mut path, i, false)?;
            let internal_address = generate_address(&self.seed, &mut path, i, true)?;
            let Address::Ed25519(address) = address;
            addresses.push((address, false));
            let Address::Ed25519(internal_address) = internal_address;
            addresses.push((internal_address, true));
        }

        Ok(addresses)
    }
}

fn generate_address(
    seed: &Seed,
    path: &mut BIP32Path,
    index: usize,
    internal: bool,
) -> Result<Address> {
    path.push(internal as u32 + HARDEND);
    path.push(index as u32 + HARDEND);

    let public_key = seed
        .generate_private_key(path)?
        .public_key()
        .to_compressed_bytes();
    // Hash the public key to get the address
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(public_key);
    let mut result: [u8; 32] = [0; 32];
    hasher.finalize_variable(|res| {
        result = res.try_into().expect("Invalid Length of Public Key");
    });

    path.pop();
    path.pop();

    Ok(Address::Ed25519(Ed25519Address::new(result)))
}
