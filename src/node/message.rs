// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{Api, Client, Result};
use bee_message::{Message, MessageId};
use bee_rest_api::types::{
    body::SuccessBody,
    responses::{
        MessageChildrenResponse, MessageMetadataResponse as MessageMetadata, MessageResponse, MessagesFindResponse,
    },
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
        let path = "api/v1/messages";

        let resp: SuccessBody<MessagesFindResponse> = self
            .client
            .node_manager
            .get_request(
                path,
                Some(&format!("index={}", hex::encode(index))),
                self.client.get_timeout(Api::GetMessage),
            )
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
        let path = &format!("api/v1/messages/{message_id}");

        let resp: SuccessBody<MessageResponse> = self
            .client
            .node_manager
            .get_request(path, None, self.client.get_timeout(Api::GetMessage))
            .await?;

        Ok(Message::try_from(&resp.data.0)?)
    }

    /// GET /api/v1/messages/{messageID}/metadata endpoint
    /// Consume the builder and find a message by its identifer. This method returns the given message metadata.
    pub async fn metadata(self, message_id: &MessageId) -> Result<MessageMetadata> {
        let path = &format!("api/v1/messages/{message_id}/metadata");

        let resp: SuccessBody<MessageMetadata> = self
            .client
            .node_manager
            .get_request(path, None, self.client.get_timeout(Api::GetMessage))
            .await?;

        Ok(resp.data)
    }

    /// GET /api/v1/messages/{messageID}/raw endpoint
    /// Consume the builder and find a message by its identifer. This method returns the given message raw data.
    pub async fn raw(self, message_id: &MessageId) -> Result<String> {
        let path = &format!("api/v1/messages/{message_id}/raw");
        let resp = self
            .client
            .node_manager
            .get_request_text(path, None, self.client.get_timeout(Api::GetMessage))
            .await?;

        Ok(resp)
    }

    /// GET /api/v1/messages/{messageID}/children endpoint
    /// Consume the builder and returns the list of message IDs that reference a message by its identifier.
    pub async fn children(self, message_id: &MessageId) -> Result<Box<[MessageId]>> {
        let path = &format!("api/v1/messages/{message_id}/children");

        let resp: SuccessBody<MessageChildrenResponse> = self
            .client
            .node_manager
            .get_request(path, None, self.client.get_timeout(Api::GetMessage))
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
