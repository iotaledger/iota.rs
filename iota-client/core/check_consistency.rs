use anyhow::Result;
use bee_bundle::Hash;
use iota_conversion::Trinary;

use crate::response::{ConsistencyResponse, ConsistencyResponseBuilder};
use crate::Client;

/// Builder to construct checkConsistency API
#[derive(Debug)]
pub struct CheckConsistencyBuilder<'a> {
    client: &'a Client<'a>,
    tails: Vec<String>,
}

impl<'a> CheckConsistencyBuilder<'a> {
    pub(crate) fn new(client: &'a Client<'a>) -> Self {
        Self {
            client,
            tails: Default::default(),
        }
    }

    /// Add Transaction hashes to check
    pub fn tails(mut self, tails: &[Hash]) -> Self {
        self.tails = tails
            .iter()
            .map(|h| h.as_bytes().trytes().unwrap())
            .collect();
        self
    }

    /// Send checkConsistency request
    pub async fn send(self) -> Result<ConsistencyResponse> {
        let client = self.client;
        let body = json!({
            "command": "checkConsistency",
            "tails": self.tails,
        });

        let res: ConsistencyResponseBuilder = response!(client, body);
        res.build().await
    }
}
