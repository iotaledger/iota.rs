// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{node::OutputsOptions, signing::SignerHandle, Client, Result};

/// Builder of get_unspent_address API
pub struct GetUnspentAddressBuilder<'a> {
    client: &'a Client,
    signer: &'a SignerHandle,
    account_index: Option<u32>,
    initial_address_index: Option<u32>,
}

impl<'a> GetUnspentAddressBuilder<'a> {
    /// Create get_unspent_address builder
    pub fn new(client: &'a Client, signer: &'a SignerHandle) -> Self {
        Self {
            client,
            signer,
            account_index: None,
            initial_address_index: None,
        }
    }

    /// Sets the account index.
    pub fn with_account_index(mut self, account_index: u32) -> Self {
        self.account_index.replace(account_index);
        self
    }

    /// Sets the index of the address to start looking for balance.
    pub fn with_initial_address_index(mut self, initial_address_index: u32) -> Self {
        self.initial_address_index.replace(initial_address_index);
        self
    }

    /// Consume the builder and get the API result
    pub async fn get(self) -> Result<(String, u32)> {
        let account_index = self.account_index.unwrap_or(0);

        let mut index = self.initial_address_index.unwrap_or(0);

        let result = loop {
            let addresses = self
                .client
                .get_addresses(self.signer)
                .with_account_index(account_index)
                .with_range(index..index + super::ADDRESS_GAP_RANGE)
                .finish()
                .await?;

            let mut address = None;
            for a in addresses {
                let address_outputs = self
                    .client
                    .get_address()
                    .outputs(
                        &a,
                        OutputsOptions {
                            include_spent: true,
                            output_type: None,
                        },
                    )
                    .await?;
                if address_outputs.is_empty() {
                    address.replace(a);
                    break;
                } else {
                    index += 1;
                }
            }

            if let Some(a) = address {
                break (a, index);
            }
        };

        Ok(result)
    }
}
