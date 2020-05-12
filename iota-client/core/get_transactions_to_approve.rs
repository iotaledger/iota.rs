use anyhow::Result;
use iota_bundle_preview::Hash;
use iota_conversion::Trinary;

use crate::response::{GTTAResponse, GTTAResponseBuilder};
use crate::Client;

/// Builder to construct getTransactionsToApprove API
#[derive(Debug)]
pub struct GetTransactionsToApproveBuilder<'a> {
    client: &'a Client,
    depth: u8,
    reference: Option<String>,
}

impl<'a> GetTransactionsToApproveBuilder<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
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
        let client = self.client;
        let mut body = json!({
            "command": "getTransactionsToApprove",
            "depth": self.depth,
        });

        if let Some(reference) = self.reference {
            body["reference"] = json!(reference);
        }

        let res: GTTAResponseBuilder = response!(client, body);
        res.build().await
    }
}
