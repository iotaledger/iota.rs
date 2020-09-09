use crate::{Client, Error, Result};

use bee_signing_ext::binary::{BIP32Path, Ed25519Seed as Seed};

/// Builder of get_balance API
pub struct GetBalanceBuilder<'a> {
    client: &'a Client,
    seed: &'a Seed,
    path: Option<BIP32Path>,
    index: Option<usize>,
}

impl<'a> GetBalanceBuilder<'a> {
    /// Create get_balance builder
    pub fn new(client: &'a Client, seed: &'a Seed) -> Self {
        Self {
            client,
            seed,
            path: None,
            index: None,
        }
    }

    /// Set path to the builder
    pub fn path(mut self, path: BIP32Path) -> Self {
        self.path = Some(path);
        self
    }

    /// Set index to the builder
    pub fn index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    /// Consume the builder and get the API result
    pub fn get(self) -> Result<u64> {
        let path = match self.path {
            Some(p) => p.0,
            None => return Err(Error::MissingParameter),
        };

        let mut index = match self.index {
            Some(r) => r,
            None => 0,
        };

        let mut balance = 0;
        loop {
            let addresses = self
                .client
                .get_addresses(self.seed)
                .path(BIP32Path(path.clone()))
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

        Ok(balance)
    }
}
