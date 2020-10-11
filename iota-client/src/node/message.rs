use crate::{Client, MessageIds, Response, Result, Error, MessageIdHex, MessageMetadata, ChildrenMessageIds};

use bee_transaction::atomic::Message;

/// Builder of GET /api/v1/message/{messageId} endpoint
pub struct GetMessageBuilder<'a> {
    client: &'a Client,
}

impl<'a> GetMessageBuilder<'a> {
    /// Create GET /api/v1/message endpoint builder
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// GET /api/v1/message endpoint
    /// Consume the builder and search for messages matching the index
    pub async fn index(self, index: &str) -> Result<Box<[MessageIdHex]>> {
        let mut url = self.client.get_node()?;
        url.set_path("api/v1/messages");
        url.set_query(Some(&format!("index={}", index)));
        let resp = reqwest::get(url).await?;

        match resp.status().as_u16() {
            200 => {
                let ids = resp.json::<Response<MessageIds>>().await?;
                Ok(ids.data.inner)
            }
            status => Err(Error::ResponseError(status)),
        }
    }

    /// Consume the builder and find a message by its identifer. This method returns the given message object.
    pub fn data(self, message_id: &MessageIdHex) -> Result<Message> {
        todo!()
    }

    /// Consume the builder and find a message by its identifer. This method returns the given message metadata.
    pub async fn metadata(self, message_id: &MessageIdHex) -> Result<MessageMetadata> {
        let mut url = self.client.get_node()?;
        url.set_path(&format!("api/v1/messages/{}/metadata", message_id.0));
        let resp = reqwest::get(url).await?;

        match resp.status().as_u16() {
            200 => {
                let meta = resp.json::<Response<MessageMetadata>>().await?;
                Ok(meta.data)
            }
            status => Err(Error::ResponseError(status)),
        }
    }

    /// Consume the builder and find a message by its identifer. This method returns the given message raw data.
    pub async fn raw(self, message_id: &MessageIdHex) -> Result<String> {
        let mut url = self.client.get_node()?;
        url.set_path(&format!("api/v1/messages/{}/metadata", message_id.0));
        let resp = reqwest::get(url).await?;

        match resp.status().as_u16() {
            200 => Ok(resp.text().await?),
            status => Err(Error::ResponseError(status)),
        }
    }

    /// Consume the builder and returns the list of message IDs that reference a message by its identifier.
    pub async fn children(self, message_id: &MessageIdHex) -> Result<Box<[MessageIdHex]>> {
        let mut url = self.client.get_node()?;
        url.set_path(&format!("api/v1/messages/{}/children", message_id.0));
        let resp = reqwest::get(url).await?;

        match resp.status().as_u16() {
            200 => {
                let meta = resp.json::<Response<ChildrenMessageIds>>().await?;
                Ok(meta.data.inner)
            }
            status => Err(Error::ResponseError(status)),
        }
    }
}
