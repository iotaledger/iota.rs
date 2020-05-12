use anyhow::Result;
use iota_bundle_preview::Hash;
use iota_conversion::Trinary;

use crate::response::{ConsistencyResponse, ConsistencyResponseBuilder};
use crate::Client;

/// Builder to construct checkConsistency API
#[derive(Debug)]
pub struct CheckConsistencyBuilder<'a> {
    client: &'a Client,
    tails: Vec<String>,
}

impl<'a> CheckConsistencyBuilder<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
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
