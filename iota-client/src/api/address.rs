use crate::{Client, Error, Result};

use bee_signing_ext::{
    binary::{BIP32Path, Ed25519PrivateKey},
    Seed,
};
use bee_transaction::prelude::{Address, Ed25519Address};


use std::ops::Range;

/// Builder of get_addresses API
pub struct GetAddressesBuilder<'a> {
    _client: &'a Client,
    seed: &'a Seed,
    path: Option<&'a BIP32Path>,
    range: Option<Range<usize>>,
}

impl<'a> GetAddressesBuilder<'a> {
    /// Create get_addresses builder
    pub fn new(_client: &'a Client, seed: &'a Seed) -> Self {
        Self {
            _client,
            seed,
            path: None,
            range: None,
        }
    }

    /// Set path to the builder
    pub fn path(mut self, path: &'a BIP32Path) -> Self {
        self.path = Some(path);
        self
    }

    /// Set range to the builder
    pub fn range(mut self, range: Range<usize>) -> Self {
        self.range = Some(range);
        self
    }

    /// Consume the builder and get the API result
    pub fn get(self) -> Result<Vec<Address>> {
        let mut path = match self.path {
            Some(p) => p.clone(),
            None => return Err(Error::MissingParameter),
        };

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
            path.push(i as u32);
            let public_key = Ed25519PrivateKey::generate_from_seed(seed, &path)
                .expect("Invalid Seed & BIP32Path")
                .generate_public_key()
                .to_bytes();
            addresses.push(Address::Ed25519(Ed25519Address::new(public_key)));
            path.pop();
        }

        Ok(addresses)
    }
}
