//! Quorum module is a extension to iota client instance which can make sure result from API calls are verified by node
//! pool and guarantee to staisfy minimum quorum threshold.

use crate::response::*;
use crate::Client;

use iota_bundle_preview::{Address, TransactionField};
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

/// Checks if an address was ever withdrawn from, either in the current epoch or in any previous epochs.
/// If an address has a pending transaction, it's also considered 'spent'.
/// # Parameters
/// * [`address`] - addresses to check (do not include the checksum)
///
/// [`address`]: ../core/struct.WereAddressesSpentFromBuilder.html#method.address
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
