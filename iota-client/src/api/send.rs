use crate::{Client, Error, Result, Output, Transfers};

use bee_signing_ext::binary::{BIP32Path, Ed25519Seed as Seed};
use bee_transaction::prelude::Hash;

/// Builder of send API
pub struct SendBuilder<'a> {
    client: &'a Client,
    seed: &'a Seed,
    path: Option<&'a BIP32Path>,
    index: Option<usize>,
    transfers: Option<Transfers>,
}

impl<'a> SendBuilder<'a> {
    /// Create sned builder
    pub fn new(client: &'a Client, seed: &'a Seed) -> Self {
        Self {
            client,
            seed,
            path: None,
            index: None,
            transfers: None,
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

    /// Set transfers to the builder
    pub fn transfers(mut self, transfers: Transfers) -> Self {
        self.transfers = Some(transfers);
        self
    }

    /// Consume the builder and get the API result
    pub fn post(self) -> Result<Vec<Hash>> {
        let path = match self.path {
            Some(p) => p,
            None => return Err(Error::MissingParameter),
        };

        let mut index = match self.index {
            Some(r) => r,
            None => 0,
        };

        let transfers = match self.transfers {
            Some(t) => t,
            None => return Err(Error::MissingParameter),
        };

        let mut balance = 0;
        let mut inputs = Vec::new();
        loop {
            let addresses = self
                .client
                .get_addresses(self.seed)
                .path(path)
                .range(index..index + 20)
                .get()?;

            let outputs = self.client.get_outputs().addresses(&addresses).get()?;

            let mut end = false;
            for output in outputs {
                match output.spent {
                    true => {
                        if output.amount != 0 {
                            return Err(Error::SpentAddress);
                        }
                    }
                    false => {
                        if output.amount != 0 {
                            balance += output.amount;
                            inputs.push(output);
                        } else {
                            end = true;
                        }
                    }
                }
            }

            match end {
                true => break,
                false => index += 20,
            }
        }

        // TODO build the transaction
        self.client.post_messages(Vec::new())
    }
}
