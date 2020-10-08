use crate::{Client, Result};

use bee_transaction::atomic::{Message, MessageId};

/// Builder of GET /api/v1/message/{messageId} endpoint
pub struct GetMessageBuilder<'a> {
    _client: &'a Client,
    message_id: &'a MessageId,
}

impl<'a> GetMessageBuilder<'a> {
    /// Create GET /api/v1/message/{messageId} endpoint builder
    pub fn new(_client: &'a Client, message_id: &'a MessageId) -> Self {
        Self {
            _client,
            message_id,
        }
    }

    /// Consume the builder and find a message by its identifer. This method returns the given message object.
    pub fn data(self) -> Result<Vec<Message>> {
        Ok(Vec::new())
    }

    /// Consume the builder and find a message by its identifer. This method returns the given message metadata.
    pub fn metadata(self) -> Result<Vec<Message>> {
        Ok(Vec::new())
    }

    /// Consume the builder and find a message by its identifer. This method returns the given message raw data.
    pub fn raw(self) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }

    /// Consume the builder and returns the list of message IDs that reference a message by its identifier.
    pub fn children(self) -> Result<Vec<MessageId>> {
        Ok(Vec::new())
    }
}
