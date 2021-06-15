use crate::error::*;
use crate::response::InputData;
use crate::Client;
use bee_ternary::T3B1Buf;
use bee_transaction::bundled::Address;
use bee_transaction::bundled::BundledTransactionField;

use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
/// Address can be used to find inputs
pub struct AddressInput {
    /// Input address
    pub address: Address,
    /// Index of the address
    pub index: u64,
    /// Index of the address
    pub security_lvl: u8,
}

/// Builder to get inputs (spent addresses included)
pub struct GetLedgerAccountDataForMigrationBuilder<'a> {
    client: &'a mut Client,
    addresses: Vec<AddressInput>,
}

impl<'a> GetLedgerAccountDataForMigrationBuilder<'a> {
    /// Create a new GetLedgerAccountDataForMigrationBuilder
    pub fn builder(client: &'a mut Client) -> Self {
        Self {
            client,
            addresses: Default::default(),
        }
    }

    /// Set addresses to search for balance
    pub fn with_addresses(mut self, addresses: Vec<AddressInput>) -> Self {
        self.addresses = addresses;
        self
    }

    /// Send GetInputs request and returns the inputs with their total balance, spent status and bundlehashes
    pub async fn finish(self) -> Result<(u64, Vec<InputData>)> {
        let mut total_balance = 0;
        let mut inputs = Vec::new();

        let input_addresses = self.addresses;
        //sync nodes
        self.client.sync().await;

        let addresses_for_api_calls: Vec<Address> =
            input_addresses.iter().map(|a| a.address.clone()).collect();

        // Get balance of the addresses
        let balance_response = if self.client.quorum {
            crate::quorum::get_balances()
                .addresses(&addresses_for_api_calls[..])
                .send(self.client)
                .await?
        } else {
            self.client
                .get_balances()
                .addresses(&addresses_for_api_calls[..])
                .send()
                .await?
        };

        // Get spent status of the addresses
        let spent_status = if self.client.quorum {
            crate::quorum::were_addresses_spent_from(&addresses_for_api_calls[..], self.client)
                .await?
        } else {
            self.client
                .were_addresses_spent_from(&addresses_for_api_calls[..])
                .await?
        };
        // Find bundle hashes for spent addresses
        let mut spent_bundle_hashes = Vec::new();
        for (index, spent) in spent_status.states.iter().enumerate() {
            if *spent {
                let tx_hashes_on_spent_addresses = self
                    .client
                    .find_transactions()
                    .addresses(&[addresses_for_api_calls[index].clone()])
                    .send()
                    .await?
                    .hashes;
                if tx_hashes_on_spent_addresses.is_empty() {
                    spent_bundle_hashes.push(None);
                    continue;
                }
                let txs_on_spent_addresses = self
                    .client
                    .get_trytes(&tx_hashes_on_spent_addresses)
                    .await?
                    .trytes;
                let mut known_bundle_hashes = HashSet::new();
                for tx in txs_on_spent_addresses {
                    if *tx.value().to_inner() < 0 {
                        known_bundle_hashes.insert(*tx.bundle());
                    }
                }
                let known_bundle_hashes: Vec<String> = known_bundle_hashes
                    .into_iter()
                    .map(|b| {
                        b.to_inner()
                            .encode::<T3B1Buf>()
                            .iter_trytes()
                            .map(char::from)
                            .collect::<String>()
                    })
                    .collect();
                // Push None so the order stays correct
                if known_bundle_hashes.is_empty() {
                    spent_bundle_hashes.push(None);
                } else {
                    spent_bundle_hashes.push(Some(known_bundle_hashes))
                }
            } else {
                // Push None so the order stays correct
                spent_bundle_hashes.push(None);
            }
        }

        //Iterate over each balance address output here
        for (index, balance) in balance_response.balances.into_iter().enumerate() {
            if balance == 0 {
                continue;
            }
            inputs.push(InputData {
                address: input_addresses[index].address.clone(),
                security_lvl: input_addresses[index].security_lvl,
                balance,
                index: input_addresses[index].index,
                spent: spent_status.states[index],
                spent_bundlehashes: spent_bundle_hashes[index].clone(),
            });
            total_balance += balance;
        }

        Ok((total_balance, inputs))
    }
}
