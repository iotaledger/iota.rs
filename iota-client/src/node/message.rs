// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{ChildrenMessageIds, Client, Error, MessageIds, MessageJson, MessageMetadata, Response, Result};

use bee_message::{Message, MessageId};

use std::convert::TryInto;

/// Builder of GET /api/v1/messages/{messageId} endpoint
pub struct GetMessageBuilder<'a> {
    client: &'a Client,
}

impl<'a> GetMessageBuilder<'a> {
    /// Create GET /api/v1/messages endpoint builder
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// GET /api/v1/messages?index={Index} endpoint
    /// Consume the builder and search for messages matching the index
    pub async fn index(self, index: &str) -> Result<Box<[MessageId]>> {
        let mut url = self.client.get_node()?;
        url.set_path("api/v1/messages");
        url.set_query(Some(&format!("index={}", index)));
        let resp = reqwest::get(url).await?;

        match resp.status().as_u16() {
            200 => {
                let ids = resp.json::<Response<MessageIds>>().await?;
                ids.data
                    .inner
                    .iter()
                    .map(|s| {
                        let mut message_id = [0u8; 32];
                        hex::decode_to_slice(s, &mut message_id)?;
                        Ok(MessageId::from(message_id))
                    })
                    .collect::<Result<Box<[MessageId]>>>()
            }
            status => Err(Error::ResponseError(status)),
        }
    }

    /// GET /api/v1/messages/{messageID} endpoint
    /// Consume the builder and find a message by its identifer. This method returns the given message object.
    pub async fn data(self, message_id: &MessageId) -> Result<Message> {
        let mut url = self.client.get_node()?;
        url.set_path(&format!("api/v1/messages/{}", message_id));
        let resp = reqwest::get(url).await?;

        match resp.status().as_u16() {
            200 => {
                let meta = resp.json::<Response<MessageJson>>().await?;
                Ok(meta.data.try_into()?)
            }
            status => Err(Error::ResponseError(status)),
        }
    }

    /// GET /api/v1/messages/{messageID}/metadata endpoint
    /// Consume the builder and find a message by its identifer. This method returns the given message metadata.
    pub async fn metadata(self, message_id: &MessageId) -> Result<MessageMetadata> {
        let mut url = self.client.get_node()?;
        url.set_path(&format!("api/v1/messages/{}/metadata", message_id));
        let resp = reqwest::get(url).await?;
        match resp.status().as_u16() {
            200 => {
                let meta = resp.json::<Response<MessageMetadata>>().await?;
                Ok(meta.data)
            }
            status => Err(Error::ResponseError(status)),
        }
    }

    /// GET /api/v1/messages/{messageID}/children endpoint
    /// Consume the builder and find a message by its identifer. This method returns the given message raw data.
    pub async fn raw(self, message_id: &MessageId) -> Result<String> {
        let mut url = self.client.get_node()?;
        url.set_path(&format!("api/v1/messages/{}/raw", message_id));
        let resp = reqwest::get(url).await?;

        match resp.status().as_u16() {
            200 => Ok(resp.text().await?),
            status => Err(Error::ResponseError(status)),
        }
    }

    /// Consume the builder and returns the list of message IDs that reference a message by its identifier.
    pub async fn children(self, message_id: &MessageId) -> Result<Box<[MessageId]>> {
        let mut url = self.client.get_node()?;
        url.set_path(&format!("api/v1/messages/{}/children", message_id));
        let resp = reqwest::get(url).await?;

        match resp.status().as_u16() {
            200 => {
                let meta = resp.json::<Response<ChildrenMessageIds>>().await?;
                meta.data
                    .inner
                    .iter()
                    .map(|s| {
                        let mut message_id = [0u8; 32];
                        hex::decode_to_slice(s, &mut message_id)?;
                        Ok(MessageId::from(message_id))
                    })
                    .collect::<Result<Box<[MessageId]>>>()
            }
            status => Err(Error::ResponseError(status)),
        }
    }
}
