// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{Client, Result, Seed};

/// Builder of get_balance API
pub struct GetBalanceBuilder<'a> {
    client: &'a Client,
    seed: &'a Seed,
    account_index: Option<usize>,
    initial_address_index: Option<usize>,
}

impl<'a> GetBalanceBuilder<'a> {
    /// Create get_balance builder
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
    pub async fn finish(self) -> Result<u64> {
        let account_index = self.account_index.unwrap_or(0);

        let mut index = self.initial_address_index.unwrap_or(0);

        // get account balance and check with value
        let mut balance = 0;
        loop {
            let addresses = self
                .client
                .find_addresses(self.seed)
                .with_account_index(account_index)
                .with_range(index..index + 20)
                .get_all()
                .await?;

            // TODO we assume all addresses are unspent and valid if balance > 0
            let mut found_zero_balance = false;
            for (address, _) in addresses {
                let address_balance = self.client.get_address().balance(&address).await?;
                match address_balance.balance {
                    0 => {
                        found_zero_balance = true;
                        break;
                    }
                    _ => balance += address_balance.balance,
                }
            }

            match found_zero_balance {
                true => break,
                false => index += 20,
            }
        }

        Ok(balance)
    }
}
