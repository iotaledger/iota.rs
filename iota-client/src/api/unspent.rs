// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{Client, Error, Result};
use bee_message::prelude::Bech32Address;
use crypto::slip10::Seed;

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
        self.account_index = Some(account_index);
        self
    }

    /// Sets the index of the address to start looking for balance.
    pub fn with_initial_address_index(mut self, initial_address_index: usize) -> Self {
        self.initial_address_index = Some(initial_address_index);
        self
    }

    /// Consume the builder and get the API result
    pub async fn get(self) -> Result<(Bech32Address, usize)> {
        let account_index = self
            .account_index
            .ok_or_else(|| Error::MissingParameter(String::from("account index")))?;

        let mut index = self.initial_address_index.unwrap_or(0);

        let result = loop {
            let addresses = self
                .client
                .get_addresses(self.seed)
                .with_account_index(account_index)
                .with_range(index..index + 20)
                .finish()
                .await?;

            // TODO we assume all addresses are unspent and valid if balance > 0
            let mut address = None;
            for a in addresses {
                let address_balance = self.client.get_address().balance(&a).await?;
                match address_balance.balance {
                    0 => {
                        address = Some(a);
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
