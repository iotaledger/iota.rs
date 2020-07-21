use crate::error::Result;
use bee_crypto::ternary::Hash;
use iota_conversion::Trinary;

use crate::response::{GetInclusionStatesResponse, GetInclusionStatesResponseBuilder};
use crate::Client;

/// Builder to construct getInclusionStates API
#[derive(Debug)]
pub struct GetInclusionStatesBuilder<'a> {
    client: &'a Client,
    transactions: Vec<String>,
}

impl<'a> GetInclusionStatesBuilder<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
            transactions: Default::default(),
        }
    }

    /// Add list of transaction hashes for which you want to get the inclusion state
    pub fn transactions(mut self, transactions: &[Hash]) -> Self {
        self.transactions = transactions
            .iter()
            .map(|h| h.as_bytes().trytes().unwrap())
            .collect();
        self
    }

    /// Send getInclusionStates request
    pub async fn send(self) -> Result<GetInclusionStatesResponse> {
        let body = json!({
            "command": "getInclusionStates",
            "transactions": self.transactions,
        });

        let client = self.client;
        let res: GetInclusionStatesResponseBuilder = response!(client, body);
        res.build().await
    }
}
