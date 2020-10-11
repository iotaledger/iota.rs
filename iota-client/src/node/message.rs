use crate::{Client, Result, Response, MessageIds};

use bee_transaction::atomic::{Message, MessageId};

/// Builder of GET /api/v1/message/{messageId} endpoint
pub struct GetMessageBuilder<'a> {
    client: &'a Client,
}

impl<'a> GetMessageBuilder<'a> {
    /// Create GET /api/v1/message endpoint builder
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
        }
    }

    /// GET /api/v1/message endpoint
    /// Consume the builder and search for messages matching the index
    pub async fn index(self, index: &str) -> Result<Box<[MessageId]>> {
        let mut url = self.client.get_node()?;
        url.set_path("api/v1/messages");
        url.set_query(Some(&format!("index={}", index)));

        let r = self.client.client.get(url).send().await?.json::<Response<MessageIds>>().await?;
        Ok(r.data.inner)
    }

    /// Consume the builder and find a message by its identifer. This method returns the given message object.
    pub fn data(self, message_id: &MessageId) -> Result<Message> {
        let mut url = self.client.get_node()?;
        url.set_path(&format!("api/v1/messages/{}", message_id));

        todo!()
    }

    /// Consume the builder and find a message by its identifer. This method returns the given message metadata.
    pub fn metadata(self) -> Result<Message> {
        todo!()
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
