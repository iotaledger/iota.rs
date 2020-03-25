use anyhow::Result;
use bee_bundle::Address;
use iota_conversion::Trinary;

use crate::response::{WereAddressesSpentFromResponse, WereAddressesSpentFromResponseBuilder};
use crate::Client;

/// Builder to construct WereAddressesSpentFrom API
#[derive(Debug)]
pub struct WereAddressesSpentFromBuilder<'a> {
    client: &'a Client<'a>,
    addresses: Vec<String>,
}

impl<'a> WereAddressesSpentFromBuilder<'a> {
    pub(crate) fn new(client: &'a Client<'a>) -> Self {
        Self {
            client,
            addresses: Default::default(),
        }
    }

    /// Add addresses (without checksum) to check
    pub fn address(mut self, addresses: &[Address]) -> Self {
        self.addresses = addresses
            .iter()
            .map(|h| h.as_bytes().trytes().unwrap())
            .collect();
        self
    }

    /// Send WereAddressesSpentFrom request
    pub async fn send(self) -> Result<WereAddressesSpentFromResponse> {
        let client = self.client;
        let body = json!({
            "command": "wereAddressesSpentFrom",
            "addresses": self.addresses,
        });

        let res: WereAddressesSpentFromResponseBuilder = response!(client, body);
        res.build().await
    }
}
