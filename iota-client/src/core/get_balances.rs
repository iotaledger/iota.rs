use crate::error::Result;
use bee_ternary::T3B1Buf;
use bee_transaction::bundled::{Address, BundledTransactionField};
use crypto::hashes::ternary::Hash;

use crate::response::{GetBalancesResponse, GetBalancesResponseBuilder};
use crate::Client;

/// Builder to construct getBalances API
#[derive(Debug)]
pub struct GetBalancesBuilder<'a> {
    client: &'a Client,
    addresses: Vec<String>,
    tips: Option<Vec<String>>,
}

impl<'a> GetBalancesBuilder<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
            addresses: Default::default(),
            tips: Default::default(),
        }
    }

    /// Add address for which to get the balance (do not include the checksum)
    pub fn addresses(mut self, addresses: &[Address]) -> Self {
        self.addresses = addresses
            .iter()
            .map(|h| {
                h.to_inner()
                    .encode::<T3B1Buf>()
                    .iter_trytes()
                    .map(char::from)
                    .collect::<String>()
            })
            .collect();
        self
    }

    /// Add tips whose history of transactions to traverse to find the balance
    pub fn tips(mut self, tips: &[Hash]) -> Self {
        self.tips = Some(
            tips.iter()
                .map(|h| {
                    (*h).encode::<T3B1Buf>()
                        .iter_trytes()
                        .map(char::from)
                        .collect::<String>()
                })
                .collect(),
        );
        self
    }

    /// Send getBalances request
    pub async fn send(self) -> Result<GetBalancesResponse> {
        let mut body = json!({
            "command": "getBalances",
            "addresses": self.addresses,
        });

        if let Some(reference) = self.tips {
            body["tips"] = json!(reference);
        }

        let client = self.client;
        let res: GetBalancesResponseBuilder = response!(client, body);
        res.build().await
    }
}
