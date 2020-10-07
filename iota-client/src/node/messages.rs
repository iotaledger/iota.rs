use crate::{Client, Result};

use bee_transaction::atomic::{Message, MessageId};

/// Builder of GET /messages/* endpoint
pub struct GetMessagesBuilder<'a> {
    _client: &'a Client,
    hashes: Option<&'a [MessageId]>,
    tags: Option<&'a [MessageId]>,
    confirmed: bool,
}

impl<'a> GetMessagesBuilder<'a> {
    /// Create GET /messages endpoint builder
    pub fn new(_client: &'a Client) -> Self {
        Self {
            _client,
            hashes: None,
            tags: None,
            confirmed: true,
        }
    }

    /// Set message hashes to the builder
    pub fn hashes(mut self, hashes: &'a [MessageId]) -> Self {
        self.hashes = Some(hashes);
        self
    }

    /// Set message tags to the builder
    pub fn tags(mut self, tags: &'a [MessageId]) -> Self {
        self.tags = Some(tags);
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
