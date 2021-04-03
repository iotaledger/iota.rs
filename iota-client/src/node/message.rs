// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{Api, Client, Result};
use bee_message::{Message, MessageId};
use bee_rest_api::types::{
    dtos::MessageDto,
    responses::{MessageChildrenResponse, MessageMetadataResponse as MessageMetadata, MessagesFindResponse},
};

use std::convert::TryFrom;

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
    pub async fn index<I: AsRef<[u8]>>(self, index: I) -> Result<Box<[MessageId]>> {
        let mut url = self.client.get_node().await?;
        let path = "api/v1/messages";
        url.set_path(path);
        url.set_query(Some(&format!("index={}", hex::encode(index))));

        #[derive(Debug, Serialize, Deserialize)]
        struct ResponseWrapper {
            data: MessagesFindResponse,
        }
        let resp: ResponseWrapper = self
            .client
            .http_client
            .get(url.as_str(), self.client.get_timeout(Api::GetMessage))
            .await?
            .json()
            .await?;

        resp.data
            .message_ids
            .iter()
            .map(|s| {
                let mut message_id = [0u8; 32];
                hex::decode_to_slice(s, &mut message_id)?;
                Ok(MessageId::from(message_id))
            })
            .collect::<Result<Box<[MessageId]>>>()
    }

    /// GET /api/v1/messages/{messageID} endpoint
    /// Consume the builder and find a message by its identifer. This method returns the given message object.
    pub async fn data(self, message_id: &MessageId) -> Result<Message> {
        let mut url = self.client.get_node().await?;
        let path = &format!("api/v1/messages/{}", message_id);
        url.set_path(path);
        #[derive(Debug, Serialize, Deserialize)]
        struct ResponseWrapper {
            data: MessageDto,
        }
        let resp: ResponseWrapper = self
            .client
            .http_client
            .get(url.as_str(), self.client.get_timeout(Api::GetMessage))
            .await?
            .json()
            .await?;

        Ok(Message::try_from(&resp.data).map_err(crate::Error::DtoError)?)
    }

    /// GET /api/v1/messages/{messageID}/metadata endpoint
    /// Consume the builder and find a message by its identifer. This method returns the given message metadata.
    pub async fn metadata(self, message_id: &MessageId) -> Result<MessageMetadata> {
        let mut url = self.client.get_node().await?;
        let path = &format!("api/v1/messages/{}/metadata", message_id);
        url.set_path(path);
        #[derive(Debug, Serialize, Deserialize)]
        struct ResponseWrapper {
            data: MessageMetadata,
        }
        let resp: ResponseWrapper = self
            .client
            .http_client
            .get(url.as_str(), self.client.get_timeout(Api::GetMessage))
            .await?
            .json()
            .await?;

        Ok(resp.data)
    }

    /// GET /api/v1/messages/{messageID}/raw endpoint
    /// Consume the builder and find a message by its identifer. This method returns the given message raw data.
    pub async fn raw(self, message_id: &MessageId) -> Result<String> {
        let mut url = self.client.get_node().await?;
        let path = &format!("api/v1/messages/{}/raw", message_id);
        url.set_path(path);
        let resp = self
            .client
            .http_client
            .get(url.as_str(), self.client.get_timeout(Api::GetMessage))
            .await?
            .text()
            .await?;

        Ok(resp)
    }

    /// GET /api/v1/messages/{messageID}/children endpoint
    /// Consume the builder and returns the list of message IDs that reference a message by its identifier.
    pub async fn children(self, message_id: &MessageId) -> Result<Box<[MessageId]>> {
        let mut url = self.client.get_node().await?;
        let path = &format!("api/v1/messages/{}/children", message_id);
        url.set_path(path);
        #[derive(Debug, Serialize, Deserialize)]
        struct ResponseWrapper {
            data: MessageChildrenResponse,
        }
        let resp: ResponseWrapper = self
            .client
            .http_client
            .get(url.as_str(), self.client.get_timeout(Api::GetMessage))
            .await?
            .json()
            .await?;

        resp.data
            .children_message_ids
            .iter()
            .map(|s| {
                let mut message_id = [0u8; 32];
                hex::decode_to_slice(s, &mut message_id)?;
                Ok(MessageId::from(message_id))
            })
            .collect::<Result<Box<[MessageId]>>>()
    }
}
