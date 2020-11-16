use crate::{Client, Error, Result};

use bee_message::prelude::{Address, Ed25519Address};
use bee_signing_ext::{
    binary::{BIP32Path, Ed25519PrivateKey},
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
    path: Option<&'a BIP32Path>,
    range: Option<Range<usize>>,
}

impl<'a> GetAddressesBuilder<'a> {
    /// Create find_addresses builder
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

    /// Consume the builder and get the vector of Address
    pub fn get(self) -> Result<Vec<Address>> {
        let mut path = match self.path {
            Some(p) => p.clone(),
            None => return Err(Error::MissingParameter(String::from("BIP32 path"))),
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
            path.push(i as u32 + HARDEND);
            let public_key = Ed25519PrivateKey::generate_from_seed(seed, &path)
                .expect(
                    "Invalid Seed & BIP32Path. Probably because the index of path is not hardened.",
                )
                .generate_public_key()
                .to_bytes();
            // Hash the public key to get the address
            let mut hasher = VarBlake2b::new(32).unwrap();
            hasher.update(public_key);
            let mut result: [u8; 32] = [0; 32];
            hasher.finalize_variable(|res| {
                result = res.try_into().expect("Invalid Length of Public Key");
            });
            addresses.push(Address::Ed25519(Ed25519Address::new(result)));
            path.pop();
        }

        Ok(addresses)
    }
}
