use anyhow::Result;
use iota_bundle_preview::Hash;
use iota_conversion::Trinary;

use crate::response::{GetInclusionStatesResponse, GetInclusionStatesResponseBuilder};
use crate::Client;

/// Builder to construct getInclusionStates API
#[derive(Debug)]
pub struct GetInclusionStatesBuilder {
    transactions: Vec<String>,
    tips: Option<Vec<String>>,
}

impl GetInclusionStatesBuilder {
    pub(crate) fn new() -> Self {
        Self {
            transactions: Default::default(),
            tips: Default::default(),
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

    /// Add list of tip transaction hashes (including milestones) you want to search for
    pub fn tips(mut self, tips: &[Hash]) -> Self {
        self.tips = Some(
            tips.iter()
                .map(|h| h.as_bytes().trytes().unwrap())
                .collect(),
        );
        self
    }

    /// Send getInclusionStates request
    pub async fn send(self) -> Result<GetInclusionStatesResponse> {
        let client = Client::get();
        let mut body = json!({
            "command": "getInclusionStates",
            "transactions": self.transactions,
        });

        if let Some(reference) = self.tips {
            body["tips"] = json!(reference);
        }

        let res: GetInclusionStatesResponseBuilder = response!(client, body);
        res.build().await
    }
}
