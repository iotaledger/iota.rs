//! Quorum module is a extension to iota client instance which can make sure result from API calls are verified by node
//! pool and guarantee to staisfy minimum quorum threshold.

use crate::response::*;
use crate::Client;

use iota_bundle_preview::{Address, Hash, TransactionField};
use iota_conversion::Trinary;

use std::collections::HashMap;

use anyhow::Result;

macro_rules! get_node_pool {
    () => {
        Client::get()
            .pool
            .clone()
            .read()
            .map_err(|_| anyhow!("Node pool read poinsened"))?
    };
}

/// Gets the confirmed balance of an address.
/// If the tips parameter is missing, the returned balance is correct as of the latest confirmed milestone.
/// This endpoint returns data only if the node is synchronized.
/// # Parameters
/// * [`addresses`] - Address for which to get the balance (do not include the checksum)
/// * [`threshold`] - (Optional) Confirmation threshold between 0 and 100, default is 100.
/// * [`tips`] - (Optional) Tips whose history of transactions to traverse to find the balance
///
/// [`addresses`]: struct.GetBalancesBuilder.html#method.addresses
/// [`threshold`]: struct.GetBalancesBuilder.html#method.threshold
/// [`tips`]: struct.GetBalancesBuilder.html#method.tips
pub fn get_balances() -> GetBalancesBuilder {
    GetBalancesBuilder::new()
}

/// Builder to construct getBalances API
#[derive(Debug)]
pub struct GetBalancesBuilder {
    addresses: Vec<String>,
    threshold: u8,
    tips: Option<Vec<String>>,
}

impl GetBalancesBuilder {
    pub(crate) fn new() -> Self {
        Self {
            addresses: Default::default(),
            threshold: 100,
            tips: Default::default(),
        }
    }

    /// Add address for which to get the balance (do not include the checksum)
    pub fn addresses(mut self, addresses: &[Address]) -> Self {
        self.addresses = addresses
            .iter()
            .map(|h| h.to_inner().as_i8_slice().trytes().unwrap())
            .collect();
        self
    }

    /// Set confirmation threshold between 0 and 100
    pub fn threshold(mut self, threshold: u8) -> Self {
        self.threshold = threshold;
        self
    }

    /// Add tips whose history of transactions to traverse to find the balance
    pub fn tips(mut self, tips: &[Hash]) -> Self {
        self.tips = Some(
            tips.iter()
                .map(|h| h.as_bytes().trytes().unwrap())
                .collect(),
        );
        self
    }

    /// Send getBalances request
    pub async fn send(self) -> Result<GetBalancesResponse> {
        let client = Client::get();
        let mut body = json!({
            "command": "getBalances",
            "addresses": self.addresses,
            "threshold": self.threshold,
        });

        if let Some(reference) = self.tips {
            body["tips"] = json!(reference);
        }

        let mut result = HashMap::new();
        for node in get_node_pool!().iter() {
            let node = node.clone();
            let res: GetBalancesResponseBuilder = response!(client, body, node);
            let res = res.build().await?;
            let counters = result.entry(res).or_insert(0);
            *counters += 1;
        }

        Ok(result
            .into_iter()
            .max_by_key(|v| v.1)
            .ok_or(anyhow!("Fail to find quorum result"))?
            .0)
    }
}

/// Checks if an address was ever withdrawn from, either in the current epoch or in any previous epochs.
/// If an address has a pending transaction, it's also considered 'spent'.
/// # Parameters
/// * `address` - addresses to check (do not include the checksum)
pub async fn were_addresses_spent_from(
    addresses: &[Address],
) -> Result<WereAddressesSpentFromResponse> {
    let addresses: Vec<String> = addresses
        .iter()
        .map(|h| h.to_inner().as_i8_slice().trytes().unwrap())
        .collect();
    let client = Client::get();
    let body = json!({
        "command": "wereAddressesSpentFrom",
        "addresses": addresses,
    });

    let mut result = HashMap::new();
    for node in get_node_pool!().iter() {
        let node = node.clone();
        let res: WereAddressesSpentFromResponseBuilder = response!(client, body, node);
        let res = res.build().await?;
        let counters = result.entry(res).or_insert(0);
        *counters += 1;
    }

    Ok(result
        .into_iter()
        .max_by_key(|v| v.1)
        .ok_or(anyhow!("Fail to find quorum result"))?
        .0)
}
