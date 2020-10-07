use crate::{Client, Error, Result};

use bee_signing_ext::{binary::BIP32Path, Seed};
use bee_transaction::prelude::Address;

/// Builder of get_unspent_address API
pub struct GetUnspentAddressBuilder<'a> {
    client: &'a Client,
    seed: &'a Seed,
    path: Option<&'a BIP32Path>,
    index: Option<usize>,
}

impl<'a> GetUnspentAddressBuilder<'a> {
    /// Create get_unspent_address builder
    pub fn new(client: &'a Client, seed: &'a Seed) -> Self {
        Self {
            client,
            seed,
            path: None,
            index: None,
        }
    }

    /// Set path to the builder
    pub fn path(mut self, path: &'a BIP32Path) -> Self {
        self.path = Some(path);
        self
    }

    /// Set index to the builder
    pub fn index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    /// Consume the builder and get the API result
    pub fn get(self) -> Result<(Address, usize)> {
        let path = match self.path {
            Some(p) => p,
            None => return Err(Error::MissingParameter),
        };

        let mut index = match self.index {
            Some(r) => r,
            None => 0,
        };

        let address = loop {
            let addresses = self
                .client
                .get_addresses(self.seed)
                .path(path)
                .range(index..index + 20)
                .get()?;

            let outputs = self.client.get_outputs().addresses(&addresses).get()?;

            let mut address = None;
            for output in outputs {
                match output.spent {
                    true => {
                        if output.amount != 0 {
                            return Err(Error::SpentAddress);
                        }
                    }
                    false => {
                        address = Some(output.address);
                        break;
                    }
                }
                index += 1;
            }

            if let Some(a) = address {
                break (a, index);
            }
        };

        Ok(address)
    }
}
