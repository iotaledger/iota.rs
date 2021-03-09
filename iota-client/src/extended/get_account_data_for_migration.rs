use crate::core::AddressBuilder;
use crate::error::*;
use crate::response::InputData;
use crate::Client;
use bee_signing::ternary::seed::Seed;
use bee_ternary::T3B1Buf;
use bee_transaction::bundled::Address;
use bee_transaction::bundled::BundledTransactionField;

use std::collections::HashSet;

/// Builder to get inputs (spent addresses included)
pub struct GetAccountDataForMigrationBuilder<'a> {
    client: &'a Client,
    seed: Option<&'a Seed>,
    start_index: u64,
    gap_limit: u64,
    security_lvl: u8,
}

impl<'a> GetAccountDataForMigrationBuilder<'a> {
    /// Create a new GetAccountDataForMigrationBuilder
    pub fn builder(client: &'a Client) -> Self {
        Self {
            client,
            seed: None,
            start_index: 0,
            gap_limit: 30,
            security_lvl: 2,
        }
    }

    /// Set the seed
    pub fn with_seed(mut self, seed: &'a Seed) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Set address index from which to start searching for balance
    pub fn with_start_index(mut self, start_index: u64) -> Self {
        self.start_index = start_index;
        self
    }

    /// Set gap limit which defines how many addresses we generate each round
    pub fn with_gap_limit(mut self, gap_limit: u64) -> Self {
        self.gap_limit = gap_limit;
        self
    }

    /// Set security level
    pub fn with_security(mut self, security: u8) -> Self {
        self.security_lvl = match security {
            1 => 1,
            2 => 2,
            3 => 3,
            _ => panic!("Invalid security level"),
        };
        self
    }

    /// Send GetInputs request and returns the inputs with their total balance, spent status and bundlehashes
    pub async fn finish(self) -> Result<(u64, Vec<InputData>)> {
        let seed = match self.seed {
            Some(s) => s,
            None => return Err(Error::MissingSeed),
        };

        let mut total_balance = 0;
        let mut inputs = Vec::new();
        // Count addresses with zero balances in a row
        let mut found_zero_balance = 0;
        let mut index = self.start_index;
        loop {
            let range = index..index + self.gap_limit;
            // Generate addresses
            let addresses_with_index = AddressBuilder::builder()
                .with_seed(&seed)
                .with_security(self.security_lvl)
                .with_range(range)
                .finish()
                .unwrap();

            let addresses_for_api_calls: Vec<Address> = addresses_with_index
                .clone()
                .into_iter()
                .map(|(_, address)| address)
                .collect();
            // Get balance of the addresses
            let balance_response = self
                .client
                .get_balances()
                .addresses(&addresses_for_api_calls[..])
                .send()
                .await?;
            // Get spent status of the addresses
            let spent_status = self
                .client
                .were_addresses_spent_from(&addresses_for_api_calls[..])
                .await?;
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
                    found_zero_balance += 1;
                    continue;
                }
                //Reset found_zero_balance
                found_zero_balance = 0;
                inputs.push(InputData {
                    address: addresses_with_index[index].1.clone(),
                    security_lvl: self.security_lvl,
                    balance,
                    index: addresses_with_index[index].0,
                    spent: spent_status.states[index],
                    spent_bundlehashes: spent_bundle_hashes[index].clone(),
                });
                total_balance += balance;
            }
            //Break if we reached gap_limit addresses without balance in a row
            if found_zero_balance >= self.gap_limit {
                break;
            }
            index += self.gap_limit;
        }

        Ok((total_balance, inputs))
    }
}
