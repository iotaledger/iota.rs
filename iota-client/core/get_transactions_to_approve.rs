use crate::error::Result;
use iota_bundle_preview::Hash;
use iota_conversion::Trinary;

use crate::response::{GTTAResponse, GTTAResponseBuilder};
use crate::Client;

/// Builder to construct getTransactionsToApprove API
#[derive(Debug)]
pub struct GetTransactionsToApproveBuilder {
    depth: u8,
    reference: Option<String>,
}

impl GetTransactionsToApproveBuilder {
    pub(crate) fn new() -> Self {
        Self {
            depth: Default::default(),
            reference: Default::default(),
        }
    }

    /// The depth for the random walk in the tip selection
    pub fn depth(mut self, depth: u8) -> Self {
        self.depth = depth;
        self
    }

    /// Add reference hashes
    pub fn reference(mut self, reference: &Hash) -> Self {
        self.reference = Some(reference.as_bytes().trytes().unwrap());
        self
    }

    /// Send getTransactionsToApprove request
    pub async fn send(self) -> Result<GTTAResponse> {
        let mut body = json!({
            "command": "getTransactionsToApprove",
            "depth": self.depth,
        });

        if let Some(reference) = self.reference {
            body["reference"] = json!(reference);
        }

        let res: GTTAResponseBuilder = response!(body);
        res.build().await
    }
}
