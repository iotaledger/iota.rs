use crate::{Client, Error, Result};

use bee_signing_ext::binary::{BIP32Path, Ed25519Seed as Seed};
use bee_transaction::atomic::{
    Hash,
    Message,
    payload::{
        Payload,
        Indexation,
        signed_transaction::Address,
    },
};

use std::num::NonZeroU64;

/// Builder of get_balance API
pub struct GetBalanceBuilder<'a> {
    client: &'a Client,
    seed: &'a Seed,
    path: Option<BIP32Path>,
    address: Option<Address>,
    value: Option<NonZeroU64>,
    index: Option<usize>,
}

impl<'a> GetBalanceBuilder<'a> {
    /// Create get_balance builder
    pub fn new(client: &'a Client, seed: &'a Seed) -> Self {
        Self {
            client,
            seed,
            path: None,
            address: None,
            value: None,
            index: None,
        }
    }

    /// Set path to the builder
    pub fn path(mut self, path: BIP32Path) -> Self {
        self.path = Some(path);
        self
    }

    /// Set address to the builder
    pub fn address(mut self, address: Address) -> Self {
        self.address = Some(address);
        self
    }

    /// Set value to the builder
    pub fn value(mut self, value: NonZeroU64) -> Self {
        self.value = Some(value);
        self
    }

    /// Set index to the builder
    pub fn index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    /// Consume the builder and get the API result
    pub fn get(self) -> Result<Message> {
        let path = match self.path {
            Some(p) => {
                if p.0.len() != 2 {
                    return Err(Error::InvalidParameter(String::from("Must provide BIP32Path with depth of 2")));
                }
                p.0
            },
            None => return Err(Error::MissingParameter),
        };

        let address = match self.address {
            Some(a) => {
                if self.client.get_addresses_balance(&[a])?[0].spent {
                    return Err(Error::InvalidParameter(String::from("Address is already spent")));
                }
            },
            None => return Err(Error::MissingParameter),
        };


        let value = match self.value {
            Some(v) => v.get(),
            None => return Err(Error::MissingParameter),
        };

        let mut index = match self.index {
            Some(r) => r,
            None => 0,
        };

        // get account balance and check with value
        let mut balance = 0;
        let mut inputs = Vec::new();
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

        if balance < value {
            return Err(Error::NotEnoughBalance(balance));
        }

        // get tips 
        let tips = self.client.get_tips()?;

        // TODO building signed_transaction payload
        
        let message = Message {
            trunk: tips.0,
            branch: tips.1,
            payload: Payload::Indexation(Box::new(Indexation{ tag: [0;16] })),
            nonce: 0,
        };
        
        // TODO POW

        Ok(message)
    }
}
