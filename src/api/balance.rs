// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{Client, Result};
use crypto::keys::slip10::Seed;

/// Builder of get_balance API
pub struct GetBalanceBuilder<'a> {
    client: &'a Client,
    seed: &'a Seed,
    account_index: usize,
    initial_address_index: usize,
    gap_limit: usize,
}

impl<'a> GetBalanceBuilder<'a> {
    /// Create get_balance builder
    pub fn new(client: &'a Client, seed: &'a Seed) -> Self {
        Self {
            client,
            seed,
            account_index: 0,
            initial_address_index: 0,
            gap_limit: super::ADDRESS_GAP_RANGE,
        }
    }

    /// Sets the account index.
    pub fn with_account_index(mut self, account_index: usize) -> Self {
        self.account_index = account_index;
        self
    }

    /// Sets the index of the address to start looking for balance.
    pub fn with_initial_address_index(mut self, initial_address_index: usize) -> Self {
        self.initial_address_index = initial_address_index;
        self
    }

    /// Sets the gap limit to specify how many addresses will be checked each round.
    /// If gap_limit amount of addresses in a row have no balance the function will return.
    pub fn with_gap_limit(mut self, gap_limit: usize) -> Self {
        self.gap_limit = gap_limit;
        self
    }

    /// Consume the builder and get the API result
    pub async fn finish(self) -> Result<u64> {
        let mut index = self.initial_address_index;

        // get account balance and check with value
        let mut balance = 0;
        // Count addresses with zero balances in a row
        let mut found_zero_balance = 0;
        loop {
            let addresses = self
                .client
                .get_addresses(self.seed)
                .with_account_index(self.account_index)
                .with_range(index..index + self.gap_limit)
                .get_all()
                .await?;

            for (address, _) in addresses {
                let address_balance = self.client.get_address().balance(&address).await?;
                match address_balance.balance {
                    0 => found_zero_balance += 1,
                    _ => {
                        balance += address_balance.balance;
                        // reset
                        found_zero_balance = 0;
                    }
                }
            }
            // The gap limit is 20 and use reference 40 here because there's public and internal addresses
            if found_zero_balance >= self.gap_limit * 2 {
                break;
            }
            index += self.gap_limit;
        }

        Ok(balance)
    }
}
