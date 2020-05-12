use anyhow::Result;
use iota_bundle_preview::Hash;
use iota_conversion::Trinary;

use crate::response::{GetTrytesResponse, GetTrytesResponseBuilder};
use crate::Client;

/// Builder to construct getTrytes API
#[derive(Debug)]
pub struct GetTrytesBuilder<'a> {
    client: &'a Client,
    hashes: Vec<String>,
}

impl<'a> GetTrytesBuilder<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
            hashes: Default::default(),
        }
    }

    /// Add transaction hashes
    pub fn hashes(mut self, hashes: &[Hash]) -> Self {
        self.hashes = hashes
            .iter()
            .map(|h| h.as_bytes().trytes().unwrap())
            .collect();
        self
    }

    /// Send getTrytes request
    pub async fn send(self) -> Result<GetTrytesResponse> {
        let client = self.client;
        let body = json!({
            "command": "getTrytes",
            "hashes": self.hashes,
        });

        let res: GetTrytesResponseBuilder = response!(client, body);
        res.build().await
    }
}
