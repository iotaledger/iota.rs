use crate::{Client, Output, Result};

use bee_transaction::prelude::{Address, Hash};

/// Builder of GET /outputs/* endpoint
pub struct GetOutputsBuilder<'a> {
    _client: &'a Client,
    hashes: Option<&'a [Hash]>,
    addresses: Option<&'a [Address]>,
}

impl<'a> GetOutputsBuilder<'a> {
    /// Create GET /outputs endpoint builder
    pub fn new(_client: &'a Client) -> Self {
        Self {
            _client,
            hashes: None,
            addresses: None,
        }
    }

    /// Set message hashes to the builder
    pub fn hashes(mut self, hashes: &'a [Hash]) -> Self {
        self.hashes = Some(hashes);
        self
    }

    /// Set addresses to the builder
    pub fn addresses(mut self, addresses: &'a [Address]) -> Self {
        self.addresses = Some(addresses);
        self
    }

    /// Consume the builder and get the API result
    pub fn get(self) -> Result<Vec<Output>> {
        Ok(Vec::new())
    }
}
