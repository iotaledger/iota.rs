// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{Client, Error, Result};

use crypto::keys::slip10::Seed;

const ADDRESS_GAP_LIMIT: usize = 20;

/// Builder of get_unspent_address API
pub struct GetUnspentAddressBuilder<'a> {
    client: &'a Client,
    seed: &'a Seed,
    account_index: Option<usize>,
    initial_address_index: Option<usize>,
}

impl<'a> GetUnspentAddressBuilder<'a> {
    /// Create get_unspent_address builder
    pub fn new(client: &'a Client, seed: &'a Seed) -> Self {
        Self {
            client,
            seed,
            account_index: None,
            initial_address_index: None,
        }
    }

    /// Sets the account index.
    pub fn with_account_index(mut self, account_index: usize) -> Self {
        self.account_index.replace(account_index);
        self
    }

    /// Sets the index of the address to start looking for balance.
    pub fn with_initial_address_index(mut self, initial_address_index: usize) -> Self {
        self.initial_address_index.replace(initial_address_index);
        self
    }

    /// Consume the builder and get the API result
    pub async fn get(self) -> Result<(String, usize)> {
        let account_index = self.account_index.ok_or(Error::MissingParameter("account index"))?;

        let mut index = self.initial_address_index.unwrap_or(0);

        let result = loop {
            let addresses = self
                .client
                .get_addresses(self.seed)
                .with_account_index(account_index)
                .with_range(index..index + ADDRESS_GAP_LIMIT)
                .finish()
                .await?;

            // TODO we assume all addresses are unspent and valid if balance > 0
            let mut address = None;
            for a in addresses {
                let address_balance = self.client.get_address().balance(&a).await?;
                match address_balance.balance {
                    0 => {
                        address.replace(a);
                        break;
                    }
                    _ => index += 1,
                }
            }

            if let Some(a) = address {
                break (a, index);
            }
        };

        Ok(result)
    }
}
