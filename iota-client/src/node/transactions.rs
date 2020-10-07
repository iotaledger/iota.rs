use crate::{Client, Result};

use bee_transaction::prelude::{Address, Message, MessageId};

/// Builder of GET /transaction-messages/* endpoint
pub struct GetTransactionsBuilder<'a> {
    _client: &'a Client,
    hashes: Option<&'a [MessageId]>,
    addresses: Option<&'a [Address]>,
    confirmed: bool,
}

impl<'a> GetTransactionsBuilder<'a> {
    /// Create GET /transaction-amessages endpoint builder
    pub fn new(_client: &'a Client) -> Self {
        Self {
            _client,
            hashes: None,
            addresses: None,
            confirmed: true,
        }
    }

    /// Set message hashes to the builder
    pub fn hashes(mut self, hashes: &'a [MessageId]) -> Self {
        self.hashes = Some(hashes);
        self
    }

    /// Set message tags to the builder
    pub fn addresses(mut self, addresses: &'a [Address]) -> Self {
        self.addresses = Some(addresses);
        self
    }

    /// Set message hashes to the builder
    pub fn confirmed(mut self, confirmed: bool) -> Self {
        self.confirmed = confirmed;
        self
    }

    /// Consume the builder and get the API result
    pub fn get(self) -> Result<Vec<Message>> {
        Ok(Vec::new())
    }
}
