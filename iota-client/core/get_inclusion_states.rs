use crate::error::Result;
use bee_crypto::ternary::Hash;
use iota_conversion::Trinary;

use crate::response::{GetInclusionStatesResponse, GetInclusionStatesResponseBuilder};
use crate::Client;

/// Builder to construct getInclusionStates API
#[derive(Debug)]
pub struct GetInclusionStatesBuilder {
    transactions: Vec<String>,
}

impl GetInclusionStatesBuilder {
    pub(crate) fn new() -> Self {
        Self {
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

        let res: GetInclusionStatesResponseBuilder = response!(body);
        res.build().await
    }
}
