// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{get_ureq_agent, Api, Client, Result};
use bee_message::{Message, MessageId};
use bee_rest_api::{
    handlers::{
        message_children::MessageChildrenResponse, message_metadata::MessageMetadataResponse as MessageMetadata,
        messages_find::MessagesForIndexResponse,
    },
    types::MessageDto,
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
        let mut url = self.client.get_node()?;
        let path = "api/v1/messages";
        url.set_path(path);
        url.set_query(Some(&format!("index={}", hex::encode(index))));

        #[derive(Debug, Serialize, Deserialize)]
        struct ResponseWrapper {
            data: MessagesForIndexResponse,
        }
        let resp: ResponseWrapper = get_ureq_agent(self.client.get_timeout(Api::GetMessage))
            .get(&url.to_string())
            .call()?
            .into_json()?;

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
        let mut url = self.client.get_node()?;
        let path = &format!("api/v1/messages/{}", message_id);
        url.set_path(path);
        #[derive(Debug, Serialize, Deserialize)]
        struct ResponseWrapper {
            data: MessageDto,
        }
        let resp: ResponseWrapper = get_ureq_agent(self.client.get_timeout(Api::GetMessage))
            .get(&url.to_string())
            .call()?
            .into_json()?;

        Ok(Message::try_from(&resp.data).expect("Can't convert MessageDto to Message"))
    }

    /// GET /api/v1/messages/{messageID}/metadata endpoint
    /// Consume the builder and find a message by its identifer. This method returns the given message metadata.
    pub async fn metadata(self, message_id: &MessageId) -> Result<MessageMetadata> {
        let mut url = self.client.get_node()?;
        let path = &format!("api/v1/messages/{}/metadata", message_id);
        url.set_path(path);
        #[derive(Debug, Serialize, Deserialize)]
        struct ResponseWrapper {
            data: MessageMetadata,
        }
        let resp: ResponseWrapper = get_ureq_agent(self.client.get_timeout(Api::GetMessage))
            .get(&url.to_string())
            .call()?
            .into_json()?;

        Ok(resp.data)
    }

    /// GET /api/v1/messages/{messageID}/children endpoint
    /// Consume the builder and find a message by its identifer. This method returns the given message raw data.
    pub async fn raw(self, message_id: &MessageId) -> Result<String> {
        let mut url = self.client.get_node()?;
        let path = &format!("api/v1/messages/{}/raw", message_id);
        url.set_path(path);
        let resp = get_ureq_agent(self.client.get_timeout(Api::GetMessage))
            .get(&url.to_string())
            .call()?
            .into_string()?;

        Ok(resp)
    }

    /// Consume the builder and returns the list of message IDs that reference a message by its identifier.
    pub async fn children(self, message_id: &MessageId) -> Result<Box<[MessageId]>> {
        let mut url = self.client.get_node()?;
        let path = &format!("api/v1/messages/{}/children", message_id);
        url.set_path(path);
        #[derive(Debug, Serialize, Deserialize)]
        struct ResponseWrapper {
            data: MessageChildrenResponse,
        }
        let resp: ResponseWrapper = get_ureq_agent(self.client.get_timeout(Api::GetMessage))
            .get(&url.to_string())
            .call()?
            .into_json()?;

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
